use yew::prelude::*;
use yew_router::prelude::*;

use crate::views::home::Home;
use crate::views::login::Login;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Login => html! { <Login />},
    }
}
