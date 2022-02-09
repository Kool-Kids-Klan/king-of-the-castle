mod handlers;
mod kotc_messages;
mod kotc_ws_server;
mod kots_ws_session;
mod lobby;

use actix::Actor;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

use crate::kotc_ws_server::KotcWsServer;
use kotc_database::get_user_repo;

pub async fn start_actix_server() -> std::io::Result<()> {
    let user_repo = get_user_repo().await;

    let kotc_ws_server = KotcWsServer::default().start(); // Create and start Kotc WebSocket server

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin();

        App::new()
            .wrap(cors)
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
