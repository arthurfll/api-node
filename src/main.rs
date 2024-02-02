use actix_web::{ App , HttpServer , web };
use std::env;
use dotenv::dotenv;
use sqlx::MySqlPool;
use std::sync::Arc;

use services::{
    usuario::{
        login,logout,cadastro
    },
    mensagem::{
        mensagens,enviar
    },
    conversa::{
        conversas,delete,criar
    }
};



#[actix_web::main]
async fn main()-> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT")
        .unwrap_or_else(|_| String::from("8080"));

    let db_url = "mysql://root:@localhost:3306/arthurfll";
    let pool = MySqlPool::connect(db_url).await.unwrap();
    println!("conectado ao banco de dados");

    HttpServer::new(move|| {
        App::new()
            .app_data(web::Data::new(Arc::new(pool.clone())))
            .service(login)
            .service(logout)
            .service(cadastro)
            .service(mensagens)
            .service(enviar)
            .service(conversas)
            .service(criar)
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