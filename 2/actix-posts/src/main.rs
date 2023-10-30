use std::io::Result;
use actix_web::{App, HttpServer, Responder, HttpResponse, get, 
    middleware::Logger};
use env_logger::Env;

mod handler;

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
        .service(handler::index)
        .wrap(Logger::default())
    })
    .bind("127.0.0.1:8000")?.run().await
}
