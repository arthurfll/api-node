pub mod database;

use actix_web::{App,HttpServer};

use database::conn::establish_connection;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::load().ok();
    let client = establish_connection().await.unwrap();
    println!("Connected to MongoDB!");

    HttpServer::new(|| {
        App::new()
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}