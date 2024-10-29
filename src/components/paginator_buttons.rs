use dioxus::prelude::*;
#[derive(PartialEq, Clone, Props)]
pub struct PaginatorButtonsProps {
    next_href: Option<String>,
    previous_href: Option<String>,
}
#[component]
pub fn PaginatorButtons(props: PaginatorButtonsProps) -> Element {
    let cols = if props.previous_href.is_some() && props.next_href.is_some() {
        "grid-cols-2"
    } else {
        "grid-cols-1"
    };
    rsx! {
        div { class: "p-2",
            div { class: "grid {cols} gap-10",
                if let Some(href) = props.previous_href {
                    a { class: "btn btn-primary ", href: "{href}", "<- Anterior" }
                }
                if let Some(href) = props.next_href {
                    a { class: "btn btn-primary ", href: "{href}", "Proximo ->" }
                }
            }
        }
    }
}
