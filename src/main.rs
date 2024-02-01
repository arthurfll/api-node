use actix_web::{ App , HttpServer };
use std::env;
use dotenv::dotenv;


#[actix_web::main]
async fn main()-> std::io::Result<()> {

    dotenv().ok();

    let port = env::var("PORT")
        .unwrap_or_else(|_| String::from("8080"));

    HttpServer::new(|| {
        App::new()
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}

mod database;
mod middlewares;
mod models;
mod services;