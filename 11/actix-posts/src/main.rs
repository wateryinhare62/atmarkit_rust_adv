use std::io::Result;
use actix_web::{App, HttpServer, web,
    middleware::Logger};
use env_logger::Env;
// テンプレート
use tera::Tera;
// セッション
use actix_web::cookie::{Key};
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
// フラッシュメッセージ
use actix_web_flash_messages::FlashMessagesFramework;
use actix_web_flash_messages::storage::SessionMessageStore;
use actix_web_flash_messages::storage::CookieMessageStore;
// CORS
use actix_web::http::header;
use actix_cors::Cors;

mod handler;

// クッキーベースのセッションを使うための関数
fn build_cookie_session_middleware(key: Key) 
        -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), key).build()
}

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let key = Key::generate();
    // メッセージストアにクッキーを使う場合
    //let message_store = CookieMessageStore::builder(key).build();
    // メッセージストアにクッキーベースのセッションを使う場合
    let message_store = SessionMessageStore::default();
    // 以降共通
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    HttpServer::new(move || {
        let tera = Tera::new("templates/**/*.html").unwrap();
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                true
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);
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
        .service(
            web::scope("/api")
                .service(handler::api_index)
                .service(handler::api_create)
                .service(handler::api_show)
                .service(handler::api_update)
                .service(handler::api_destroy)
                .default_service(web::to(handler::api_not_found))
        )
                .wrap(Logger::default())
        .wrap(message_framework.clone())
        .wrap(build_cookie_session_middleware(key.clone()))
        .wrap(cors)
    })
    .bind("127.0.0.1:8000")?.run().await
}
