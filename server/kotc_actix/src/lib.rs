mod handlers;
mod kotc_messages;
mod kotc_ws_server;
mod kots_ws_session;

use actix::Actor;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;

use crate::kotc_ws_server::KotcWsServer;
use kotc_database::establish_connection;
use kotc_database::repo::user_repo::PostgresUserRepo;

pub async fn start_actix_server() -> std::io::Result<()> {
    let conn_pool = Arc::new(establish_connection().await);

    let user_repo = Arc::new(PostgresUserRepo::new(conn_pool.clone()));

    let kotc_ws_server = KotcWsServer::default().start(); // Create and start Kotc WebSocket server

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_repo.clone()))
            .app_data(web::Data::new(kotc_ws_server.clone()))
            .service(handlers::get_users)
            .service(handlers::get_user)
            .service(handlers::verify_user)
            .service(handlers::create_user)
            .service(handlers::join_lobby)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
