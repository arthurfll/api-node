use actix_web::{get,Responder};
use crate::controllers::LicenceController::home_controller;


#[get("/")]
pub async fn home() -> impl Responder {
    home_controller()
}
