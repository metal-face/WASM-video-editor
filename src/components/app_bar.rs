use crate::components::github_login_button::GitHubLoginButton;
use crate::icons::hamburger_menu::HamburgerMenu;
use yew::{classes, html, Callback, Component, Context, Html, Properties};
use yew_router::components::Link;

use crate::router::index::Route;

pub struct AppBar;

#[derive(Clone, PartialEq, Properties)]
pub struct AppBarProps {
    pub close_drawer: Callback<bool>,
    pub drawer_state: Option<bool>,
}

impl Component for AppBar {
    type Message = ();
    type Properties = AppBarProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let close_drawer = ctx.props().close_drawer.reform(move |_| false);
        let open_drawer = ctx.props().close_drawer.reform(move |_| true);

        if ctx.props().drawer_state.unwrap().clone() {
            html! {
                <div class={classes!("flex", "items-center", "shadow-xl", "sticky", "h-24",  "z-20", "w-full", "bg-slate-800")}>
                    <div class={classes!("h-max", "w-1/2", "pl-3", "text-white")}>
                        <button onclick={close_drawer} type="button" class={classes!("p-1", "rounded-full", "hover:bg-slate-500", "ease-in-out")}>
                            <HamburgerMenu />
                        </button>
                    </div>
                    <div class={classes!("flex", "justify-end", "items-center", "h-auto", "w-1/2")}>
                        <div class={classes!("h-fit", "text-white", "hover:underline", "text-xl", "m-1")}>
                            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                        </div>
                        <div class="h-fit m-1">
                            <GitHubLoginButton />
                        </div>
                    </div>
                </div>
            }
        } else {
            html! {
                <div class={classes!("flex", "items-center", "shadow-xl", "sticky", "h-24",  "z-20", "w-full", "bg-slate-800",)}>
                    <div class={classes!("h-max", "w-1/2", "pl-3", "text-white")}>
                        <button onclick={open_drawer} type="button" class={classes!("p-1", "rounded-full", "hover:bg-slate-500", "ease-in-out")}>
                            <HamburgerMenu />
                        </button>
                    </div>
                    <div class={classes!("flex", "justify-end", "items-center", "h-auto", "w-1/2")}>
                        <div class={classes!("h-fit", "text-white", "text-xl", "hover:underline", "m-1")}>
                            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                        </div>

                        <div>
                            <GitHubLoginButton />
                        </div>
                    </div>
                </div>
            }
        }
    }
}
