use actix_web::{get,Responder};
use crate::controllers::LicenceController;


#[get("/checkout")]
pub async fn checkout() -> impl Responder {
    LicenceController::checkout_controller()
}