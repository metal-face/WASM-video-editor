mod components;
mod icons;
mod router;
mod views;

use crate::components::app_bar::AppBar;
use crate::router::index::{switch, Route};
use yew::{function_component, html, Html};
use yew_router::prelude::*;

#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <main class="h-screen w-screen">
                <AppBar />
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
