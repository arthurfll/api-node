use actix_web::{get,Responder,HttpResponse};


#[get("/conversas")]
pub async fn conversas() -> impl Responder {
    HttpResponse::Ok().json("conversas")
}

#[get("/delete")]
pub async fn delete() -> impl Responder {
    HttpResponse::Ok().json("delete")
}
