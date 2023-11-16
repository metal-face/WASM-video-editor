use yew::{html, AttrValue, Callback, Component, Context, Html, Properties, UseStateHandle};

use super::*;

pub struct NavigationDrawer;

#[derive(Clone, PartialEq, Properties)]
pub struct NavDrawerProps {
    pub drawer_state: Option<bool>,
    pub toggle_state: Callback<bool>,
}   

impl Component for NavigationDrawer {
    type Message = ();
    type Properties = NavDrawerProps;

    fn create(ctx: &yew::prelude::Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.props().toggle_state.reform(move |_| false);
        if ctx.props().drawer_state.unwrap() {
            html! {
                <nav class="h-full w-1/4 bg-slate-400">
                    <div>
                        <p>{"Good Day"}</p>
                    </div>
                    <button type="button" {onclick}>{"hell yeah"}</button>
                </nav>
            }
        } else {
            html! {
                <></>
            }
        }
    }
}
