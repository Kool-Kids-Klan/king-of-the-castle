mod components;
mod router;

use components::header::Header;
use kotc_reqwasm::connect_websocket;
use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::BasicStore;
use yewdux_functional::use_store;

#[derive(Clone, Debug, Default)]
pub struct User {
    id: u32,
    username: String,
}

#[derive(Clone, Debug)]
pub struct LoggedUser{
    logged_user: Option<User>,
}

impl Default for LoggedUser {
    fn default() -> Self {
        LoggedUser {
            logged_user: None,
        }
    }
}

#[function_component(App)]
fn app() -> Html {

    html! {
        <div class="container" >
            <BrowserRouter>
                <Header />
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </div>
    }
}

fn main() {
    // connect_websocket(); // TODO: pls find suitable place to call this method and change its return to KotcWebSocket
    yew::start_app::<App>();
    wasm_logger::init(wasm_logger::Config::default());
}
