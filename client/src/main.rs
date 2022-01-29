use kotc_reqwasm::connect_websocket;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World!" }</h1>
    }
}

fn main() {
    connect_websocket();
    yew::start_app::<App>();
}
