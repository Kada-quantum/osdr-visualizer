use esvg::Element;

use crate::utils;

pub struct ChartBuilder {
    e: Element,
}

impl ChartBuilder {
    pub fn new() -> Self {
        Self {
            e: utils::new_group("flowchart"),
        }
    }
    pub fn append_node(&mut self) -> &mut Self {
        self
    }
    pub fn build(&mut self) -> Element {
        self.e.clone()
    }
}
