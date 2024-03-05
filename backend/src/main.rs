mod config;
mod routes;
mod controllers;
mod models;

use actix_web::{App,HttpServer};
use dotenv::dotenv;

use config::settings::get_ip;
use routes::user_route::user_route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let _ = dotenv();
    let addr = get_ip();

    HttpServer::new(|| {
        App::new()
            .configure(user_route)
    })
    .bind(addr)?
    .run()
    .await
}

