use gloo::console::log;
use gloo::dialogs::alert;
use gloo::timers::callback::Timeout;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew::{classes, html};

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
            alert("balls in your face sonnnn")
        }
    };

    html! {
        <div class="flex flex-col h-screen bg-400-red">
            <nav class="bg-green-400 h-16 px-8 py-2">
                <div class="container flex mx-auto gap-6 items-center h-full">
                    <button {onclick}>{ "+1" }</button>
                    <p>{ *counter }</p>
                </div>
            </nav>
        </div>
    }
}

fn main() {
    let timeout = Timeout::new(1_000, move || {
        let object = JsValue::from("any JsValue can be logged");
        log!("text", object);
    });

    // Since we don't plan on cancelling the timeout, call `forget`.
    timeout.forget();
    yew::Renderer::<App>::new().render();
}
