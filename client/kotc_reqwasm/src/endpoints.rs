use std::collections::HashMap;

use gloo_storage::{SessionStorage, Storage};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;

use crate::{Color, Player};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use yew::Callback;

use crate::server_structs::{Card, Column, Token};

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

#[derive(Clone, Debug, Default)]
pub struct LoggedUser {
    pub logged_user: Option<User>,
}

#[derive(Clone, Debug, Default)]
pub struct GameStarted {
    pub game_started: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ColumnsStore {
    pub columns: Vec<Column>,
}

#[derive(Clone, Debug, Default)]
pub struct HandStore {
    pub hand: Vec<Card>,
}

#[derive(Clone, Debug, Default)]
pub struct LogStore {
    pub logs: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct TokenStore {
    pub tokens: HashMap<String, (Color, Vec<Token>)>,
}

#[derive(Clone, Debug, Default)]
pub struct CardStore {
    pub card: Option<usize>,
}

#[derive(Clone, Debug, Default)]
pub struct PlayerOnTurnStore {
    pub player: Option<Player>,
}

#[derive(Clone, Debug, Default)]
pub struct FinalResultsStore {
    pub game_ended: bool,
    pub results: HashMap<String, (Color, u8)>,
}

pub fn get_server_url() -> String {
    match option_env!("SERVER_URL") {
        Some(url) => url.to_string(),
        _ => "127.0.0.1:8081".to_string(),
    }
}

pub fn get_user(id: i32, store: Callback<Option<User>>) {
    spawn_local(async move {
        let resp = Request::get(&format!("http://{}/users/{}", get_server_url(), id))
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
        let login_data = LoginData { username, password };
        let body = serde_json::to_string(&login_data).unwrap();
        log::info!("{:?}", body);
        let resp = Request::post(&format!("http://{}/users/login", get_server_url()))
            .body(body)
            .send()
            .await
            .unwrap();
        log::info!("{:?}", resp);
        let user: Option<User> = resp.json().await.unwrap();
        log::info!("{:?}", user);
        SessionStorage::set("user", &user).unwrap();
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
        let register_data = RegisterData {
            username,
            email,
            password,
        };
        let body = serde_json::to_string(&register_data).unwrap();
        log::info!("{:?}", body);
        let resp = Request::post(&format!("http://{}/users", get_server_url()))
            .body(body)
            .send()
            .await
            .unwrap();
        log::info!("{:?}", resp);
        log::info!("Register successful: {:?}", resp.ok());
    })
}
