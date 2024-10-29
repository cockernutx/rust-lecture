use dioxus::prelude::*;
#[derive(PartialEq, Clone, Props)]
pub struct ParadigmProps {
    title: String,
    children: Element,
}
#[component]
pub fn Paradigm(props: ParadigmProps) -> Element {
    rsx! {
        div { class: "mt-2",
            div { class: "collapse bg-base-200",
                input { r#type: "checkbox" }
                div { class: "collapse-title text-xl font-figtree font-medium", "{props.title}" }
                div { class: "collapse-content", {props.children} }
            }
        }
    }
}
