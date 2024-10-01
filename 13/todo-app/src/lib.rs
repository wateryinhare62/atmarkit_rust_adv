mod utils;
mod store;
mod template;

use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use chrono::{Local, Duration};
use utils::log;
use store::TaskStore;

/* #[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, todo-app!");
} */

// アプリの基点となる関数
#[wasm_bindgen(start)]
pub fn initial() {
    log(&"Init app".to_string());
    if let Some(storage) = TaskStore::new("rust_todo_app") {
        let window = web_sys::window().unwrap();
        let document = Rc::new(window.document().unwrap());
        let mut maximum_id = 0;
        let storage = Rc::new(RefCell::new(storage));
        let item_element = document.get_element_by_id("items").unwrap();
        for item in storage.borrow().list.iter() {
            let item_rendered = template::render_item(&item);
            let _ = item_element
                .insert_adjacent_html("afterbegin", &item_rendered);
            if item.id > maximum_id {
                maximum_id = item.id;
            }
        }
        if maximum_id > 0 {
            let _ = document.get_element_by_id("no_task").unwrap()
                .dyn_ref::<HtmlElement>().unwrap()
                .style().set_property("display", "none");
        }
        // 新規タスクボタンクリック
        {
            let document_clone = document.clone();
            let storage = storage.clone();
            let new_button_event = Closure::<dyn FnMut()>::new(move || {
                log(&"New button clicked".to_string());
                maximum_id += 1;
                let now = Local::now();
                let period = now + Duration::days(7);
                let item = store::Task { id: maximum_id, title: "新規タスク".into(), 
                    period: period.format("%Y-%m-%d").to_string(), completed: false };
                storage.borrow_mut().insert(item.clone());
                let item_rendered = template::render_item(&item);
                let _ = document_clone.get_element_by_id("items").unwrap()
                    .insert_adjacent_html("afterbegin", &item_rendered);
                let _ = document_clone.get_element_by_id("no_task").unwrap()
                    .dyn_ref::<HtmlElement>().unwrap()
                    .style().set_property("display", "none");
            });
            document.get_element_by_id("new_button").unwrap()
                .dyn_ref::<HtmlElement>().unwrap()
                .set_onclick(Some(new_button_event.as_ref().unchecked_ref()));
            new_button_event.forget();
        }
    }
    
}
