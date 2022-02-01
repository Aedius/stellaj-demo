mod header;
mod map;
mod welcome;

use std::fmt::Debug;

use yew::prelude::*;

use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::html;

use header::{Header, Theme};
use map::MapHtml;
use welcome::Welcome;

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

pub struct App {
    theme: Theme,
}

pub enum AppMessage {
    ChangeTheme(Theme),
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
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

        App { theme: Theme::Dark }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::ChangeTheme(theme) => {
                self.theme = theme;
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_theme_change = ctx.link().callback(|theme| AppMessage::ChangeTheme(theme));

        log::info!("theme : {:?}", self.theme);

        let theme = self.theme;

        html! {
            <div class={self.theme}>
                <Header theme={self.theme} {on_theme_change}/>
                <div class="body">
                    <p>{ "Hello World" }</p>
                    <p>
                        <a href="/auth/login">{"login"}</a>
                    </p>
                    <p>
                        <a href="/event/hello/">{"clic and add your name"}</a>
                    </p>
                    <hr/>
                    <Welcome />
                    <MapHtml {theme} size=2000/>
                </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
