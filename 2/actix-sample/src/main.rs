use std::io::Result;
use actix_web::{App, HttpServer, Responder, HttpResponse, get};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}

#[actix_rt::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("127.0.0.1:8000")?.run().await
}
