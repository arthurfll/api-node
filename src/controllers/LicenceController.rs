use actix_web::{HttpResponse};


pub fn home_controller()-> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}