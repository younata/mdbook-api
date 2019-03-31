extern crate mdbook;
extern crate mdbook_api;

use mdbook::renderer::RenderContext;
use std::io;

fn main() {
    let mut stdin = io::stdin();
    let context = RenderContext::from_json(&mut stdin).expect("Unable to create RenderContext.");

    mdbook_api::generate(&context).expect("Unable to generate api");
}