mod pages;
mod router;
mod header;

use yew_router::prelude::*;
use yew::prelude::*;
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
    yew::start_app::<App>();
}