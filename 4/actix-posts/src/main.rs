use std::io::Result;
use actix_web::{App, HttpServer, Responder, HttpResponse, get, web,
    middleware::Logger};
use env_logger::Env;
use tera::Tera;

mod handler;

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        let mut tera = Tera::new("templates/**/*.html").unwrap();
        //tera.autoescape_on(vec![]);
        App::new()
        .app_data(web::Data::new(tera))
        .service(handler::index)
        .service(handler::new)
        .service(handler::create)
        .service(handler::show)
        .service(handler::edit)
        .service(handler::update)
        .service(handler::destroy)
        .default_service(web::to(handler::not_found))
        .wrap(Logger::default())
    })
    .bind("127.0.0.1:8000")?.run().await
}
