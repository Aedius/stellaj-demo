use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Hello World" }</h1>
        <p>
            <a href="/auth/login">{"login"}</a>
            <a href="/event/hello/">{"clic and add your name"}</a>
        </p>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}