#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::{Level, info};
//use chrono::{Local};

mod data;
use data::{Message, ResponseContent};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

type Posts = im_rc::HashMap<i32, Message>;

#[derive(PartialEq)]
enum Mode {
    None,
    New,
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
    let mut posts_source = use_resource(|| data::call_index());
    let mut posts = use_signal(Posts::default);
    let mut sender = use_signal(String::new);
    let mut content = use_signal(String::new);
    let mut mode = use_signal(|| Mode::None);

    rsx! {
        match &*posts_source.read_unchecked() {
            Some(Ok(res)) => {
                if posts.read().is_empty() {
                    match &res.result {
                        ResponseContent::Items(items) => {
                            for item in items {
                                posts.write().insert(item.id, 
                                    Message {
                                        id: item.id,
                                        posted: item.posted.clone(),
                                        sender: item.sender.clone(),
                                        content: item.content.clone(),
                                    }
                                );
                            }
                            rsx! { div { "データ読み込みを終了しました" } }
                        },
                        ResponseContent::Item(item) =>
                            rsx! { div { "{serde_json::to_string(&item).unwrap()}" } },
                        ResponseContent::Reason(reason) => rsx! { div { "{reason}" } },
                        ResponseContent::None => rsx! { div {} },
                    }
                } else {
                    //rsx! { div { "ここに投稿データを表示します" } }
                    let mut filtered_posts = posts.read()
                        .iter()
                        .map(|f| *f.0)
                        .collect::<Vec<_>>();
                    filtered_posts.sort_unstable_by(|a, b| b.cmp(a));
                    rsx! {
                        div {
                            p {
                                class: "text-end mb-2",
                                button {
                                    class: "btn btn-primary me-2",
                                    onclick: move |_| {
                                        *mode.write() = Mode::New;
                                    },
                                    "新規投稿",
                                }
                                button {
                                    class: "btn btn-primary",
                                    onclick: move |_| {
                                        posts.write().clear();
                                        posts_source.restart();
                                        *mode.write() = Mode::None;
                                    },
                                    "再読み込み",
                                }
                            }
                        }
                        if *mode.read() == Mode::New {
                            div {
                                class: "mb-4",
                                input {
                                    class: "d-block mb-2",
                                    placeholder: "お名前をどうぞ",
                                    value: "{sender.read()}",
                                    oninput: move |e| sender.set(e.value().clone()),
                                    autofocus: "true",
                                }
                                textarea {
                                    class: "d-block w-100 mb-2",
                                    placeholder: "メッセージをどうぞ",
                                    value: "{content.read()}",
                                    oninput: move |e| content.set(e.value().clone()),
                                }
                                button {
                                    r#type: "button",
                                    class: "btn btn-primary me-2",
                                    onclick: move |_| {
                                        if !sender.read().is_empty() && !content.read().is_empty() {
                                            spawn( async move {
                                                let message = Message {
                                                    id: 0,
                                                    posted: "".to_string(),
                                                    sender: sender.read().clone(),
                                                    content: content.read().clone(),
                                                };
                                                let res = data::call_create(&message).await.unwrap();
                                                match &res.result {
                                                    ResponseContent::Item(item) => {
                                                        posts.write().insert(
                                                            item.id,
                                                            item.clone()
                                                        );
                                                    },
                                                    _ => {}
                                                };
                                                content.set("".to_string());
                                            });
                                            *mode.write() = Mode::None;
                                        }
                                    },
                                    "投稿",
                                }
                                button {
                                    r#type: "button",
                                    class: "btn btn-outline-dark",
                                    onclick: move |_| {
                                        *mode.write() = Mode::None;
                                    },
                                    "キャンセル",
                                }
                            }
                        }
                        {
                            filtered_posts.iter().map(|id| {
                                //info!("map: {id}");
                                rsx!(PostEntry { 
                                    id: *id, 
                                    set_posts: posts
                                })
                            })
                        }
                    }
                }
            },
            Some(Err(err)) => rsx! { div { "初期データの読み込みに失敗しました：{err}" } },
            None => rsx! { div { "データを読み込んでいます..." } }
        }
    }

}

//#[derive(Props, Clone, PartialEq)]
//struct PostEntryProps {
//    set_posts: Signal<Posts>,
//    set_flags: Signal<Flags>,
//    id: i32,
//}

#[component]
//fn PostEntry(props: PostEntryProps) -> Element {
fn PostEntry(id: i32, set_posts: Signal<Posts>) -> Element {
    //let posts = props.set_posts.read();
    let posts = set_posts.read();
    //let post = &posts[&props.id];
    let post = &posts[&id];

    rsx! {
        div {
            class: "card mb-3",
            div {
                div {
                    class: "card-header",
                    "{post.sender} {post.posted}",
                }
                div {
                    class: "card-body",
                    p {
                        class: "card-text",
                        dangerous_inner_html: post.content.replace("\n", "<br />")
                    }
                }
            }
        }
    }
}
