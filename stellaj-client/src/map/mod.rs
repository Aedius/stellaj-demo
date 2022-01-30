mod ellipse;

use yew::{Component, Context, Html};

use crate::map::ellipse::Ellipse;

use crate::Theme;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct MapProps {
    pub theme: Theme,
    pub size: usize,
}

pub struct MapHtml {
    pub theme: Theme,
    pub size: usize,
}


impl Component for MapHtml {
    type Message = ();
    type Properties = MapProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();

        MapHtml {
            theme: props.theme,
            size: props.size,
        }
    }


    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let props = ctx.props().clone();
        if props.theme != self.theme {
            self.theme = props.theme;
            return true;
        }
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        let color = match self.theme {
            Theme::Dark => {"white"}
            Theme::Light => {"black"}
        };

        let viewbow= format!("0 0 {} {}", self.size, self.size);
        let center = format!("{}",self.size/2);

        html! {
            <svg id="my_map" viewBox={viewbow}>
                <circle cx={center.clone()} cy={center.clone()} r="20" fill="yellow"/>
                <Ellipse center={self.size/2} width={200} length={50} color={color} />
                <Ellipse center={self.size/2} width={300} length={75} color={color} rotation={-35}/>
                <Ellipse center={self.size/2} width={400} length={100} color={color} />
                <Ellipse center={self.size/2} width={800} length={200} color={color} rotation={10} />
                <Ellipse center={self.size/2} width={1200} length={300} color={color} />
            </svg>
        }
    }
}
