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
            let dummy = Task {id: 1, title: "生活費を口座に入金する".to_string(), 
                period: "2024-08-20".to_string(), completed: false};
            store.list.push(dummy);
            let dummy = Task {id: 2, title: "メルカルの荷物を発送する".to_string(), 
                period: "2024-08-25".to_string(), completed: true};
            store.list.push(dummy);
            let dummy = Task {id: 3, title: "チカイワの抽選に申し込む".to_string(), 
                period: "2024-08-31".to_string(), completed: false};
            store.list.push(dummy);
            Some(store)
        } else {
            None
        }
    }
}
