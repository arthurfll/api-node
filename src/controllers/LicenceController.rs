use actix_web::{web::Redirect, Responder};
use std::env;

pub fn checkout_controller() -> impl Responder {
   let link: String = env::var("LINK_LEGADO_LICENCE").expect("LINK_LEGADO_LICENCE must be set");

   Redirect::to(link)
}