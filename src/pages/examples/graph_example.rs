use dioxus::prelude::*;
mod graph;
#[component]
pub fn GraphExample() -> Element {
    let graph = use_signal(|| graph::Graph::new());
    let mut content = use_signal(|| String::new());
    let mut id = use_signal(|| String::new());
    let add_child = move || {
        graph.write_unchecked().add_node(
            content.read_unchecked().clone(),
            id.read_unchecked().clone(),
        );
    };
    let relate = move |e: Event<FormData>| {
        let formdata = e.data();
        let values = formdata.values();
        let r#in = values.get("in").unwrap().as_value();
        let relation_type = values.get("relation_type").unwrap().as_value();
        let out = values.get("out").unwrap().as_value();
        match graph.write_unchecked().relate(r#in, out, relation_type) {
            Ok(_) => {}
            Err(why) => {
                gloo::dialogs::alert(&format!("could not relate graph: {why}"));
            }
        };
    };
    rsx! {
        document::Title { "Grafo" }
        div { class: "p-2",
            div { class: "collapse bg-base-200",
                input { r#type: "checkbox" }
                div { class: "collapse-title text-xl font-medium", "Inputs" }
                div { class: "collapse-content",
                    form {
                        class: "join join-vertical w-full max-w-x",
                        onsubmit: move |_| add_child(),
                        input {
                            class: "input input-bordered w-full max-w-x join-item",
                            r#type: "text",
                            placeholder: "id",
                            value: "{id}",
                            oninput: move |e| id.set(e.value()),
                            required: true,
                        }
                        textarea {
                            class: "textarea textarea-bordered w-full max-w-x join-item",
                            placeholder: "content",
                            value: "{content}",
                            oninput: move |e| content.set(e.value()),
                            required: true,
                        }
                        button {
                            class: "btn btn-primary w-full max-w-x join-item",
                            r#type: "submit",
                            "Add node"
                        }
                    }
                    form { class: "mt-4", onsubmit: relate,
                        div { class: "join w-[100%]",
                            input {
                                class: "input input-bordered w-full max-w-x join-item",
                                r#type: "text",
                                placeholder: "in",
                                name: "in",
                                required: true,
                            }
                            input {
                                class: "input input-bordered w-full max-w-x join-item",
                                r#type: "text",
                                placeholder: "Relation type",
                                name: "relation_type",
                                required: true,
                            }
                            input {
                                class: "input input-bordered w-full max-w-x join-item",
                                r#type: "text",
                                placeholder: "out",
                                name: "out",
                                required: true,
                            }
                        }
                        button {
                            class: "btn btn-primary w-[100%] mt-2",
                            r#type: "submit",
                            "Relate"
                        }
                    }
                }
            }
            div { class: "mt-4",
                ul {
                    for node in graph().clone().nodes {
                        li { class: "mt-2",
                            div { class: "collapse bg-base-200",
                                input { r#type: "checkbox" }
                                div { class: "collapse-title text-xl font-medium", "{node.id}" }
                                div { class: "collapse-content",
                                    p { "{node.content}" }
                                    if let Ok(relations) = &graph.read().get_edges(node.id.clone()) {
                                        ul { class: "p-2",
                                            for relation in relations {
                                                li { class: "mt-2",
                                                    "{relation.r#in}->{relation.relation_type}->{relation.out}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
