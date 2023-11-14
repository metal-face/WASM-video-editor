use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub open: Callback<bool>,
}



#[function_component(NavigationDrawer)]
pub fn navigation_drawer(props: &Props) -> Html {
    

    if (props.open) {
        html! {
            <nav class="h-full w-1/3 bg-slate-600">
                <button onclick={open.emit()}>{"X"}</button>

            </nav>

        }
    }
}
