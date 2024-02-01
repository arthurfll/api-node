use actix_web::{ App , HttpServer };
use std::env;
use dotenv::dotenv;

use services::{
    usuario::{
        login,logout,cadastro
    },
    mensagem::{
        mensagens,enviar
    },
    conversa::{
        conversas,delete
    }
};



#[actix_web::main]
async fn main()-> std::io::Result<()> {

    dotenv().ok();

    let port = env::var("PORT")
        .unwrap_or_else(|_| String::from("8080"));

    HttpServer::new(|| {
        App::new()
            .service(login)
            .service(logout)
            .service(cadastro)
            .service(mensagens)
            .service(enviar)
            .service(conversas)
            .service(delete)
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}



mod database;
mod middlewares;
mod models;
mod services;