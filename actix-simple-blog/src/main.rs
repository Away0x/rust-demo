use std::net::TcpListener;

use actix_simple_blog::start;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// std::env::set_var("RUST_LOG", "actix_web=info");
	// env_logger::init();
	
	let listener = TcpListener::bind("0.0.0.0:8080")?;
	start(listener)?.await?;
	Ok(())
}