use crate::Route;
use dioxus::prelude::*;
pub mod graph_example;
#[component]
pub fn Examples() -> Element {
    rsx! {
        document::Title { "Exemplos" }
        div { class: "p-2",
            Link { to: Route::GraphExample {}, "Graph" }
        }
    }
}
