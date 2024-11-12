use dioxus::document::eval;
use dioxus::prelude::*;
use gloo::utils::window;
use crate::get_asset::{get_asset, GetAssetError};
use crate::Route;

// Only here to validate the example when compiling
pub(self) mod example;

#[component]
pub fn TypeSystem() -> Element {
    const CODE_EXAMPLE: Asset = asset!("/src/pages/type_system/example.rs");

    let fetch_example: Resource<Result<String, GetAssetError>> =
        use_resource(move || async move {
    
            let text = get_asset(&CODE_EXAMPLE).await?;

            Ok(text)
        });

    use_memo(move || {
        if let Some(Ok(_)) = &*fetch_example.read() {

            eval(
                r#"
                    setTimeout(() => {hljs.highlightAll();}, 50);
                "#,
            );
        }
    });

    let slide = move |id: &str| {
        let ev = eval(r#"
            let id = await dioxus.recv();
            console.log(id);
            document.getElementById(id).scrollIntoView();
        "#);
        ev.send(id).unwrap();
    };

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
                                src: asset!("/src/pages/type_system/primitive_types.webp"),
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
                                src: asset!("/src/pages/type_system/references_and_borrows.webp"),
                                class: "max-w-sm rounded-lg shadow-2xl",
                            }
                            div {
                                h1 { class: "text-5xl font-bold", "Tipos Primitivos" }
                                p { class: "py-6",
                                    "Rust tem um sistema de referência que impede o uso incorreto de dados. Isso é feito através das regras de empréstimo (borrowing):"
                                }
                                ul { class: "list-disc ",
                                    li { "Referências podem ser imutáveis (&T) ou mutáveis (&mut T)." }
                                    li {
                                        "Apenas uma referência mutável pode existir por vez, mas múltiplas referências imutáveis são permitidas, garantindo segurança contra condições de corrida."
                                    }
                                    li {
                                        "O verificador de empréstimos (borrow checker) do Rust assegura que essas regras sejam seguidas durante a compilação, prevenindo acessos simultâneos ou inválidos a dados."
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "carousel-item w-full", id: "item3",
                    div { class: "hero bg-base-200  rounded-box",
                        div { class: "hero-content flex-col lg:flex-row",
                            img {
                                src: asset!("/src/pages/type_system/enums_and_pattern_matching.webp"),
                                class: "max-w-sm rounded-lg shadow-2xl",
                            }
                            div {
                                h1 { class: "text-5xl font-bold", "Enums e Pattern Matching" }
                                p { class: "py-6",
                                    "Rust permite criar Enums que podem representar diferentes estados e simplificam a modelagem de dados complexos. Um exemplo comum é o uso do Option e do Result, que são enums para valores opcionais e para tratamento de erros. Junto com o pattern matching, esses enums permitem construir código seguro e sem a necessidade de null."
                                }
                            }
                        }
                    }
                }
                div { class: "carousel-item w-full", id: "item4",
                    div { class: "hero bg-base-200  rounded-box",
                        div { class: "hero-content flex-col lg:flex-row",
                            img {
                                src: asset!("/src/pages/type_system/traits.webp"),
                                class: "max-w-sm rounded-lg shadow-2xl",
                            }
                            div {
                                h1 { class: "text-5xl font-bold", "Traits" }
                                p { class: "py-6",
                                    "Rust usa traits para definir comportamento comum. Uma trait é como uma interface em outras linguagens, mas com diferenças:"
                                }
                                ul { class: "list-disc",
                                    li {
                                        "Elas definem um conjunto de métodos que os tipos podem implementar."
                                    }
                                    li {
                                        "Isso permite polimorfismo no Rust e substitui o uso de herança, promovendo a composição ao invés da herança."
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "flex w-full justify-center gap-2 py-2",
                button { onclick: move |_| slide("item1"), class: "btn btn-xs", "1" }
                button { onclick: move |_| slide("item2"), class: "btn btn-xs", "2" }
                button { onclick: move |_| slide("item3"), class: "btn btn-xs", "3" }
                button { onclick: move |_| slide("item4"), class: "btn btn-xs", "4" }
            }
        }
        div { class: "mt-12",
            h1 { class: "text-center text-5xl font-figtree font-extrabold", "Porque esse sistema?" }
        }
        div { class: "mt-6",
            p { class: " p-4 text-xl font-semibold",
                "O Rust tem um sistema de tipos extremamente complexo, isto se é devido a seu modelo e paradigma de validação de lifetimes e borrow checking. Rust foi criado para facilitar o gerenciamento de memoria e impedir bugs comuns que aconteciam com C e C++, você pode ler mais sobre isso lendo a nossa secção sobre a "
                Link { to: Route::RustHistory {}, class: "link link-accent", "historia da linguagem" }
                "."
            }
        }
    }
}
