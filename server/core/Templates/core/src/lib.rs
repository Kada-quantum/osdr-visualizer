use esvg::page::{Borders, Page};
use esvg::{create_document, save, Element};
use polygonical::point::Point;

fn pos2point(e: &Element) -> Point {
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

pub fn make_svg(msg: String) -> String {
    let page = Page {
        dpi: 91,
        height: 500,
        width: 500,
        borders: Borders::default(91),
    };
    let mut doc = create_document(&page);

    let mut group = Element::new("g");
    group.set("class", "foo");

    let mut circle = esvg::shapes::circle(page.center(), 50);
    let mut rect = esvg::shapes::rectangle(page.center_top(), 50., 50.);
    let mut ellipse = esvg::shapes::ellipse(Point { x: 300., y: 300. }, 30., 100.);
    ellipse.add_style("fill", "green");
    circle.add_style("stroke", "red");
    rect.add_style("stroke", "#0323f3");
    rect.add_style("fill", "#00000000");
    rect.set("transform", "translate(-25,-25)");
    println!("{:?}", circle.style_map().unwrap());
    let mut text = esvg::text::create_text(
        msg.clone(),
        pos2point(&circle),
        &esvg::text::create_text_style("sans-serif", 32, "normal", 0.3, "black", "", 1.),
    );
    text.set("text-anchor", "middle");
    group.add(&circle);
    group.add(&text);
    let mut group2 = Element::new("g");
    group2.set("class", "foo1");
    let mut text = esvg::text::create_text(
        msg.clone(),
        pos2point(&rect),
        &esvg::text::create_text_style("sans-serif", 32, "normal", 0.3, "white", "black", 1.),
    );
    text.set("text-anchor", "middle");
    group2.add(&rect);
    group2.add(&text);

    doc.add(&ellipse);
    doc.add(&group);
    doc.add(&group2);
    println!("{doc}");
    save("./image.svg", &doc).expect("Could not write");
    doc.to_pretty_string()
}
