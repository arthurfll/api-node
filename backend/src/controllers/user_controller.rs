use actix_web::{
    get, post, delete, patch,
    HttpResponse, Responder,web::{Data,Json,Path}
};
use crate::models::user_model::{UserCadastro,EditPerfilURL, User,RequestUser};
use crate::config::database::Database;


#[get("/all_users")]
pub async fn all_users(db: Data<Database>) -> impl Responder {
    let users = db.get_all_users().await;
    HttpResponse::Ok().body(format!("{:?}",users))
}


#[post("/cadastro")]
async fn cadastro(body: Json<UserCadastro>) -> impl Responder {

//    let username = body.username.clone();
//    let email = body.email.clone();
//    let first_name = body.first_name.clone();
//    let last_name = body.last_name.clone();
//    let password = body.password.clone();

    HttpResponse::Ok().body("selva")
}


#[get("/perfil/{uuid}")]
async fn perfil() -> impl Responder {
    HttpResponse::Ok().body("Profile endpoint")
}


#[patch("/edit_user/{uuid}")]
async fn edit_perfil(edit_perfil_url: Path<EditPerfilURL>) -> impl Responder {
    let uuid = edit_perfil_url.into_inner().uuid;
    HttpResponse::Ok().body("Edit user endpoint")
}


#[delete("/delete_user/{uuid}")]
async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("Delete user endpoint")
}

