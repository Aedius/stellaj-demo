use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::window::{WindowHandler, WindowHelper, WindowStartupInfo};
use speedy2d::{Graphics2D, WebCanvas};
use yew::{Component, Context, Html};

use crate::Theme;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct MapProps {
    pub theme: Theme,
}

pub struct MapHtml {
    pub theme: Theme,
    canvas: Option<WebCanvas<MapEvent>>,
}

struct MapHandler {
    size: Vector2<f32>,
    bg_color: Color,
    color: Color,
}

struct MapEvent {
    theme: Theme,
}

impl WindowHandler<MapEvent> for MapHandler {
    fn on_start(&mut self, _helper: &mut WindowHelper<MapEvent>, info: WindowStartupInfo) {
        self.size = Vector2::new(
            info.viewport_size_pixels().x as f32,
            info.viewport_size_pixels().y as f32,
        );
    }

    fn on_user_event(&mut self, helper: &mut WindowHelper<MapEvent>, user_event: MapEvent) {
        match user_event.theme {
            Theme::Dark => {
                self.bg_color = Color::GRAY;
                self.color = Color::BLUE;
            }
            Theme::Light => {
                self.bg_color = Color::YELLOW;
                self.color = Color::GREEN;
            }
        }
        helper.request_redraw()
    }

    fn on_resize(&mut self, helper: &mut WindowHelper<MapEvent>, size_pixels: Vector2<u32>) {
        self.size = Vector2::new(size_pixels.x as f32, size_pixels.y as f32);

        helper.request_redraw();
    }

    fn on_draw(&mut self, _helper: &mut WindowHelper<MapEvent>, g: &mut Graphics2D) {
        g.clear_screen(Color::GRAY);
        let pos = Vector2::new(self.size.x / 2.0, self.size.y / 2.0);
        g.draw_circle(pos, 50.0, Color::BLUE);
    }
}

impl Component for MapHtml {
    type Message = ();
    type Properties = MapProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();

        MapHtml {
            theme: props.theme,
            canvas: None,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let props = ctx.props().clone();
        if props.theme != self.theme {
            self.theme = props.theme;
        }
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas id="my_canvas"></canvas>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        let handler = MapHandler {
            size: Vector2::new(0.0, 0.0),
            bg_color: Color::GRAY,
            color: Color::BLUE,
        };

        let mut canvas = WebCanvas::new_for_id_with_user_events("my_canvas", handler).unwrap();
        canvas.unregister_when_dropped();
        self.canvas = Some(canvas);
    }
}
