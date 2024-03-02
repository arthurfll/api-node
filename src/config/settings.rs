

// dependências ===============================================================

use actix_web::http::header;
use actix_cors::Cors;
use std::env;
use sqlx::mysql::MySqlPool;


// endereço ===================================================================

pub fn address() -> String {

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let ip = env::var("IP").unwrap_or_else(|_| "127.0.0.1".to_string());

    format!("{}:{}", ip, port)
}


// conexão com o banco de dados ===============================================




// cors header ================================================================


pub fn configure_cors() -> Cors {
    let allowed = env::var("CORS_HEADER").expect("CORS_HEADER não definido");
    let cors = Cors::default()
        .allowed_origin(&allowed)
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .max_age(3600);

    cors
}