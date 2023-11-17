mod components;
mod icons;
mod router;
mod views;

use crate::components::app_bar::AppBar;
use crate::components::navigation_drawer::NavigationDrawer;
use crate::router::index::{switch, Route};
use yew::{html, Component, Context, Html};
use yew_router::prelude::*;

pub enum Msg {
    ButtonClick(bool),
}
pub struct App {
    drawer_state: Option<bool>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            drawer_state: Some(false),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ButtonClick(drawer_state) => {
                self.drawer_state = Some(drawer_state);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_closed = ctx.link().callback(Msg::ButtonClick);
        let on_opened = ctx.link().callback(Msg::ButtonClick);

        html! {
            <BrowserRouter>
                <main class="h-screen w-screen">
                    <AppBar close_drawer={on_opened} drawer_state={self.drawer_state}/>

                    <NavigationDrawer drawer_state={self.drawer_state} toggle_state={on_closed}/>
                    <Switch<Route> render={switch} />
                </main>
            </BrowserRouter>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
