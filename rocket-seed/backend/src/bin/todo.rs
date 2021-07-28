use backend::db::{create_task, delete_tasks, establish_connection, query_task, update_tasks};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let subcommand = &args[1];
    match subcommand.as_ref() {
        "new" => run_new_task(&args[2..]),
        "show" => run_show_tasks(&args[2..]),
        "done" => run_done_tasks(&args[2..]),
        "delete" => run_delete_tasks(&args[2..]),
        _ => help(),
    }
}

fn help() {
    println!("subcommands:");
    println!("    new<title>: create a new task");
    println!("    show<title?>: show tasks");
    println!("    done<title>: done tasks");
    println!("    delete<title>: delete tasks");
}

fn run_new_task(args: &[String]) {
    if args.len() < 1 {
        println!("new: missing <title>");
        help();
        return;
    }

    let conn = establish_connection();
    create_task(&conn, &args[0]);
}

fn run_show_tasks(args: &[String]) {
    if args.len() > 1 {
        println!("show: unexpected argument");
        help();
        return;
    }

    let conn = establish_connection();
    println!("TASKS\n-----");

    let results = query_task(
        &conn,
        if let Some(title) = args.get(0) {
            Some(title)
        } else {
            None
        },
    );

    print_tasks(results);
}

fn run_done_tasks(args: &[String]) {
    if args.len() < 1 {
        println!("done: missing <title>");
        help();
        return;
    }

    let conn = establish_connection();
    let results = update_tasks(&conn, &args[0]);

    print_tasks(results);
}

fn run_delete_tasks(args: &[String]) {
    if args.len() < 1 {
        println!("done: missing <title>");
        help();
        return;
    }

    let conn = establish_connection();
    delete_tasks(&conn, &args[0]);
}

fn print_tasks(results: Vec<backend::db::models::Task>) {
    for task in results {
        println!(
            "({}) {}: {}",
            task.id,
            task.title,
            if task.status == 0 { "Not done" } else { "Done" }
        );
    }
}
