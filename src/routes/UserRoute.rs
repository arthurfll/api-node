use actix_web::web;
use crate::controllers::UserController::{all_users};

pub fn UserRoute(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user/")
            .service(all_users)
    );
}
