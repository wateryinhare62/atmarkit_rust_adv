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
// sqlx
use dotenv;
use sqlx::sqlite::SqlitePool;
use std::env;

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
    // sqlx
    dotenv::dotenv().expect(".envの読み込み失敗");
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URLがセットされていません");
    let pool = SqlitePool::connect(&database_url).await.unwrap();
    HttpServer::new(move || {
        let tera = Tera::new("templates/**/*.html").unwrap();
        App::new()
        .app_data(web::Data::new(tera))
        .app_data(web::Data::new(pool.clone()))
        .service(handler::index)
        .service(handler::new)
        .service(handler::create)
        .service(handler::show)
        .service(handler::edit)
        .service(handler::update)
        .service(handler::destroy)
        .default_service(web::to(handler::not_found))
        .service(
            web::scope("/api/db")
                .service(handler::api_index_withdb)
                .service(handler::api_show_withdb)
                .service(handler::api_create_withdb)
                .service(handler::api_update_withdb)
                .service(handler::api_destroy_withdb)
                .default_service(web::to(handler::api_not_found))
        )
        .service(
            web::scope("/api")
                .service(handler::api_index)
                .service(handler::api_show)
                .service(handler::api_create)
                .service(handler::api_update)
                .service(handler::api_destroy)
                .default_service(web::to(handler::api_not_found))
        )
        .wrap(Logger::default())
        .wrap(message_framework.clone())
        .wrap(build_cookie_session_middleware(key.clone()))
    })
    .bind("127.0.0.1:8000")?.run().await
}
