use actix_web::{HttpResponse,Responder,get,post,web};
use crate::models::usuario::Cadastro;
use sqlx;
use std::sync::Arc;
use sqlx::MySqlPool;
use actix_web::web::Data;



#[post("cadastro/")]
pub async fn cadastro(cadastro: web::Json<Cadastro>, pool: Data<Arc<MySqlPool>>) -> HttpResponse {
    let query = format!(
        r#"
            INSERT INTO usuario (username, first_name, last_name, email, password, saldo)
            VALUES ('{}', '{}', '{}', '{}', '{}', {})
        "#,
        cadastro.username,
        cadastro.first_name,
        cadastro.last_name,
        cadastro.email,
        cadastro.password,
        cadastro.saldo
    );

    let _ = sqlx::query(&query)
        .execute(&***pool)
        .await;
    println!("200");
    return HttpResponse::Ok().json(cadastro)
}


#[get("/login")]
pub async fn login() -> impl Responder {
    HttpResponse::Ok().json("login")
}

#[get("/logout")]
pub async fn logout() -> impl Responder {
    HttpResponse::Ok().json("logout")
}
