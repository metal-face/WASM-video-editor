use yew::{function_component, html, Html};
use yew_router::components::Link;

use crate::router::index::Route;

#[function_component(AppBar)]
pub fn app_bar() -> Html {
    html! {
        <div class="sticky w-screen h-16 bg-slate-800 flex justify-between align-middle">
            <div class="w-1/2 h-max">
                <h1>{"WASM Video Editor"}</h1>
            </div>
            <div class="w-1/2">
                <div class="mr-12">
                    <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                </div>
            </div>
        </div>
    }
}
