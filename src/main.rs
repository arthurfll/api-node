mod config;
mod routes;
mod controllers;
mod models;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

use config::settings::{
    address , conn_db , configure_cors
};
use routes::{
    user_route::user_scope,
    licence_route::licence_scope
};



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let address = address();
    let pool = conn_db().await;

    HttpServer::new(move || {
        let cors = configure_cors();
        App::new()
            .wrap(cors)
            .data(pool.clone())
            .service(user_scope())
            .service(licence_scope())
    })
    .bind(address)?
    .run()
    .await
}