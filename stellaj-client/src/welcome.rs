use gloo_events::EventListener;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Event, EventSource, MessageEvent};
use yew::{Component, Context, Html};

use yew::prelude::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Wc {
    name: String,
}

pub enum WelcomeMsg {
    EsReady(Result<Wc, serde_json::Error>),
}

pub struct WelcomeComp {
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
            member: Vec::new(),
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
            <p>{"Last connected"}</p>
            {
                self.member.clone().into_iter().map(|name| {
                    html!{<div key={name.clone()}>{ format!("Welcome {}!",name.clone()) }</div>}
                }).collect::<Html>()
            }
            </div>
        }
    }
}
