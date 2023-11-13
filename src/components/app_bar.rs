use yew::{function_component, html, Html};
use yew_router::components::Link;

use crate::router::index::Route;

#[function_component(AppBar)]
pub fn app_bar() -> Html {
    html! {
        <div style="align-items: center;"  class="flex shadow-xl sticky h-12 w-full bg-slate-800">
            <div class="h-auto w-1/2 pl-3">
                <h1 class=" text-white">{"WASM Video Editor"}</h1>
            </div>
            <div class="flex justify-end h-auto w-1/2">
                <div class="h-fit text-white hover:underline m-1">
                    <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                </div>
                <div class="m-1 text-white">
                    <Link<Route> to={Route::Login}>{"Login"}</Link<Route>>
                </div>
            </div>
        </div>
    }
}
