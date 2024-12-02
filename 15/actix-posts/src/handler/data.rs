use std::fs;
use chrono::NaiveDateTime;
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

pub fn get(id: i32) -> Message {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let json_data: Vec<Message> = serde_json::from_str(&file).unwrap();
    let mut message = Message {id: 0, posted: "".to_string(), 
        sender: "".to_string(), content: "".to_string()};
    if let Some(index) = json_data.iter().position(|item| item.id == id) {
        message = json_data[index].clone();
    }
    message   
}

pub fn create(mut message: Message) -> Message {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let mut json_data: Vec<Message> = serde_json::from_str(&file).unwrap();
    let mut max = 0;
    for item in &json_data {
        max = std::cmp::max(item.id, max);
    }
    message.id = max + 1; 
    println!("Create: {} {} {} {}", 
        message.id, 
        message.posted,
        message.sender,
        message.content);
    json_data.push(message);
    let json_str = serde_json::to_string(&json_data).unwrap();
    let _ = fs::write(DATA_FILENAME, json_str);
    json_data.pop().unwrap()
}

pub fn update(message: &Message) {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let mut json_data: Vec<Message> = serde_json::from_str(&file).unwrap();
    if let Some(index) = json_data.iter().position(|item| item.id == message.id) {
        json_data[index] = message.clone();
        let json_str = serde_json::to_string(&json_data).unwrap();
        let _ = fs::write(DATA_FILENAME, json_str);
    }
}

pub fn remove(id: i32) {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let mut json_data: Vec<Message> = serde_json::from_str(&file).unwrap();
    json_data.retain(|item| item.id != id);
    let json_str = serde_json::to_string(&json_data).unwrap();
    let _ = fs::write(DATA_FILENAME, json_str);
}

// 以降、データベース版

use sqlx::sqlite::Sqlite;
use sqlx::pool::Pool;
//use sqlx::Row;          

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Post {
    pub id: i64,                // ID  
    pub posted: NaiveDateTime,  // 投稿日時
    pub sender: String,         // 投稿者名
    pub content: String,        // 投稿内容
}

pub async fn get_all_withdb(pool: Pool<Sqlite>) -> Vec<Message> {
    // マクロ版
    let posts = sqlx::query_as!(
        Post,
        "select id, posted, sender, content from posts order by posted desc"
    )
    // 関数版
    //let posts = sqlx::query_as::<_, Post>(
    //    "select id, posted, sender, content from posts order by posted desc"
    //)
    .fetch_all(&pool)
    .await.unwrap();
    let mut messages: Vec<Message> = Vec::new();
    for post in posts {
        let message = Message {
            id: post.id as i32, 
            posted: post.posted.to_string(), 
            sender: post.sender, 
            content: post.content};
        messages.push(message);
    }
    messages
}

pub async fn get_withdb(pool: Pool<Sqlite>, id: i32) -> Message {
    // マクロ版
    let post = sqlx::query_as!(
        Post,
        "select id, posted, sender, content from posts where id = ?",
        id
    )
    // 関数版
    //let post = sqlx::query_as::<_, Post>(
    //    "select id, posted, sender, content from posts where id = ?"
    //)
    //.bind(id)
    .fetch_one(&pool)
    .await.unwrap();
    let message = Message {
        id: post.id as i32, 
        posted: post.posted.to_string(), 
        sender: post.sender, 
        content: post.content
    };
    message   
}

pub async fn create_withdb(pool: Pool<Sqlite>, mut message: Message) -> Message {
    let post = Post {
        id: 0, 
        posted: NaiveDateTime::parse_from_str(&message.posted, "%Y-%m-%d %H:%M:%S").unwrap(), 
        sender: message.sender.clone(), content: message.content.clone()};
    let result = sqlx::query!(
        "insert into posts(posted, sender, content) values (?, ?, ?)",
        post.posted, post.sender, post.content)
    .execute(&pool)
    .await.unwrap();
    message.id = result.last_insert_rowid() as i32;
    message
}

pub async fn update_withdb(pool: Pool<Sqlite>, message: &Message) {
    let post = Post {
        id: message.id as i64, 
        posted: NaiveDateTime::parse_from_str(&message.posted, "%Y-%m-%d %H:%M:%S").unwrap(), 
        sender: message.sender.clone(), content: message.content.clone()};
    let _result = sqlx::query!(
        "update posts set posted = ?, sender = ?, content = ? where id = ?",
        post.posted, post.sender, post.content, post.id
    )
    .execute(&pool)
    .await.unwrap();
}

pub async fn remove_withdb(pool: Pool<Sqlite>, id: i32) {
    let _result = sqlx::query!(
        "delete from posts where id = ?", id
    )
    .execute(&pool)
    .await.unwrap();
}
