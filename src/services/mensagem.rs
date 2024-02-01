use actix_web::{get,Responder,HttpResponse};


#[get("/mensagens")]
pub async fn mensagens() -> impl Responder {
    HttpResponse::Ok().json("mensagens")
}

#[get("/enviar")]
pub async fn enviar() -> impl Responder {
    HttpResponse::Ok().json("enviar")
}
