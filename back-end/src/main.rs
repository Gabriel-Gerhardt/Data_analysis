mod data_base;
pub mod api;
use data_base::db_op;
use dotenv::dotenv;
use std::env;

use api::task::{
    get_task
}; 

use actix_web::{App, web::Data,HttpServer,middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setting environment variables for logging and backtrace
    dotenv().ok();
    env_logger::init();

    // Run the Actix Web server
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)  // Add logging middleware
            .service(get_task)  // Register the `get_task` route
    })
    .bind("127.0.0.1:8080")?  // Bind to localhost on port 8080
    .run() // Run the server
    .await?;

    // db_op();

    Ok(())
}