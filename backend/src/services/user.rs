use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[post("/login")]
pub async fn login() -> impl Responder {
    HttpResponse::Ok().body("login")
}

#[post("/cadastro")]
pub async fn cadastro() -> impl Responder {
    HttpResponse::Ok().body("cadastro")
}