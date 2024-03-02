use actix_web::{HttpResponse};



pub async fn sucess_url() -> HttpResponse {
    HttpResponse::Ok().body("sucess_url")
}

pub async fn fail_url() -> HttpResponse {
    HttpResponse::Ok().body("fail_url")
}

pub async fn checkout() -> HttpResponse {
    HttpResponse::Ok().body("checkout")
}

pub async fn overview() -> HttpResponse {
    HttpResponse::Ok().body("overview")
}
