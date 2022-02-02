use actix::Addr;
use actix_web::web::{Data, Path, Payload, Json};
use actix_web::{get, post, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use serde::Deserialize;
use std::sync::Arc;

use pwhash::sha512_crypt::{hash, verify};

use crate::kotc_ws_server::KotcWsServer;
use crate::kots_ws_session::KotcWsSession;
use kotc_database::repo::user_repo::*;

extern crate google_signin;

#[derive(Debug, Deserialize)]
pub struct UserData {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct PasswordData {
    password: String,
}

#[get("/users")]
pub async fn get_users(data: Data<Arc<PostgresUserRepo>>, req: HttpRequest) -> impl Responder {
    let mut client = google_signin::Client::new();
    client.audiences.push("229405536082-o0p730oresk0eeprtm1j9p27523thc47.apps.googleusercontent.com".to_string()); // required
    let users = data.list_users().await.unwrap_or_default();
    let x = req.head().headers().get("authorization").unwrap().to_str().unwrap();
    println!("{:?}", x);
    let mut y = x.split(' ');
    y.next();
    let z = y.next().unwrap();
    println!("{}", z);
    let id_info = client.verify(z).expect("Expected token to be valid");
    println!("{:?}", id_info);

    HttpResponse::Ok().json(users)
}

#[get("/users/{user_id}")]
pub async fn get_user(
    path: Path<i32>,
    data: Data<Arc<PostgresUserRepo>>,
) -> impl Responder {
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
    srv: Data<Addr<KotcWsServer>>,
) -> Result<HttpResponse, Error> {
    let ws = KotcWsSession::new(lobby_id, srv.get_ref().clone());

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}

#[post("/users/{user_id}/login")]
pub async fn verify_user(
    path: Path<i32>,
    data: Data<Arc<PostgresUserRepo>>,
    body: Json<PasswordData>,
) -> impl Responder {
    let id = path.into_inner();
    let result = data.get_user(id).await;

    match result {
        Ok(user) => {
            let verified = verify(&body.password, &user.passhash);
            HttpResponse::Ok().json(verified)
        }
        Err(_) => HttpResponse::NotFound().json(""),
    }
}

#[post("/users")]
pub async fn create_user(
    data: Data<Arc<PostgresUserRepo>>,
    user_body: Json<UserData>,
) -> impl Responder {
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
