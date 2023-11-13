use yew::{function_component, html, Html};

use crate::components::login_form::LoginForm;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <div class="bg-yellow-300 h-screen w-screen">
            <LoginForm />
        </div>
    }
}
