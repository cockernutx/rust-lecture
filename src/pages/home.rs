use crate::Route;
use dioxus::prelude::*;
#[derive(PartialEq, Clone, Props)]
pub struct InfoParagraphProps {
    title: String,
    line_color: String,
    children: Element,
}
#[component]
fn InfoParagraph(props: InfoParagraphProps) -> Element {
    rsx! {
        div {
            h2 { class: "text-5xl font-title font-extrabold", "{props.title}" }
            div { class: "h-1 rounded-sm {props.line_color} " }
            div { class: "mt-3 text-2xl", {props.children} }
        }
    }
}
#[component]
pub fn Home() -> Element {
    const TITLE: &str = "A linguagem de programação Rust";
    rsx! {
        document::Title { "{TITLE}" }
        div { class: "mt-6 font-figtree p-2",
            div { class: "flex flex-col items-center justify-center",
                h1 { class: "text-center text-8xl font-black ",
                    p { "A linguagem  " }
                    p { "de programação" }
                    div { class: "inline-grid",
                        span { class: "pointer-events-none col-start-1 row-start-1 bg-[linear-gradient(90deg,theme(colors.error)_0%,theme(colors.secondary)_9%,theme(colors.secondary)_42%,theme(colors.primary)_47%,theme(colors.accent)_100%)] bg-clip-text blur-xl [-webkit-text-fill-color:transparent] [transform:translate3d(0,0,0)] before:content-[attr(data-text)] [@supports(color:oklch(0%_0_0))]:bg-[linear-gradient(90deg,oklch(var(--s))_4%,color-mix(in_oklch,oklch(var(--s)),oklch(var(--er)))_22%,oklch(var(--p))_45%,color-mix(in_oklch,oklch(var(--p)),oklch(var(--a)))_67%,oklch(var(--a))_100.2%)]",
                            "RUST"
                        }
                        span { class: "[&::selection]:text-base-content relative col-start-1 row-start-1 bg-[linear-gradient(90deg,theme(colors.error)_0%,theme(colors.secondary)_9%,theme(colors.secondary)_42%,theme(colors.primary)_47%,theme(colors.accent)_100%)] bg-clip-text [-webkit-text-fill-color:transparent] [&::selection]:bg-blue-700/20 [@supports(color:oklch(0%_0_0))]:bg-[linear-gradient(90deg,oklch(var(--s))_4%,color-mix(in_oklch,oklch(var(--s)),oklch(var(--er)))_22%,oklch(var(--p))_45%,color-mix(in_oklch,oklch(var(--p)),oklch(var(--a)))_67%,oklch(var(--a))_100.2%)] font-figtree",
                            "RUST"
                        }
                    }
                }
                div { class: "mt-12",
                    div { class: "inline-flex w-full flex-col items-stretch justify-center gap-2 px-4 md:flex-row xl:justify-start xl:px-0",
                        Link { to: Route::AboutUs {}, class: "btn btn-accent", "Sobre nós" }
                    }
                }
            }
            section { class: "mt-32" }
            InfoParagraph { title: "Rust", line_color: "bg-green-300",
                "Rust é uma linguagem de programação focada em desempenho, segurança de memória e concorrência, desenvolvida pela Mozilla Research. Ela é projetada para combinar a eficiência e o controle de baixo nível típicos de linguagens como C e C++, mas com garantias de segurança contra erros comuns de memória."
            }
            section { class: "mt-12" }
            InfoParagraph { title: "Por que Rust?", line_color: "bg-purple-300",
                div { class: "grid grid-cols-3 gap-6 mt-2",
                    div {
                        p { class: "text-3xl font-extrabold", "Desempenho" }
                        p { class: "mt-4 ",
                            "Rust é extremamente rápido e gerencia memória eficientemente: sem runtime ou garbage collector, podendo potencializar a performance de serviços críticos, rodar em sistemas embarcados, e facilmente integrar-se a outras linguagens. "
                        }
                    }
                    div {
                        p { class: "text-3xl font-extrabold", "Confiabilidade" }
                        p { class: "mt-4 ",
                            "O rico sistema de tipos de Rust e seu modelo de ownership garantem segurança de memória e segurança de concorrência — e permite que você elimine muitas categorias de erros durante a compilação. "
                        }
                    }
                    div {
                        p { class: "text-3xl font-extrabold", "Produtividade" }
                        p { class: "mt-4 ",
                            "Rust possui uma ótima documentação, um compilador amigável com mensagens de erros úteis, e ferramental de primeira qualidade — uma ferramenta integrada de compilação e gerenciamento de pacotes, suporte inteligente para múltiplos editores com autocompleção e inspeções de tipos, um formatador automático, e muito mais. "
                        }
                    }
                }
            }
            div { class: "h-1 rounded-sm bg-rust mt-12" }
            div { class: "grid grid-cols-2 gap-8 mt-10",
                div {
                    p { class: "text-6xl font-extrabold", "Aprenda mais!" }
                    p { class: "mt-6  text-xl",
                        "Descubra o poder do Rust e aprenda mais sobre essa incrível linguagem! Combinando segurança de memória, alta performance e um sistema de gerenciamento de recursos único, Rust oferece uma experiência de desenvolvimento que permite criar software robusto, seguro e incrivelmente eficiente."
                    }
                }
                div { class: "flex items-center justify-center ",
                    Link {
                        to: Route::BorrowChecker {},
                        class: "btn btn-secondary h-32 w-64 rounded-full",
                        div {
                            p { class: "text-3xl font-extrabold ", "COMECE AGORA" }
                        }
                    }
                }
            }
        }
    }
}
