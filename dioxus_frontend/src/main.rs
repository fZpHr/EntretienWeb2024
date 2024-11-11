#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info};

mod pages;
use pages::home::Home;
use pages::faq::Faq;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/faq")]
    Faq {},
}


fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}


fn App() -> Element {
    let css = include_str!("../tailwind.css");
    rsx! {
        style { "{css}" }
        Router::<Route> {}
    }
}
