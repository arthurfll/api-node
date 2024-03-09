use actix_web::{
    get, post, delete, patch,
    HttpResponse, Responder,web::{Data,Json,Path}
};
use crate::models::user_model::{UserCadastro,EditPerfilURL, User,RequestUser};
use crate::config::database::Database;
use uuid::Uuid;


#[get("/all_users")]
pub async fn all_users(db: Data<Database>) -> impl Responder {
    let users = db.get_all_users().await;
    HttpResponse::Ok().json(users)
}


#[post("/cadastro")]
async fn cadastro(body: Json<UserCadastro>, db: Data<Database>) -> impl Responder {

    let username = body.username.clone();
    let email = body.email.clone();
    let first_name = body.first_name.clone();
    let last_name = body.last_name.clone();
    let password = body.password.clone();

    let mut buffer = Uuid::encode_buffer();
    let new_uuid = Uuid::new_v4()
        .simple()
        .encode_lower(&mut buffer);

    let _new_user = db.db_cadastro(User::new(
        String::from(new_uuid),
        username,email,first_name,last_name,password
    )).await;

    HttpResponse::Ok().body("200")
}


#[get("/perfil/{username}")]
async fn perfil() -> impl Responder {
    HttpResponse::Ok().body("Profile endpoint")
}


#[patch("/edit_user/{uuid}")]
async fn edit_perfil(edit_perfil_url: Path<EditPerfilURL>,db: Data<Database>) -> impl Responder {
    let uuid = edit_perfil_url.into_inner().uuid;
    let update_result = db.update_user(uuid).await;
    HttpResponse::Ok().json(update_result)
}


#[delete("/delete_user/{username}")]
async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("Delete user endpoint")
}

