use wasm_bindgen::{prelude::*, JsCast};
use js_sys::{JSON, Array};
use crate::utils::log;

#[derive(Clone)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub period: String,
    pub completed: bool,
}

pub struct TaskStore {
    local_storage: web_sys::Storage,
    name: String,
    pub list: Vec<Task>,
}

impl TaskStore {
    pub fn new(name: &str) -> Option<TaskStore> {
        let window = web_sys::window()?;
        if let Ok(Some(local_storage)) = window.local_storage() {
            let mut store = TaskStore {
                local_storage,
                name: String::from(name),
                list: Vec::new(),
            };
            store.fetch_data();
            /*
            let dummy = Task {id: 1, title: "生活費を口座に入金する".to_string(), 
                period: "2024-09-20".to_string(), completed: false};
            store.list.push(dummy);
            let dummy = Task {id: 2, title: "メルカルの荷物を発送する".to_string(), 
                period: "2024-09-25".to_string(), completed: true};
            store.list.push(dummy);
            let dummy = Task {id: 3, title: "チカイワの抽選に申し込む".to_string(), 
                period: "2024-09-30".to_string(), completed: false};
            store.list.push(dummy);
             */
            Some(store)
        } else {
            None
        }
    }

    // 初期データ読み込み
    fn fetch_data(&mut self) -> Option<()> {
        let mut item_list = Vec::<Task>::new();
        if let Ok(Some(value)) = self.local_storage.get_item(&self.name) {
            log(&value);
            let data = JSON::parse(&value).ok()?;
            let iter = js_sys::try_iter(&data).ok()??;
            for item in iter {
                let item = item.ok()?;
                let item_array: &Array = JsCast::dyn_ref(&item)?;
                let id = item_array.shift().as_f64()?;
                let id = id as i32;
                let title = item_array.shift().as_string()?;
                let period = item_array.shift().as_string()?;
                let completed = item_array.shift().as_bool()?;
                item_list.push(Task {id, title, period, completed,});
            }
        }
        self.list = item_list;
        Some(())
    }

    // データ保存
    fn sync_data(&mut self) {
        let array = Array::new();
        for item in self.list.iter() {
            let child = Array::new();
            child.push(&JsValue::from(item.id));
            child.push(&JsValue::from(&item.title));
            child.push(&JsValue::from(&item.period));
            child.push(&JsValue::from(item.completed));
            array.push(&JsValue::from(child));
        }
        if let Ok(storage_string) = JSON::stringify(&JsValue::from(array)) {
            let storage_string: String = storage_string.into();
            log(&storage_string);
            self.local_storage.set_item(&self.name, &storage_string).unwrap();
        }
    }

    // タスク検索
    pub fn get(&self, id: i32) -> Option<Task> {
        for i in &self.list {
            if i.id == id {
                return Some((*i).clone());
            }
        }
        None
    }

    // タスク挿入
    pub fn insert(&mut self, item: Task) {
        self.list.push(item);
        self.sync_data();
    }

    // タスク更新
    pub fn update(&mut self, item: Task) {
        for i in &mut self.list {
            if i.id == item.id {
                *i = item;
                break;
            }
        }
        self.sync_data();
    }

    // タスク削除
    pub fn remove(&mut self, id: i32) {
        self.list.retain(|item| item.id != id);
        self.sync_data();
    }

}
