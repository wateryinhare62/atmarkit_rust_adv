use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: i32,                // ID  
    pub posted: String,         // 投稿日時
    pub sender: String,         // 投稿者名
    pub content: String,        // 投稿内容
}

static DATA_FILENAME: &str = "data.json";

pub fn get_all() -> Vec<Message> {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let mut json_data: Vec<Message> = serde_json::from_str(&file).unwrap();
    json_data.sort_by(|a, b| b.posted.cmp(&a.posted));
    json_data   
}
