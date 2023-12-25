use actix_web::{
    HttpServer,App
};

use std::env;
use dotenv::dotenv;

mod routes;

use routes::legado_route::{
    checkout_route,
    sucess_route,
    fail_route
};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok().expect("erro ao carregar as variáveis de ambiente");

    let port = env::var("PORT").expect("erro");

    HttpServer::new(|| {
        App::new()
            .service(checkout_route)
            .service(sucess_route)
            .service(fail_route)
    })
    .bind(format!("127.0.0.1:{}",port))?
    .run()
    .await
}

