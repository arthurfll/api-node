use actix_web::{
    App,HttpServer,
    get,post,put,delete,
    Responder,HttpResponse
};

#[get("/get")]
async fn get_all() -> impl Responder {
    HttpResponse::Ok().body("CRUD Read")
}

#[post("/post")]
async fn create() -> impl Responder {
    HttpResponse::Ok().body("CRUD Create")
}

#[put("/put")]
async fn update() -> impl Responder {
    HttpResponse::Ok().body("CRUD Update")
}

#[delete("/delete")]
async fn delete_by_id() -> impl Responder {
    HttpResponse::Ok().body("CRUD Delete")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .service(get_all)
            .service(create)
            .service(update)
            .service(delete_by_id)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}