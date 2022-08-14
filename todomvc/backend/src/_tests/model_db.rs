use super::init_db;

#[tokio::test]
async fn model_db_init_db() -> Result<(), Box<dyn std::error::Error>> {
	// ACTION
	let db = init_db().await?;

	// CHECK (init_db 会插入两条数据(02-dev-seed.sql))
	let result = sqlx::query("SELECT * from todo").fetch_all(&db).await?;
	assert_eq!(2, result.len(), "number of seed todos");

	Ok(())
}
