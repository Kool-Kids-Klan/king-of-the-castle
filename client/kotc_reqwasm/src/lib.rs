use futures::{SinkExt, StreamExt};
use log::{info, Level};
use reqwasm::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;

use kotc_commons::TestEvent;

pub fn connect_websocket() {
    console_log::init_with_level(Level::Debug).unwrap();
    let ws = WebSocket::open("ws://127.0.0.1:8081/lobby/1234").unwrap();
    let (mut write, mut read) = ws.split();

    spawn_local(async move {
        info!("FIRST SPAWN LOCAL, state");
        let test_event = TestEvent {
            user_id: 1,
            msg: String::from("message"),
        };
        let test_event_serialized = serde_json::to_string(&test_event).unwrap();
        write
            .send(Message::Text(test_event_serialized))
            .await
            .unwrap();
        write
            .send(Message::Text(String::from("test 2")))
            .await
            .unwrap();
        info!("FINISHING FIRST LOCAL");
    });

    spawn_local(async move {
        info!("SECOND SPAWN LOCAL");
        while let Some(msg) = read.next().await {
            info!("this is message {:?}", msg)
            // log!(format!("1. {:?}", msg));
        }
        // log!("WebSocket Closed");
    })
}
