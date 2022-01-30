use yew::{function_component, html,Properties};


#[derive(Properties, PartialEq)]
pub struct EllipseProps{
    pub center: usize,
    pub width: usize,
    pub length: usize,
    pub color : String,
    pub rotation: Option<isize>,
}

#[function_component(Ellipse)]
pub fn ellipse(p : &EllipseProps) -> Html {

    let center = format!("{}", p.center);
    let width = format!("{}", p.width);
    let length = format!("{}", p.length);

    if p.rotation.is_some() {

        let transform = format!("rotate({},{},{})",p.rotation.unwrap(),p.center, p.center );

        html! {
            <ellipse cx={center.clone()} cy={center} rx={width} ry={length} stroke={p.color.clone()} stroke-width="1" fill="none"
                transform={transform}
            />
                }
    } else {
        html! {
            <ellipse cx={center.clone()} cy={center} rx={width} ry={length} stroke={p.color.clone()} stroke-width="1" fill="none"/>
        }
    }



}