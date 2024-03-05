use actix_web::{web};
use crate::controllers::user_controller::{
    cadastro, all_users, perfil, edit_perfil, delete_user
};


pub fn user_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(cadastro)
            .service(all_users)
            .service(perfil)
            .service(edit_perfil)
            .service(delete_user),
    );
}