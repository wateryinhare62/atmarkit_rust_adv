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
    Menu,
    Edit,
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
    //let mut posts = use_signal(Posts::default);   // コンテキスト化により無効
    let mut posts = use_context_provider(|| Signal::new(Posts::default()));
    //let mut mode = use_signal(|| Mode::None);     // コンテキスト化により無効
    let mut mode = use_context_provider(|| Signal::new(Mode::None));
    let _target_id = use_context_provider(|| Signal::new(0));
    let mut sender = use_signal(String::new);
    let mut content = use_signal(String::new);

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
                                    //set_posts: posts, // コンテキスト化により無効 
                                    //mode: mode,       // コンテキスト化により無効
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
//    id: i32,
//    set_posts: Signal<Posts>,
//    mode: Signal<Mode>, 
//    target_id: Signal<i32>,
//}

#[component]
//fn PostEntry(props: PostEntryProps) -> Element { 
fn PostEntry(id: i32) -> Element {
    //let posts = props.set_posts.read();   // コンテキスト化により無効   
    //let posts = set_posts.read();         // コンテキスト化により無効
    let set_posts = use_context::<Signal<Posts>>();
    let mut mode = use_context::<Signal<Mode>>();
    let mut target_id = use_context::<Signal<i32>>();
    let posted = use_signal(String::new);
    let sender = use_signal(String::new);
    let content = use_signal(String::new);

    //let post = &posts[&props.id]; // コンテキスト化により無効
    let post = &set_posts.read()[&id];
    //info!("render {id}: {target_id}: {0}", post.sender);
    if *mode.read() != Mode::Edit || id != *target_id.read() {
        rsx! {
            div {
                onclick: move |_| {
                    //info!("onclick item {id}");
                    *mode.write() = Mode::Menu;
                    *target_id.write() = id;
                },
                class: "card mb-3",
                div {
                    div {
                        class: "card-header",
                        "{post.sender} {post.posted}",
                        EditMenu {
                            id: id,
                            posted: posted,
                            sender: sender,
                            content: content,
                        }
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
    } else {
        rsx! {
            PostEdit {
                id: id,
                posted: posted,
                sender: sender,
                content: content,
            }
        }
    }
}

#[component]
fn EditMenu(id: i32, posted: Signal<String>, sender: Signal<String>, content: Signal<String>) -> Element {
    let mut posts = use_context::<Signal<Posts>>();
    let mut mode = use_context::<Signal<Mode>>();
    let mut target_id = use_context::<Signal<i32>>();

    rsx!{
        if *mode.read() == Mode::Menu && id == *target_id.read() {
            button {
                class: "btn btn-primary btn-sm mx-2",
                onclick: move |e| {
                    //info!("onclick edit {id}");
                    *mode.write() = Mode::Edit;
                    *posted.write() = posts.read()[&id].posted.clone();
                    *sender.write() = posts.read()[&id].sender.clone();
                    *content.write() = posts.read()[&id].content.clone();
                    e.stop_propagation();
                },
                "編集"
            }
            button {
                class: "btn btn-danger btn-sm me-2",
                onclick: move |e| {
                    //info!("onclick delete {id}");
                    spawn( async move {
                        let res = data::call_delete(id).await.unwrap();
                        match (&res.status).as_str() {
                            "OK" => {
                                posts.write().remove(&id);
                                *mode.write() = Mode::None;
                                *target_id.write() = 0;
                            },
                            _ => {}
                        };
                    });
                    e.stop_propagation();
                },
                "削除"
            }
            button {
                class: "btn btn-outline-dark btn-sm",
                onclick: move |e| {
                    //info!("onclick cancel {id}");
                    *mode.write() = Mode::None;
                    *target_id.write() = 0;
                    e.stop_propagation();
                },
                "キャンセル"
            }
        }
    }
}

#[component]
fn PostEdit(id: i32, posted: Signal<String>, sender: Signal<String>, content: Signal<String>) -> Element {
    let mut set_posts = use_context::<Signal<Posts>>();
    let mut mode = use_context::<Signal<Mode>>();
    let mut target_id = use_context::<Signal<i32>>();

    rsx! {
        div {
            class: "mb-4",
            input {
                class: "d-block mb-2 me-2",
                placeholder: "お名前を修正してください",
                value: "{sender.read()}",
                oninput: move |e| {
                    sender.set(e.value().clone());
                    e.stop_propagation();
                },
                autofocus: "true",
            }
            textarea {
                class: "d-block w-100 mb-2",
                placeholder: "メッセージを修正してください",
                value: "{content.read()}",
                oninput: move |e| {
                    content.set(e.value().clone());
                    e.stop_propagation();
                }
            }
            button {
                r#type: "button",
                class: "btn btn-primary btn-sm me-2",
                onclick: move |e| {
                    if !sender.read().is_empty() && !content.read().is_empty() {
                        spawn( async move {
                            let message = Message {
                                id: id,
                                posted: posted.read().clone(),
                                sender: sender.read().clone(),
                                content: content.read().clone(),
                            };
                            let res = data::call_update(&message).await.unwrap();
                            match &res.result {
                                ResponseContent::Item(item) => {
                                    set_posts.write().insert(
                                        item.id,
                                        item.clone()
                                    );
                                },
                                _ => {}
                            };
                        });
                        *mode.write() = Mode::None;
                        e.stop_propagation();
                    }
                },
                "更新",
            }
            button {
                class: "btn btn-outline-dark btn-sm",
                onclick: move |e| {
                    //info!("onclick cancel {id}");
                    *target_id.write() = 0;
                    *mode.write() = Mode::None;
                    e.stop_propagation();
                },
                "キャンセル"
            }
        }
    }
}
