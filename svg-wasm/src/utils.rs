use esvg::Element;
use polygonical::point::Point;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn pos2point(e: &Element) -> Point {
    let x = if let Some(i) = e.get("cx") {
        i.parse().unwrap()
    } else if let Some(i) = e.get("x") {
        i.parse().unwrap()
    } else {
        panic!("could not find location")
    };
    let y = if let Some(i) = e.get("cy") {
        i.parse().unwrap()
    } else if let Some(i) = e.get("y") {
        i.parse().unwrap()
    } else {
        panic!("could not find location")
    };
    Point { x, y }
}

pub fn new_group(class: &str) -> Element {
    let mut group = Element::new("g");
    group.set("class", class);
    group
}

pub fn rotate(e: &mut Element, angle: u16) {
    let transform = e.get("transform").unwrap_or(String::new());
    e.set("transform", format!("{transform} rotate({angle})"));
}

pub fn construct_arrow(dir: Direction) -> Element {
    let mut head_r = esvg::shapes::rounded_rectangle(Point { x: 333., y: 333. }, 5., 60., 3.);
    rotate(&mut head_r, 45);
    let mut head_l = esvg::shapes::rounded_rectangle(Point { x: 333., y: 333. }, 5., 60., 3.);
    let mut body = esvg::shapes::rounded_rectangle(Point { x: 333., y: 333. }, 5., 60., 3.);
    let mut group = new_group("arrow");
    group.add(&head_r);
    group.add(&head_l);
    group.add(&body);
    group
}
