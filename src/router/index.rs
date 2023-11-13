use yew::prelude::*;
use yew_router::prelude::*;

use crate::views::home::Home;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
    }
}
