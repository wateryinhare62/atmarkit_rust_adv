mod utils;
mod store;
mod template;

use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, HtmlInputElement, MouseEvent};
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
        let current_id = Rc::new(RefCell::new(0));
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
        // 編集フォームクリック
        {
            let form_click_event = Closure::<dyn FnMut(_)>::new(move |e: MouseEvent| {
                e.stop_propagation();
            });
            document.get_element_by_id("edit_form").unwrap()
                .dyn_ref::<HtmlElement>().unwrap()
                .set_onclick(Some(form_click_event.as_ref().unchecked_ref()));
            form_click_event.forget();
        }
        // タスク一覧クリック
        {
            let document_clone = document.clone();
            let storage = Rc::clone(&storage);
            let current_id = Rc::clone(&current_id);
            let task_click_event = Closure::<dyn FnMut(_)>::new(move |e: MouseEvent| {
                let mut target_element = e.target().unwrap()
                    .dyn_ref::<HtmlElement>().unwrap()
                    .clone();
                log(&format!("Clicked id: {}", target_element.id()));
                let mut id = target_element.id();
                while !id.starts_with("task") {
                    //log(&format!("Task {} clicked", hidden.dyn_ref::<HtmlInputElement>().unwrap().value()));
                    target_element = target_element.parent_element().unwrap()
                        .dyn_ref::<HtmlElement>().unwrap()
                        .clone();
                    id = target_element.dyn_ref::<HtmlElement>().unwrap().id();
                    if id == "items".to_string() {
                        break;
                    }
                }
                log(&format!("Found id: {}", id));
                if id != "items".to_string() {
                    *current_id.borrow_mut() = target_element
                        .first_element_child().unwrap()
                        .first_element_child().unwrap()
                        .dyn_ref::<HtmlInputElement>().unwrap()
                        .value().parse().unwrap();
                    log(&format!("Task {} clicked", *current_id.borrow()));
                    let item = storage.borrow().get(*current_id.borrow()).unwrap();
                    document_clone.get_element_by_id("edit_completed").unwrap()
                        .dyn_ref::<HtmlInputElement>().unwrap()
                        .set_checked(item.completed);
                    document_clone.get_element_by_id("edit_title").unwrap()
                        .dyn_ref::<HtmlInputElement>().unwrap()
                        .set_value(item.title.as_str());
                    document_clone.get_element_by_id("edit_period").unwrap()
                        .dyn_ref::<HtmlInputElement>().unwrap()
                        .set_value(item.period.as_str());
                    let form_element = document_clone.get_element_by_id("edit_form").unwrap();
                    let items_element = document_clone.get_element_by_id("items").unwrap();
                    let _ = items_element.insert_before(&form_element, Some(&target_element));
                    form_element
                        .dyn_ref::<HtmlElement>().unwrap()
                        .style().set_property("display", "block").unwrap();
                    target_element
                        .dyn_ref::<HtmlElement>().unwrap()
                        .style().set_property("display", "none").unwrap();
                }
            });
            document.get_element_by_id("items").unwrap()
                .dyn_ref::<HtmlElement>().unwrap()
                .set_onclick(Some(task_click_event.as_ref().unchecked_ref()));
            task_click_event.forget();
        }
        // 編集フォーム更新ボタンクリック
        {
            let document_clone = document.clone();
            let storage = Rc::clone(&storage);
            let current_id = Rc::clone(&current_id);
            let ok_button_event = Closure::<dyn FnMut(_)>::new(move |e: MouseEvent| {
                log(&"Edit Ok button clicked".to_string());
                document_clone.get_element_by_id("edit_form").unwrap()
                    .dyn_ref::<HtmlElement>().unwrap()
                    .style().set_property("display", "none").unwrap();
                log(&format!("Task {} clicked", *current_id.borrow()));
                let mut item = storage.borrow().get(*current_id.borrow()).unwrap();
                item.completed = document_clone.get_element_by_id("edit_completed").unwrap()
                    .dyn_ref::<HtmlInputElement>().unwrap().clone().checked();
                item.title = document_clone.get_element_by_id("edit_title").unwrap()
                    .dyn_ref::<HtmlInputElement>().unwrap().clone().value();
                item.period = document_clone.get_element_by_id("edit_period").unwrap()
                    .dyn_ref::<HtmlInputElement>().unwrap().clone().value();
                storage.borrow_mut().update(item.clone());
                let item_rendered = template::render_item(&item);
                let item_element = document_clone.get_element_by_id(&format!("task{}", 
                    *current_id.borrow())).unwrap();
                item_element
                    .insert_adjacent_html("afterend", &item_rendered).unwrap();
                item_element.remove();
                e.stop_propagation();
            });
            document.get_element_by_id("edit_ok_button").unwrap()
                .dyn_ref::<HtmlElement>().unwrap()
                .set_onclick(Some(ok_button_event.as_ref().unchecked_ref()));
            ok_button_event.forget();
        }
        // 編集フォームキャンセルボタンクリック
        {
            let document_clone = document.clone();
            let current_id = Rc::clone(&current_id);
            let cancel_button_event = Closure::<dyn FnMut(_)>::new(move |e: MouseEvent| {
                log(&"Edit cancel button clicked".to_string());
                document_clone.get_element_by_id("edit_form").unwrap()
                    .dyn_ref::<HtmlElement>().unwrap()
                    .style().set_property("display", "none").unwrap();
                document_clone.get_element_by_id(&format!("task{}", 
                    *current_id.borrow())).unwrap().dyn_ref::<HtmlElement>().unwrap()
                    .style().set_property("display", "block").unwrap();
                e.stop_propagation();
            });
            document.get_element_by_id("edit_cancel_button").unwrap()
                .dyn_ref::<HtmlElement>().unwrap()
                .set_onclick(Some(cancel_button_event.as_ref().unchecked_ref()));
            cancel_button_event.forget();
        }
        // 編集フォーム削除ボタンクリック
        {
            let document_clone = document.clone();
            let current_id = Rc::clone(&current_id);
            let storage = Rc::clone(&storage);
            let delete_button_event = Closure::<dyn FnMut(_)>::new(move |e: MouseEvent| {
                log(&"Edit Delete button clicked".to_string());
                document_clone.get_element_by_id("edit_form").unwrap()
                    .dyn_ref::<HtmlElement>().unwrap()
                    .style().set_property("display", "none").unwrap();
                log(&format!("Task {} clicked", *current_id.borrow()));
                storage.borrow_mut().remove(*current_id.borrow() as i32);
                document_clone.get_element_by_id(&format!("task{}", 
                    *current_id.borrow())).unwrap().remove();
                if storage.borrow().list.len() == 0 {
                    let no_task_element = document_clone.get_element_by_id("no_task").unwrap();
                    let _ = no_task_element.dyn_ref::<HtmlElement>().unwrap()
                        .style().set_property("display", "block");
                }
                e.stop_propagation();
            });
            document.get_element_by_id("edit_delete_button").unwrap()
                .dyn_ref::<HtmlElement>().unwrap()
                .set_onclick(Some(delete_button_event.as_ref().unchecked_ref()));
            delete_button_event.forget();
        }
    }
}
