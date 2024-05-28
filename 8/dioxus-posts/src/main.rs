#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

mod data;
use data::{ResponseContent};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    /*rsx! {
        link { rel: "stylesheet", href: "main.css" }
        img { src: "header.svg", id: "header" }
        div { id: "links",
            a { target: "_blank", href: "https://dioxuslabs.com/learn/0.5/", "📚 Learn Dioxus" }
            a { target: "_blank", href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
            a { target: "_blank", href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
            a { target: "_blank", href: "https://github.com/DioxusLabs/dioxus-std", "⚙️ Dioxus Standard Library" }
            a { target: "_blank", href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
            a { target: "_blank", href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
        }
    }*/
    let posts_source = use_resource(|| data::call_index());

    rsx! {
        match &*posts_source.read_unchecked() {
            Some(Ok(res)) => {
                match &res.result {
                    ResponseContent::Items(items) => {
                        rsx! {
                            for item in items {
                                {rsx! { div { "{serde_json::to_string(&item).unwrap()}" } }}
                            }
                        }
                    },
                    ResponseContent::Item(item) =>
                        rsx! { div { "{serde_json::to_string(&item).unwrap()}" } },
                    ResponseContent::Reason(reason) => rsx! { div { "{reason}" } },
                    ResponseContent::None => rsx! { div {} },
                }
            },
            Some(Err(err)) => rsx! { div { "初期データの読み込みに失敗しました：{err}" } },
            None => rsx! { div { "データを読み込んでいます..." } }
        }
    }
}
