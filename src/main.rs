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
        }
    };

    html! {
        <div class={classes!("bg-red-400", "h-full")}>
            <nav class={classes!("h-full")}>
                <div class={classes!("bg-red-400", "h-full")}>
                    <button {onclick}>{ "+1" }</button>
                    <p class={classes!("text-emerald-600")}>{ *counter }</p>
                </div>
            </nav>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
