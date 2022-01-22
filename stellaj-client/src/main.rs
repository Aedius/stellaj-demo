use std::fmt::Debug;

use yew::prelude::*;

use gloo_events::EventListener;
use reqwasm::http::Request;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, Event, EventSource, MessageEvent};
use yew::{html, Component, Context, Html};

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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Wc {
    name: String,
}

enum WelcomeMsg {
    EsReady(Result<Wc, serde_json::Error>),
}

struct WelcomeComp {
    es: EventSource,
    member: Vec<String>,
    _listener: EventListener,
}

impl Component for WelcomeComp {
    type Message = WelcomeMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let es = EventSource::new("/event/greetings")
            .map_err(|js_value: JsValue| {
                let err: js_sys::Error = js_value.dyn_into().unwrap();
                log::error!("error : {:?}", err);
                err
            })
            .unwrap();

        let cb = ctx
            .link()
            .callback(|bufstr: String| WelcomeMsg::EsReady(serde_json::from_str(&bufstr)));
        let listener = EventListener::new(&es, "message", move |event: &Event| {
            let event = event.dyn_ref::<MessageEvent>().unwrap();
            let text = event.data().as_string().unwrap();

            cb.emit(text);
        });

        WelcomeComp {
            es,
            member: vec!["test1".to_string(), "test2".to_string()],
            _listener: listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WelcomeMsg::EsReady(response) => {
                match response {
                    Ok(data_result) => {
                        self.member.push(data_result.name.clone());
                    }
                    Err(e) => {
                        log::error!("{}", e);
                    }
                };
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div id="welcome">
            <p>{ format!("Connection State: {:?}", self.es.ready_state()) }</p>
            {
                self.member.clone().into_iter().map(|name| {
                    html!{<div key={name.clone()}>{ format!("Welcome {}!",name.clone()) }</div>}
                }).collect::<Html>()
            }
            </div>
        }
    }
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
