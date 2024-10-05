use esvg::Element;
use polygonical::point::Point;

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
