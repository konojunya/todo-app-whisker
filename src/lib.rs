//! A small, local-first Todo app built with Whisker.

mod app;
mod components;
mod model;
mod repository;
mod theme;

use crate::app::AppRoot;
use whisker::prelude::*;
use whisker::runtime::view::Element;

#[whisker::main]
fn app() -> Element {
    render! {
        AppRoot
    }
}
