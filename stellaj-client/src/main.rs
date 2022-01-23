mod welcome;

use std::fmt::Debug;

use yew::prelude::*;

use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::html;

use welcome::WelcomeComp;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tokens {
    access_token: String,
    expires_in: usize,
    refresh_expires_in: usize,
    refresh_token: String,
    token_type: String,
    id_token: String,
    #[serde(rename = "not-before-policy")]
    not_before_policy: usize,
    session_state: String,
    scope: String,
}

#[function_component(App)]
fn app() -> Html {
    let window = window().unwrap();
    let location = window.location();
    let params = location.search().unwrap();

    if !params.is_empty() {
        log::info!("location : {:?}", params);
        spawn_local(async move {
            let resp = Request::post(format!("/auth/login-token{}", params).as_str())
                .send()
                .await
                .unwrap();

            let token = resp.json::<Tokens>().await.unwrap();
            log::info!("tokens : {:?}", token);

            let message = Request::get("/auth/welcome")
                .header(
                    "Authorization",
                    format!("Bearer {}", token.access_token).as_str(),
                )
                .send()
                .await
                .unwrap();

            log::info!("message : {:?}", message.text().await.unwrap());

            LocalStorage::set("token", token).unwrap();

            location.replace("game").unwrap();
        });
    }

    html! {
        <div>
            <h1>{ "Hello World" }</h1>
            <p>
                <a href="/auth/login">{"login"}</a>
            </p>
            <p>
                <a href="/event/hello/">{"clic and add your name"}</a>
            </p>
            <hr/>
            <WelcomeComp />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
