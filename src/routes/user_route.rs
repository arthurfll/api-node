use actix_web::{web,Scope};

use crate::controllers::user_controller::*;

pub fn user_scope() -> Scope {
    web::scope("/user")
        .route("/cadastro", web::get().to(cadastro))
        .route("/login", web::get().to(login))
        .route("/logout", web::get().to(logout))
        .route("/profile", web::get().to(profile))
}