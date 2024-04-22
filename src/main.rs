use actix_web::{App,HttpServer};
use routes::UserRoute::UserRoute;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(UserRoute)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

mod routes;
mod controllers;
mod config;
mod database;
mod middlewares;
mod models;
