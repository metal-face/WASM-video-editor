use yew::{function_component, html, Html};

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    html! {
        <div class="h-full w-full flex justify-center align-middle">
            <div class="w-1/2 h-1/2 flex outline">
                <div>
                    <label for="username">{"Username"}</label>
                    <input id="username" type="text" placeholder="Username" />
                </div>
                <div>
                    <label for="password">{"Password"}</label>
                    <input id="password" type="password" placeholder="Password" />
                </div>
            </div>
        </div>

    }
}
