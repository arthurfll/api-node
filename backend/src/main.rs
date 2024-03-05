use actix_web::{App,HttpServer};
use dotenv::dotenv;

use config::settings::get_ip;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenv();
    let addr = get_ip();
    HttpServer::new(|| {
        App::new()
    })
    .bind(addr)?
    .run()
    .await
}

pub mod config;
