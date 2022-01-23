use enum_iterator::IntoEnumIterator;
use yew::{Component, Context, Html, Properties};

use yew::prelude::*;

#[derive(Debug, PartialEq, IntoEnumIterator, Copy, Clone)]
pub enum Theme {
    Dark,
    Light,
}

impl Into<Classes> for Theme {
    fn into(self) -> Classes {
        match self {
            Theme::Dark => Classes::from("theme-dark"),
            Theme::Light => Classes::from("theme-light"),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub theme: Theme,
    pub on_theme_change: Callback<Theme>,
}

pub struct Header {
    pub theme: Theme,
    pub on_theme_change: Callback<Theme>,
}

pub enum Message {
    ChangeTheme(Theme),
}

impl Component for Header {
    type Message = Message;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();
        Header {
            theme: props.theme,
            on_theme_change: props.on_theme_change,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::ChangeTheme(theme) => {
                self.on_theme_change.emit(theme);
            }
        };
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let props = ctx.props().clone();
        if props.theme != self.theme {
            self.theme = props.theme;
            return true;
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="header">
                {"select theme"}
                {
                    Theme::into_enum_iter().map(| theme | {

                        if theme == self.theme{
                            html!{<div>{
                                format!("Choose {:?}!", theme)
                            }</div>}
                        }else{

                            let onclick = ctx.link().callback(move |_| Message::ChangeTheme(theme));

                            html!{<button {onclick}>{
                                format!("Choose {:?}!", theme)
                            }</button>}
                        }
                    }).collect::<Html>()
                }
            </div>
        }
    }
}
