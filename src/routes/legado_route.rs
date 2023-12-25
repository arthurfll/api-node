use actix_web::{
    Responder,get,
    web::Redirect,
    HttpResponse
};


#[get("/sucess")]
pub async fn sucess_route() -> impl Responder {
    // lógica
    Redirect::to("http://localhost:8000/")
}



#[get("/fail")]
pub async fn fail_route() -> impl Responder {
    // lógica
    Redirect::to("https://buy.stripe.com/eVa6pc4Pq6D0eA0146")
}



#[get("/")]
pub async fn checkout_route()-> impl Responder {
    // lógica
    HttpResponse::Ok().body("checkout")
}
