use actix::Addr;
use actix_web::web::{Data, Path, Payload};
use actix_web::{get, post, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use serde::Deserialize;
use std::sync::Arc;

use pwhash::sha512_crypt::{hash, verify};

use crate::kotc_ws_server::KotcWsServer;
use crate::kots_ws_session::KotcWsSession;
use kotc_database::repo::user_repo::{UserRepo, PostgresUserRepo};

#[derive(Debug, Deserialize)]
pub struct UserData {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginData {
    username: String,
    password: String,
}

#[get("/users")]
pub async fn get_users(data: Data<Arc<PostgresUserRepo>>) -> impl Responder {
    let users = data.list_users().await.unwrap_or_default();

    HttpResponse::Ok().json(users)
}

#[get("/users/{user_id}")]
pub async fn get_user(path: Path<i32>, data: Data<Arc<PostgresUserRepo>>) -> impl Responder {
    let id = path.into_inner();
    let result = data.get_user(id).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().json(""),
    }
}

#[get("/lobby/{lobby_id}")] // Create WS session and connect it to lobby
pub async fn join_lobby(
    req: HttpRequest,
    stream: Payload,
    Path(lobby_id): Path<usize>,
    server: Data<Addr<KotcWsServer>>,
) -> Result<HttpResponse, Error> {
    let ws = KotcWsSession::new(lobby_id, server.get_ref().clone());

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}

#[post("/users/login")]
pub async fn verify_user(
    data: Data<Arc<PostgresUserRepo>>,
    body: String,
) -> impl Responder {
    let result = data.list_users().await;

    println!("{:?}", body);
    let login_body: LoginData = serde_json::from_str(&body).unwrap();
    println!("{:?}", login_body);

    match result {
        Ok(user) => {
            let verified = user.iter().filter(|user| user.username == login_body.username && verify(&login_body.password, &user.passhash)).last();
            HttpResponse::Ok().json(verified)
        }
        Err(_) => HttpResponse::NotFound().json(""),
    }
}

#[post("/users")]
pub async fn create_user(
    data: Data<Arc<PostgresUserRepo>>,
    body: String,
) -> impl Responder {
    println!("{:?}", body);
    let user_body: UserData = serde_json::from_str(&body).unwrap();
    println!("{:?}", user_body);

    let result = data
        .create_user(
            &user_body.username,
            &user_body.email,
            &hash(&user_body.password).unwrap(),
        )
        .await;

    match result {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}
