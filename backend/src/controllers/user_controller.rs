use actix_web::{
    get, post, delete, patch,
    HttpResponse, Responder,web::Json
};
use crate::models::user_model::UserCadastro;



#[get("/all_users")]
async fn all_users() -> impl Responder {
    HttpResponse::Ok().body("All users endpoint")
}


#[post("/cadastro")]
async fn cadastro(body: Json<UserCadastro>) -> impl Responder {

    let username = body.username.clone();
    let email = body.email.clone();
    let first_name = body.first_name.clone();
    let last_name = body.last_name.clone();
    let senha = body.senha.clone()

    HttpResponse::Ok().body("selva")
}


#[get("/perfil/{uuid}")]
async fn perfil() -> impl Responder {
    HttpResponse::Ok().body("Profile endpoint")
}


#[patch("/edit_user/{uuid}")]
async fn edit_perfil() -> impl Responder {
    HttpResponse::Ok().body("Edit user endpoint")
}


#[delete("/delete_user/{uuid}")]
async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("Delete user endpoint")
}

