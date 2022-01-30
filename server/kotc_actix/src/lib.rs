mod handlers;

use std::sync::Arc;

use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use kotc_database::repo::user_repo::{
    PostgresUserRepo,
};
use kotc_database::establish_connection;

struct KotcWs;

impl Actor for KotcWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for KotcWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(KotcWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

// #[actix_web::main]
pub async fn start_actix_server() -> std::io::Result<()> {
    let conn_pool = Arc::new(establish_connection().await);

    let user_repo = Arc::new(PostgresUserRepo::new(conn_pool.clone()));

    HttpServer::new(move || {
        App::new()
            .route("/ws/", web::get().to(index))
            .app_data(web::Data::new(user_repo.clone()))
            .service(handlers::get_users)
            .service(handlers::get_user)
            .service(handlers::verify_user)
            .service(handlers::create_user)
    })
        .bind("127.0.0.1:8081")?
        .run()
        .await
}
