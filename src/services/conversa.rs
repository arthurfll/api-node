use actix_web::{get,Responder,HttpResponse};


#[get("/criar")]
pub async fn criar(criar: web::Json<Criar>, pool: Data<Arc<MySqlPool>>) -> impl Responder {
    let query = format!(
        r#"
            INSERT INTO conversa (username, first_name, last_name, email, password, saldo)
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
    return HttpResponse::Ok().json(criar)
}

#[get("/conversas")]
pub async fn conversas() -> impl Responder {
    HttpResponse::Ok().json("conversas")
}

#[get("/delete")]
pub async fn delete() -> impl Responder {
    HttpResponse::Ok().json("delete")
}
