mod config;
mod routes;
mod controllers;
mod models;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

use config::settings::{
    address , configure_cors
};
use routes::{
    user_route::user_scope,
    licence_route::licence_scope
};



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let address = address();

    HttpServer::new(move || {
        let cors = configure_cors();
        App::new()
            .wrap(cors)
            .service(user_scope())
            .service(licence_scope())
    })
    .bind(address)?
    .run()
    .await
}