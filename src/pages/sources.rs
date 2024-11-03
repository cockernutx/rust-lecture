use dioxus::prelude::*;
use gloo::utils::window;
#[component]
pub fn Sources() -> Element {
    const SOURCES: Asset = asset!("./src/pages/sources/sources.json");
    let sources_fetch = use_resource(move || async move {
        let Ok(base_path) = window().location().origin() else {
            return Err("could not get base path".to_string());
        };
        let path = format!("{base_path}{SOURCES}");
        let req = match reqwest::get(path).await {
            Ok(r) => r,
            Err(why) => {
                tracing::error!("{:?}", why);
                return Err(why.to_string());
            }
        };
        let vec: Vec<String> = match req.json().await {
            Ok(j) => j,
            Err(why) => return Err(why.to_string()),
        };
        Ok(vec)
    });
    rsx! {
        document::Title { "Fontes" }
        div { class: "p-2",
            match &sources_fetch() {
                Some(Ok(data)) => rsx! {
                    ul { class: "list-disc",
                        for source in data {
                            li {
                                a { href: "{source}", target: "_blank", "{source}" }
                            }
                        }
                    }
                },
                Some(Err(why)) => rsx! {
                    div { role: "alert", class: "alert alert-error",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            "viewBox": "0 0 24 24",
                            fill: "none",
                            class: "h-6 w-6 shrink-0 stroke-current",
                            path {
                                d: "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
                                "stroke-linejoin": "round",
                                "stroke-linecap": "round",
                                "stroke-width": "2",
                            }
                        }
                        span { "Error: {why}" }
                    }
                },
                None => rsx! {
                    span { class: "loading loading-spinner text-primary" }
                },
            }
        }
    }
}
