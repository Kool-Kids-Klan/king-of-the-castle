use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;

use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use yew::Callback;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub passhash: String,
    pub games_played: i32,
    pub games_won: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Clone, Debug)]
pub struct LoggedUser{
    pub logged_user: Option<User>,
}

impl Default for LoggedUser {
    fn default() -> Self {
        LoggedUser {
            logged_user: None,
        }
    }
}

pub fn get_user(id: i32, store: Callback<Option<User>>) {
    spawn_local(async move {
        let resp = Request::get(&format!("http://127.0.0.1:8081/users/{}", id))
            .send()
            .await
            .unwrap();
        log::info!("{:?}", resp);
        let user: Option<User> = resp.json().await.unwrap();
        log::info!("{:?}", user);
        store.emit(user);
    })
}

#[derive(Debug, Serialize)]
pub struct LoginData {
    username: String,
    password: String,
}

pub fn login_user(username: String, password: String, store: Callback<Option<User>>) {
    spawn_local(async move {
        let login_data = LoginData {username, password};
        let body = serde_json::to_string(&login_data).unwrap();
        log::info!("{:?}", body);
        let resp = Request::post(&format!("http://127.0.0.1:8081/users/login"))
            .body(body)
            .send()
            .await
            .unwrap();
        log::info!("{:?}", resp);
        let user: Option<User> = resp.json().await.unwrap();
        log::info!("{:?}", user);
        store.emit(user);
    })
}

#[derive(Debug, Serialize)]
pub struct RegisterData {
    username: String,
    email: String,
    password: String,
}

pub fn register_user(username: String, email: String, password: String) {
    spawn_local(async move {
        let register_data = RegisterData {username, email, password};
        let body = serde_json::to_string(&register_data).unwrap();
        log::info!("{:?}", body);
        let resp = Request::post(&format!("http://127.0.0.1:8081/users"))
            .body(body)
            .send()
            .await
            .unwrap();
        log::info!("{:?}", resp);
        log::info!("Register successful: {:?}", resp.ok());
    })
}
