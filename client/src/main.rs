mod components;
mod router;

use components::header::Header;
use gloo_storage::{SessionStorage, Storage};
use kotc_reqwasm::endpoints::LoggedUser;
use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::*;

#[function_component(App)]
fn app() -> Html {
    let store = use_store::<BasicStore<LoggedUser>>();
    let user_state = store.state();
    let maybe_user = SessionStorage::get("user");
    if let Ok(user) = maybe_user {
        match user_state {
            Some(_) => {}
            None => store
                .dispatch()
                .reduce(|state| state.logged_user = Some(user)),
        }
    };

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
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
