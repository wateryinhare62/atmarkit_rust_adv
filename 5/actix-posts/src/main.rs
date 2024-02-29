use std::io::Result;
use actix_web::{App, HttpServer, Responder, HttpResponse, get, web,
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
        let mut tera = Tera::new("templates/**/*.html").unwrap();
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
        .wrap(message_framework.clone())
        .wrap(build_cookie_session_middleware(key.clone()))
    })
    .bind("127.0.0.1:8000")?.run().await
}
