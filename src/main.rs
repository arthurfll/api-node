mod database;
mod controllers;
mod middlewares;
mod models;
mod routes;

use actix_web::{ App , HttpServer };
use std::env;

use routes::LicenceRoute::home;

#[actix_web::main]
async fn main()-> std::io::Result<()> {

    dotenv::dotenv().ok();

    let port = env::var("PORT")
        .unwrap_or_else(|_| String::from("8080"));

    HttpServer::new(|| {
        App::new()
            .service(home)
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
