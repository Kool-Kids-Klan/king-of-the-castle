use yew_router::prelude::*;
use yew::prelude::*;
use crate::pages::login::Login;
use crate::pages::register::Register;
use crate::pages::lobby::Lobby;
use crate::pages::game::Game;
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

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
