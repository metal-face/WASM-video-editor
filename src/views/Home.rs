use yew::{classes, function_component, html, use_state, Html};

#[function_component(Home)]
pub fn Home() -> Html {
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
                <div class={classes!("bg-red-400", "h-max")}>
                    <button {onclick}>{ "+1" }</button>
                    <p class={classes!("text-emerald-600")}>{ *counter }</p>
                </div>
            </nav>
        </div>
    }
}
