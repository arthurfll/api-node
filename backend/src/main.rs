mod config;
mod routes;
mod controllers;
mod models;

use actix_web::{App,HttpServer, web::Data};
use dotenv::dotenv;

use config::{
    settings::get_ip ,
    database::Database
};
use routes::user_route::user_route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let _ = dotenv();
    let addr = get_ip();

    let db = Database::init()
        .await
        .expect("error connecting to database");
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
        .app_data(db_data.clone())
            .configure(user_route)
    })
    .bind(addr)?
    .run()
    .await
}

