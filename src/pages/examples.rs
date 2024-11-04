pub mod image_manipulation;

use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Examples() -> Element {
    rsx! {
        document::Title { "Exemplos" }
        ul { class: "menu rounded-box w-56",
            li {
                h2 { class: "menu-title", "Examples" }
                ul {
                    li {
                        Link { to: Route::ImageManipulation {}, "Image manipulation" }
                    }
                }
            }
        }
    }
}
