use crate::components::game::Game;
use crate::components::pages::home::Home;
use crate::components::pages::lobby::Lobby;
use crate::components::pages::login::Login;
use crate::components::pages::not_found::NotFound;
use crate::components::pages::register::Register;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/lobby")]
    Lobby,
    #[at("/game")]
    Game,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Login => html! { <Login /> },
        Route::Register => html! { <Register /> },
        Route::Lobby => html! { <Lobby /> },
        Route::Game => html! { <Game /> },
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <NotFound /> },
    }
}
