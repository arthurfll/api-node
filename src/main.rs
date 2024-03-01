use actix_web::{App, HttpServer};
use dotenv::dotenv;

use config::settings::{address,conn_db};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let address = address();
    let pool = conn_db().await;

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
    })
    .bind(address)?
    .run()
    .await
}



mod config;
mod routes;