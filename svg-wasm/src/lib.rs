mod chart;
mod utils;

use chart::ChartBuilder;
use wasm_bindgen::prelude::*;

use esvg::page::{Borders, Page};
use esvg::{create_document, Element};
use polygonical::point::Point;

#[wasm_bindgen]
pub fn make_chart(msg: String, width: i32, height: i32, dpi: i32) -> String {
    utils::set_panic_hook();
    let page = Page {
        dpi,
        height,
        width,
        borders: Borders::even(0., dpi),
    };
    let mut doc = create_document(&page);

    let chart = ChartBuilder::new().append_node().build();
    let a = utils::construct_arrow(utils::Direction::Up);

    let mut group = Element::new("g");
    group.set("class", "foo");

    let mut circle = esvg::shapes::circle(page.center(), 50);
    let mut rect = esvg::shapes::rectangle(
        page.center_top().translate(&Point { x: 0., y: 50. }),
        50.,
        50.,
    );
    let mut ellipse = esvg::shapes::ellipse(Point { x: 300., y: 300. }, 30., 100.);
    ellipse.add_style("fill", "green");
    circle.add_style("stroke", "red");
    rect.add_style("stroke", "#0323f3");
    rect.add_style("fill", "#00000000");
    rect.set("transform", "translate(-25,-25)");
    let mut text = esvg::text::create_text(
        msg.clone(),
        utils::pos2point(&circle),
        &esvg::text::create_text_style("sans-serif", 32, "normal", 0.3, "black", "", 1.),
    );
    text.set("text-anchor", "middle");
    group.add(&circle);
    group.add(&text);
    let mut group2 = Element::new("g");
    group2.set("class", "foo1");
    let mut text = esvg::text::create_text(
        msg.clone(),
        utils::pos2point(&rect),
        &esvg::text::create_text_style("sans-serif", 32, "normal", 0.3, "white", "black", 1.),
    );
    text.set("text-anchor", "middle");
    group2.add(&rect);
    group2.add(&text);

    doc.add(&ellipse);
    doc.add(&group);
    doc.add(&group2);
    doc.add(&a);
    let mut result = doc.to_pretty_string();
    result = result.split('\n').skip(2).collect();
    result
}
