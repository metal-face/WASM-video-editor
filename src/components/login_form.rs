use yew::{function_component, html, Html};

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    html! {
        <div class="h-full w-full flex justify-center items-center">
            <div class="w-1/2 h-1/2 flex flex-col items-center outline">
                <div class="h-full w-full flex flex-col justify-center items-center ">
                    <div class="flex flex-col">
                        <label class="text-center" for="username">{"Username"}</label>
                        <input class="rounded p-1 shadow-" id="username" type="text" placeholder="Username" />
                    </div>
                    <div class="flex flex-col">
                        <label class="text-center" for="password">{"Password"}</label>
                        <input class="rounded p-1" id="password" type="password" placeholder="Password" />
                    </div>
                    <div class="flex h-fit justify-items-end items-end">
                        <button class="rounded p-1 shadow-xl bg-slate-600 text-white" type="button">{"Login"}</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
