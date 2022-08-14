use crate::{
	model::{
		self,
		db::init_db,
		todo::{TodoMac, TodoPatch, TodoStatus},
	},
	security::utx_from_token,
};

use super::Todo;

#[tokio::test]
async fn model_todo_create() -> Result<(), Box<dyn std::error::Error>> {
	// -- FIXTURE
	let db = init_db().await?;
	let utx = utx_from_token(&db, "123").await?;
	let data_fx = TodoPatch {
		title: Some("test - model_todo_create 1".to_string()),
		..Default::default()
	};

	println!("{:?}", data_fx);

	// -- ACTION
	let todo_created = TodoMac::create(&db, &utx, data_fx.clone()).await?;

	// -- CHECK
	assert!(todo_created.id >= 1000, "Id should be >= 1000");
	assert_eq!(data_fx.title.unwrap(), todo_created.title);
	assert_eq!(TodoStatus::Open, todo_created.status);

	Ok(())
}

#[tokio::test]
async fn model_todo_list() -> Result<(), Box<dyn std::error::Error>> {
	// -- FIXTURE
	let db = init_db().await?;
	let utx = utx_from_token(&db, "123").await?;

	// -- ACTION
	let todos = TodoMac::list(&db, &utx).await?;

	// -- CHECK
	assert_eq!(2, todos.len());
	println!("\n\n->> {:?}", todos);
	// todo 101
	assert_eq!(101, todos[0].id);
	assert_eq!(123, todos[0].cid);
	assert_eq!("todo 101", todos[0].title);
	// todo 100
	assert_eq!(100, todos[1].id);
	assert_eq!(123, todos[1].cid);
	assert_eq!("todo 100", todos[1].title);

	Ok(())
}

#[tokio::test]
async fn model_todo_get_ok() -> Result<(), Box<dyn std::error::Error>> {
	// -- FIXTURE
	let db = init_db().await?;
	let utx = utx_from_token(&db, "123").await?;

	// -- ACTION
	let todo = TodoMac::get(&db, &utx, 100).await?;

	// -- CHECK
	assert_eq!(100, todo.id);
	assert_eq!("todo 100", todo.title);
	assert_eq!(TodoStatus::Close, todo.status);

	Ok(())
}

#[tokio::test]
async fn model_todo_get_wong_id() -> Result<(), Box<dyn std::error::Error>> {
	// -- FIXTURE
	let db = init_db().await?;
	let utx = utx_from_token(&db, "123").await?;

	// -- ACTION
	let result = TodoMac::get(&db, &utx, 999).await;

	// -- CHECK
	match result {
		Ok(_) => assert!(false, "Should not succeed"),
		Err(model::Error::EntityNotFound(typ, id)) => {
			assert_eq!("todo", typ);
			assert_eq!(999.to_string(), id);
		}
		other_error => assert!(false, "Wrong Error {:?} ", other_error),
	}

	Ok(())
}

#[tokio::test]
async fn model_todo_update_ok() -> Result<(), Box<dyn std::error::Error>> {
	// -- FIXTURE
	let db = init_db().await?;
	let utx = utx_from_token(&db, "123").await?;
	let data_fx = TodoPatch {
		title: Some("test - model_todo_update_ok 1".to_string()),
		..Default::default()
	};
	let todo_fx = TodoMac::create(&db, &utx, data_fx.clone()).await?;
	let update_data_fx = TodoPatch {
		title: Some("test - model_todo_update_ok 2".to_string()),
		..Default::default()
	};

	// -- ACTION
	let todo_updated = TodoMac::update(&db, &utx, todo_fx.id, update_data_fx.clone()).await?;

	// -- CHECK
	let todos = TodoMac::list(&db, &utx).await?;
	assert_eq!(3, todos.len());
	assert_eq!(todo_fx.id, todo_updated.id);
	assert_eq!(update_data_fx.title.unwrap(), todo_updated.title);

	Ok(())
}

#[tokio::test]
async fn model_todo_delete_simple() -> Result<(), Box<dyn std::error::Error>> {
	// -- FIXTURE
	let db = init_db().await?;
	let utx = utx_from_token(&db, "123").await?;

	// -- ACTION
	let todo = TodoMac::delete(&db, &utx, 100).await?;

	// -- CHECK - deleted item
	assert_eq!(100, todo.id);
	assert_eq!("todo 100", todo.title);

	// -- CHECK - list
	let todos: Vec<Todo> = sqlb::select().table("todo").fetch_all(&db).await?;
	assert_eq!(1, todos.len());

	Ok(())
}
