use actix_web::{web, Scope};

use crate::controllers::licence_controller::*;

pub fn licence_scope() -> Scope {
    web::scope("/licence")
        .route("/sucess_url", web::get().to(sucess_url))
        .route("/fail_url", web::get().to(fail_url))
        .route("/checkout", web::get().to(checkout))
        .route("/overview", web::get().to(overview))
}