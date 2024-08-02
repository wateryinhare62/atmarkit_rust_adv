use serde::{Serialize, Deserialize};
use log::info;

static BASE_API_URL: &str = "http://localhost:8000/api/posts";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: i32,
    pub posted: String,
    pub sender: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseContent {
    Items(Vec<Message>),
    Item(Message),
    Reason(String),
    None,   
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    pub status: String,
    pub result: ResponseContent,
}

pub async fn call_index() -> Result<ApiResponse, reqwest::Error> {
    let url = format!("{}", BASE_API_URL);
    //reqwest::get(&url).await?.json::<ApiResponse>().await
    let client = reqwest::Client::new();
    client.get(&url)
        //.fetch_mode_no_cors()
        .send()
        .await?.json::<ApiResponse>().await
}

pub async fn call_create(message: &Message) -> Result<ApiResponse, reqwest::Error> {
    let url = format!("{}/create", BASE_API_URL);
    let client = reqwest::Client::new();
    info!("{}", serde_json::to_string(&message).unwrap());
    client.post(&url)
        //.fetch_mode_no_cors()
        .json(message)
        .send()
        .await?.json::<ApiResponse>().await
}

pub async fn call_update(message: &Message) -> Result<ApiResponse, reqwest::Error> {
    let url = format!("{}/update", BASE_API_URL);
    let client = reqwest::Client::new();
    info!("{}", serde_json::to_string(&message).unwrap());
    client.put(&url)
        //.fetch_mode_no_cors()
        .json(message)
        .send()
        .await?.json::<ApiResponse>().await
}

pub async fn call_delete(id: i32) -> Result<ApiResponse, reqwest::Error> {
    let url = format!("{}/{}/delete", BASE_API_URL, id);
    let client = reqwest::Client::new();
    info!("{}", id);
    client.delete(&url)
        //.fetch_mode_no_cors()
        .send()
        .await?.json::<ApiResponse>().await
}