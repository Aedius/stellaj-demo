mod planete;

use rand::Rng;
use rand_core::SeedableRng;
use wyhash::WyRng;

use yew::{Component, Context, Html};

use crate::map::planete::Planet;

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

fn generate_seed(x: u32, y: u32) -> u64 {
    ((x & 0xFFFFFF) << 16 | (y & 0xFFFF)) as u64
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
            Theme::Dark => "white",
            Theme::Light => "black",
        };

        let viewbow = format!("0 0 {} {}", self.size, self.size);
        let center = format!("{}", self.size / 2);

        let mut stars = Vec::new();

        for x in (0..self.size).step_by(5) {
            for y in (0..self.size).step_by(5) {
                let mut rng = WyRng::seed_from_u64(generate_seed(x as u32, y as u32));
                if rng.gen_ratio(1, 50) {
                    stars.push(StarProps {
                        x: x + rng.gen_range(0..5),
                        y: y + rng.gen_range(0..5),
                        r: rng.gen_range(1..5),
                        color: rng.gen_range(120..245),
                    });
                }
            }
        }

        html! {
            <div class="map_container">
            <svg id="my_map" viewBox={viewbow}>
                {
                    stars.into_iter().map(|star| {
                        let x = format!{"{}", star.x};
                        let y = format!{"{}", star.y};
                        let r = format!("{}", star.r);
                        let rgb = format!("rgb({}, {}, {})", star.color, star.color, star.color);
                        html!{<circle cx={x} cy={y} r={r} fill={rgb} />}
                    }).collect::<Html>()
                }

                <circle cx={center.clone()} cy={center.clone()} r="20" fill="yellow"/>
                <Planet center={self.size/2} width={200} length={50} color={color} />
                <Planet center={self.size/2} width={300} length={75} color={color} rotation={-35}/>
                <Planet center={self.size/2} width={400} length={100} color={color} />
                <Planet center={self.size/2} width={800} length={200} color={color} rotation={10} />
                <Planet center={self.size/2} width={1200} length={300} color={color} />
            </svg>
            </div>
        }
    }
}

pub struct StarProps {
    pub x: usize,
    pub y: usize,
    pub r: usize,
    pub color: usize,
}
