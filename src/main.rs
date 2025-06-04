use actix_web::{middleware::Logger, web::{self, route}, App, HttpServer};
use dotenvy::dotenv;
use sqlx::MySqlPool;
use std::env;
mod routes;
mod middleware;
// src/main.rs
mod models {
    pub mod users;
    pub mod ticket;
}
mod utils;
mod controllers{
    pub mod userController;
    pub mod ticket_controller;
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = MySqlPool::connect(&db_url).await.expect("Failed to connect to DB");

    HttpServer::new(move ||{
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .configure(routes::init)   
    }).bind(("127.0.0.1",3000))?
      .run()
      .await
}
