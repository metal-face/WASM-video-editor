mod router;
mod views;

use crate::router::index::{switch, Route};
use yew::{function_component, html, Html};
use yew_router::prelude::*;

#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <main class="h-screen w-screen">
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
