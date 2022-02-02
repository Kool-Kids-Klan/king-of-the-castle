use kotc_reqwasm::connect_websocket;

mod pages;
mod router;
mod header;

use yew_router::prelude::*;
use yew::prelude::*;
use kotc_reqwasm::connect_websocket;
use crate::router::{Route, switch};
use crate::header::Header;

#[function_component(App)]
fn app() -> Html {
   return html! {
    <div class="container" >
       <BrowserRouter>
            <Header />
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    </div>
   }
}

fn main() {
    connect_websocket();
    yew::start_app::<App>();
}
