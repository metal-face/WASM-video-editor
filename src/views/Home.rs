use yew::{function_component, html, use_state, Html};

#[function_component(Home)]
pub fn home() -> Html {
    let counter = use_state(|| 0);
    let add = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let subtract = {
        let counter = counter.clone();
        move |_| {
            let value = *counter - 1;
            counter.set(value);
        }
    };

    html! {
        <div class="bg-pink-400 p-2 h-full w-full flex flex-col justify-center">
            <div class=" h-fit w-full flex justify-center align-middle">
                <button type="button" onclick={subtract} class="bg-slate-400 rounded p-1 m-1 h-fit shadow-xl">{"Minus One"}</button>
                <p class="h-fit w-12 p-1 rounded shadow-xl bg-yellow-400 text-black text-center m-1" >{ *counter }</p>
                <button type="button" class="h-fit w-fit p-1 bg-slate-400 rounded m-1 cursor-pointer shadow-lg" onclick={add}>{ "Add One" }</button>
            </div>
        </div>
    }
}
