use dioxus::prelude::*;
use log::info;

mod data;
use data::{ResponseContent};

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).unwrap();
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    //info!("Called App");
    //cx.render(
    //    rsx! {
    //        div { "Hello, World!!" }
    //    }
    //)
    let posts_source = use_future(cx, (), |_| data::call_index());

    cx.render(
        match posts_source.value() {
            Some(Ok(res)) => {
                match &res.result {
                    ResponseContent::Items(items) => {
                        rsx! {
                            for item in items {
                                rsx! { div { "{serde_json::to_string(&item).unwrap()}" } }
                            }
                        }
                    },
                    ResponseContent::Item(item) =>
                        rsx!{ div { "{serde_json::to_string(&item).unwrap()}" } },
                    ResponseContent::Reason(reason) => rsx!{ div { "{reason}" } },
                    ResponseContent::None => rsx!{ div {} },
                }
            },
            Some(Err(err)) => rsx! { div { "初期データの読み込みに失敗しました：{err}" } },
            None => rsx! { div { "データを読み込んでいます..." } }
        }
    )
}
