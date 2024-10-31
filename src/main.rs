#![allow(non_snake_case)]
use dioxus::prelude::*;
use layouts::nav_bar::NavBar;
use manganis::CssAssetBuilder;
use pages::{
    about_us::AboutUs,
    borrow_checker::BorrowChecker,
    conclusion::Conclusion,
    examples::{graph_example::GraphExample, Examples},
    home::Home,
    license::License,
    rust_history::RustHistory,
    sources::Sources,
    type_system::TypeSystem,
};
mod components;
mod layouts;
mod pages;
#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home {},
    #[route("/borrow_checker")]
    BorrowChecker {},
    #[route("/type_system")]
    TypeSystem {},
    #[route("/rust_history")]
    RustHistory {},
    #[route("/about_us")]
    AboutUs {},
    #[route("/sources")]
    Sources {},
    #[route("/license")]
    License {},
    #[route("/conclusion")]
    Conclusion {},
    #[nest("/examples")]
    #[route("/")]
    Examples {},
    #[route("/graph")]
    GraphExample {},
}
fn main() {
    dioxus_logger::init(tracing::Level::DEBUG).expect("failed to init logger");
    tracing::info!("starting application on http://localhost:8080");
    launch(App);
}
fn App() -> Element {
    const TAILWIND_CSS: &str = asset!(file("./target/tailwind.css"));
    rsx! {
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link {
            crossorigin: "false",
            href: "https://fonts.gstatic.com",
            rel: "preconnect",
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Figtree:ital,wght@0,300..900;1,300..900&display=swap",
        }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link {
            rel: "icon",
            href: "https://www.rust-lang.org/logos/rust-logo-blk.svg",
            r#type: "image/svg",
        }
        document::Title { "Projeto sobre a linguagem Rust" }
        Router::<Route> {}
    }
}
