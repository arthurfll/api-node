use actix_web::{get,HttpResponse,Responder};

#[get("/all_users/")]
pub async fn all_users() -> impl Responder {
    HttpResponse::Ok().body("selva")
}