use std::{fmt::{Debug}};

use yew::prelude::*;


use gloo_events::EventListener;
use wasm_bindgen::{JsCast, JsValue};

use yew::{html, Component, Context, Html};
use web_sys::{Event, EventSource, MessageEvent};

#[macro_use]
extern crate serde_derive;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Wc {
    name: String
}

enum WelcomeMsg{
    EsReady(Result<Wc, serde_json::Error>)
}

struct WelcomeComp{
    es: EventSource,
    member: Vec<String>,
    _listener: EventListener,
}

impl Component for WelcomeComp{
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

        let cb = ctx.link().callback(|bufstr: String| {
            log::info!("callback");
            WelcomeMsg::EsReady(serde_json::from_str(&bufstr))
        });
        let listener = EventListener::new(&es, "message", move |event: &Event| {
            log::info!("event received");
            let event = event.dyn_ref::<MessageEvent>().unwrap();
            let text = event.data().as_string().unwrap();

            cb.emit(text);
        });

        log::info!("load");

        WelcomeComp{
            es,
            member: vec!("test1".to_string(),"test2".to_string()),
            _listener: listener
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {

        log::info!("update");

        match msg {
            WelcomeMsg::EsReady(response) => {
                match response {
                    Ok(data_result) => {
                        log::info!("passe");
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