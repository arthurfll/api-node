use actix_web::{HttpResponse};



pub async fn cadastro() -> HttpResponse {
    HttpResponse::Ok().body("cadastro")
}

pub async fn login() -> HttpResponse {
    HttpResponse::Ok().body("login")
}

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().body("logout")
}

pub async fn profile() -> HttpResponse {
    HttpResponse::Ok().body("profile")
}