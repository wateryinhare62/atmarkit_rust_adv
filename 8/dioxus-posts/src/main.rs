use dioxus::prelude::*;
use log::info;

mod data;
use data::{Message, ResponseContent};

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).unwrap();
    dioxus_web::launch(App);
}

type Posts = im_rc::HashMap<i32, Message>;

#[component]
fn App(cx: Scope) -> Element {
    //info!("Called App");
    //cx.render(
    //    rsx! {
    //        div { "Hello, World!!" }
    //    }
    //)
    let posts_source = use_future(cx, (), |_| data::call_index());
    let posts = use_ref(cx, Posts::default);

    cx.render(
        match posts_source.value() {
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
                            rsx!{ div { "{serde_json::to_string(&item).unwrap()}" } },
                        ResponseContent::Reason(reason) => rsx!{ div { "{reason}" } },
                        ResponseContent::None => rsx!{ div {} },
                    }
                } else {
                    //rsx! { div { "ここに投稿データを表示します" } }
                    let mut filtered_posts = posts.read()
                        .iter()
                        .map(|f| *f.0)
                        .collect::<Vec<_>>();
                    filtered_posts.sort_unstable_by(|a, b| b.cmp(a));
                    rsx! {
                        filtered_posts.iter().map(|id| 
                            rsx!(PostEntry { 
                                id: *id, 
                                set_posts: posts 
                            })
                        )
                    }
                }
            },
            Some(Err(err)) => rsx! { div { "初期データの読み込みに失敗しました：{err}" } },
            None => rsx! { div { "データを読み込んでいます..." } }
        }
    )
}

#[derive(Props)]
struct PostEntryProps<'a> {
    set_posts: &'a UseRef<Posts>,
    id: i32,
}

#[component]
fn PostEntry<'a>(cx: Scope<'a, PostEntryProps<'a>>) -> Element {
    let posts = cx.props.set_posts.read();
    let post = &posts[&cx.props.id];

    render!(div {
        class: "card mb-3",
        div {
            div {
                class: "card-header",
                "{post.sender} {post.posted}"
            }
            div {
                class: "card-body",
                p {
                    class: "card-text",
                    "{post.content}"
                }
            }
        }
    })
}

