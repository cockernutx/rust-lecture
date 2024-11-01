use dioxus::document::eval;
use dioxus::prelude::*;
use gloo::utils::window;

// Only here to validate the example when compiling
pub(self) mod example;

#[component]
pub fn TypeSystem() -> Element {
    const CODE_EXAMPLE: &str = asset!("./src/pages/type_system/example.rs");

    let fetch_example: Resource<Result<String, reqwest::Error>> =
        use_resource(move || async move {
            let url = format!("{}{}", window().location().origin().unwrap(), CODE_EXAMPLE);
            let req = reqwest::get(url).await?;
            let text = req.text().await?;

            Ok(text)
        });

    use_memo(move || {
        if let Some(Ok(_)) = &*fetch_example.read() {
            tracing::debug!("highlighting code");
            eval(
                r#"
                    setTimeout(() => {hljs.highlightAll();}, 50);
                "#,
            );
        }
    });

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/hybrid.min.css",
        }

        script { src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js" }
        script { src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/rust.min.js" }
        h1 { class: "text-6xl font-extrabold text-center mt-6",
            p { "OS TIPOS DO " }
            div { class: "inline-grid",
                span { class: "pointer-events-none col-start-1 row-start-1 bg-[linear-gradient(90deg,theme(colors.error)_0%,theme(colors.secondary)_9%,theme(colors.secondary)_42%,theme(colors.primary)_47%,theme(colors.accent)_100%)] bg-clip-text blur-xl [-webkit-text-fill-color:transparent] [transform:translate3d(0,0,0)] before:content-[attr(data-text)] [@supports(color:oklch(0%_0_0))]:bg-[linear-gradient(90deg,oklch(var(--s))_4%,color-mix(in_oklch,oklch(var(--s)),oklch(var(--er)))_22%,oklch(var(--p))_45%,color-mix(in_oklch,oklch(var(--p)),oklch(var(--a)))_67%,oklch(var(--a))_100.2%)]",
                    "RUST"
                }
                span { class: "[&::selection]:text-base-content relative col-start-1 row-start-1 bg-[linear-gradient(90deg,theme(colors.error)_0%,theme(colors.secondary)_9%,theme(colors.secondary)_42%,theme(colors.primary)_47%,theme(colors.accent)_100%)] bg-clip-text [-webkit-text-fill-color:transparent] [&::selection]:bg-blue-700/20 [@supports(color:oklch(0%_0_0))]:bg-[linear-gradient(90deg,oklch(var(--s))_4%,color-mix(in_oklch,oklch(var(--s)),oklch(var(--er)))_22%,oklch(var(--p))_45%,color-mix(in_oklch,oklch(var(--p)),oklch(var(--a)))_67%,oklch(var(--a))_100.2%)] font-figtree",
                    "RUST"
                }
            }
        }
        div { class: "p-2 mt-8 ",
            div { class: "mockup-code bg-[#1d1f21]",
                match &*fetch_example.read() {
                    Some(Ok(example)) => rsx! {
                        pre {
                            span {
                                code { "{example}" }
                            }
                        }
                    },
                    Some(Err(why)) => rsx! {
                        div { "{why}" }
                    },
                    None => rsx! {
                        div { class: "flex w-full flex-col gap-2 p-2",
                            div { class: "skeleton h-4 w-72" }
                            div { class: "skeleton h-4 w-64" }
                            div { class: "skeleton h-4 w-36" }
                            div { class: "skeleton h-4 w-full" }
                        }
                    },
                }
            }
        }
        div { class: "relative -mt-12 ml-2 flex justify-end",
            div { class: "badge badge-outline badge-neutral",
                "O exemplo representa uma implementação de codigo em Rust"
            }
        }
        div { class: "mt-12",
            h1 { class: "text-center text-5xl font-figtree font-extrabold",
                "O que são os tipos do Rust?"
            }
        }
        div { class: "mt-6 px-6 rounded-box",
            div { class: "carousel w-full",
                div { class: "carousel-item w-full", id: "item1",
                    div { class: "hero bg-base-200  rounded-box",
                        div { class: "hero-content flex-col lg:flex-row",
                            img {
                                src: asset!("./src/pages/type_system/primitive_types.webp"),
                                class: "max-w-sm rounded-lg shadow-2xl",
                            }
                            div {
                                h1 { class: "text-5xl font-bold", "Tipos Primitivos" }
                                p { class: "py-6",
                                    "Rust oferece vários tipos primitivos, que incluem: "
                                }
                                ul { class: "list-disc ",
                                    li {
                                        "Numéricos: i8, i16, i32, i64, i128, isize para inteiros com sinal; u8, u16, u32, u64, u128, usize para inteiros sem sinal; e f32, f64 para ponto flutuante."
                                    }
                                    li { "Booleanos: bool, com valores true ou false." }
                                    li {
                                        "Caracteres: char, que armazena um caractere Unicode de 4 bytes."
                                    }
                                    li {
                                        "Tuplas: tipos heterogêneos, como (i32, f64, char), que podem armazenar valores de diferentes tipos."
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "carousel-item w-full", id: "item2",
                    div { class: "hero bg-base-200  rounded-box",
                        div { class: "hero-content flex-col lg:flex-row",
                            img {
                                src: asset!("./src/pages/type_system/references_and_borrows.webp"),
                                class: "max-w-sm rounded-lg shadow-2xl",
                            }
                            div {
                                h1 { class: "text-5xl font-bold", "Tipos Primitivos" }
                                p { class: "py-6",
                                    "Rust tem um sistema de referência que impede o uso incorreto de dados. Isso é feito através das regras de empréstimo (borrowing):"
                                }
                                ul { class: "list-disc ",
                                    li {
                                        "Referências podem ser imutáveis (&T) ou mutáveis (&mut T)."
                                    }
                                    li { "Apenas uma referência mutável pode existir por vez, mas múltiplas referências imutáveis são permitidas, garantindo segurança contra condições de corrida." }
                                    li {
                                        "O verificador de empréstimos (borrow checker) do Rust assegura que essas regras sejam seguidas durante a compilação, prevenindo acessos simultâneos ou inválidos a dados."
                                    }
                          
                                }
                            }
                        }
                    }
                }
                div { class: "carousel-item w-full", id: "item3",
                    img {
                        src: "https://img.daisyui.com/images/stock/photo-1414694762283-acccc27bca85.webp",
                        class: "w-full",
                    }
                }
                div { class: "carousel-item w-full", id: "item4",
                    img {
                        src: "https://img.daisyui.com/images/stock/photo-1665553365602-b2fb8e5d1707.webp",
                        class: "w-full",
                    }
                }
            }
            div { class: "flex w-full justify-center gap-2 py-2",
                a { href: "#item1", class: "btn btn-xs", "1" }
                a { href: "#item2", class: "btn btn-xs", "2" }
                a { href: "#item3", class: "btn btn-xs", "3" }
                a { href: "#item4", class: "btn btn-xs", "4" }
            }
        }
    }
}
