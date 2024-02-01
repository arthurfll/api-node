use actix_web::{get,Responder,HttpResponse};


#[get("/cadastro")]
pub async fn cadastro() -> impl Responder {
    HttpResponse::Ok().json("cadastro")
}

#[get("/login")]
pub async fn login() -> impl Responder {
    HttpResponse::Ok().json("login")
}

#[get("/logout")]
pub async fn logout() -> impl Responder {
    HttpResponse::Ok().json("logout")
}
