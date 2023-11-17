use crate::icons::github_icon::GitHubIcon;
use yew::{function_component, html, Html};

#[function_component]
pub fn GitHubLoginButton() -> Html {
    html! {
        <button class="m-1 text-xl flex items-center justify-around w-36 h-12 bg-black text-white rounded shadow-xxl">{"Login"} <GitHubIcon /></button>
    }
}
