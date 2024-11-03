use dioxus::prelude::*;
#[component]
pub fn RustHistory() -> Element {
    rsx! {
        div { class: "p-2",
            ul { class: "timeline timeline-snap-icon max-md:timeline-compact timeline-vertical",
                li {
                    div { class: "timeline-middle",
                        svg {
                            fill: "currentColor",
                            "viewBox": "0 0 20 20",
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "h-5 w-5",
                            path {
                                d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z",
                                "clip-rule": "evenodd",
                                "fill-rule": "evenodd",
                            }
                        }
                    }
                    div { class: "timeline-start mb-10 md:text-end",
                        time { class: "font-mono italic", "2006" }
                        div { class: "text-lg font-black", "O início de Rust." }
                        "O projeto Rust é iniciado por Graydon Hoare como um projeto pessoal, enquanto ele trabalhava na Mozilla."
                    }
                    hr {}
                }
                li {
                    hr {}
                    div { class: "timeline-middle",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "currentColor",
                            "viewBox": "0 0 20 20",
                            class: "h-5 w-5",
                            path {
                                "clip-rule": "evenodd",
                                "fill-rule": "evenodd",
                                d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z",
                            }
                        }
                    }
                    div { class: "timeline-end mb-10",
                        time { class: "font-mono italic", "2010" }
                        div { class: "text-lg font-black", "Primeira versão pública." }
                        div {
                            "A primeira versão do Rust é lançada como um projeto de código aberto."
                            img {
                                src: "https://www.rust-lang.org/logos/rust-logo-blk.svg",
                                width: "200vw",
                                class: "invert",
                            }
                        }
                    }
                    hr {}
                }
                li {
                    hr {}
                    div { class: "timeline-middle",
                        svg {
                            fill: "currentColor",
                            xmlns: "http://www.w3.org/2000/svg",
                            "viewBox": "0 0 20 20",
                            class: "h-5 w-5",
                            path {
                                "clip-rule": "evenodd",
                                "fill-rule": "evenodd",
                                d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z",
                            }
                        }
                    }
                    div { class: "timeline-start mb-10 md:text-end",
                        time { class: "font-mono italic", "2012" }
                        div { class: "text-lg font-black", "Rust 0.1." }
                        "A linguagem atinge a versão 0.1, introduzindo novas funcionalidades e melhorias de desempenho."
                    }
                    hr {}
                }
                li {
                    hr {}
                    div { class: "timeline-middle",
                        svg {
                            "viewBox": "0 0 20 20",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "currentColor",
                            class: "h-5 w-5",
                            path {
                                "clip-rule": "evenodd",
                                "fill-rule": "evenodd",
                                d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z",
                            }
                        }
                    }
                    div { class: "timeline-end mb-10",
                        time { class: "font-mono italic", "2013" }
                        div { class: "text-lg font-black", "Rust 0.6 e a adoção pela Mozilla." }
                        "A Mozilla anuncia que está oficialmente apoiando o desenvolvimento do Rust. A linguagem é usada em alguns projetos internos."
                    }
                    hr {}
                }
                li {
                    hr {}
                    div { class: "timeline-middle",
                        svg {
                            "viewBox": "0 0 20 20",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "currentColor",
                            class: "h-5 w-5",
                            path {
                                "clip-rule": "evenodd",
                                "fill-rule": "evenodd",
                                d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z",
                            }
                        }
                    }
                    div { class: "timeline-start mb-10 md:text-end",
                        time { class: "font-mono italic", "2015" }
                        div { class: "text-lg font-black", "Rust 0.6 e a adoção pela Mozilla." }
                        "Lançamento do Rust 1.0."
                    }
                    hr {}
                }
                li {
                    hr {}
                    div { class: "timeline-middle",
                        svg {
                            "viewBox": "0 0 20 20",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "currentColor",
                            class: "h-5 w-5",
                            path {
                                "clip-rule": "evenodd",
                                "fill-rule": "evenodd",
                                d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z",
                            }
                        }
                    }
                    div { class: "timeline-end mb-10",
                        time { class: "font-mono italic", "2016" }
                        div { class: "text-lg font-black", " Ferramentas e melhorias na comunidade." }
                        "Ferramentas como Cargo, o gerenciador de pacotes do Rust, se tornam uma parte integral do ecossistema, facilitando a construção e o gerenciamento de dependências."
                    }
                    hr {}
                }
                li {
                    hr {}
                    div { class: "timeline-middle",
                        svg {
                            "viewBox": "0 0 20 20",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "currentColor",
                            class: "h-5 w-5",
                            path {
                                "clip-rule": "evenodd",
                                "fill-rule": "evenodd",
                                d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z",
                            }
                        }
                    }
                    div { class: "timeline-start mb-10 md:text-end",
                        time { class: "font-mono italic", "2018" }
                        div { class: "text-lg font-black", "Rust 2018." }
                        "Uma nova edição da linguagem é lançada, introduzindo melhorias na ergonomia e na capacidade de escrita de código."
                    }
                    hr {}
                }
                li {
                    hr {}
                    div { class: "timeline-middle",
                        svg {
                            "viewBox": "0 0 20 20",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "currentColor",
                            class: "h-5 w-5",
                            path {
                                "clip-rule": "evenodd",
                                "fill-rule": "evenodd",
                                d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z",
                            }
                        }
                    }
                    div { class: "timeline-end mb-10",
                        time { class: "font-mono italic", "2021" }
                        div { class: "text-lg font-black", " Rust 2021." }
                        "Uma nova edição é lançada, trazendo mais atualizações, melhorias na linguagem e no compilador."
                    }
                    hr {}
                }
                li {
                    hr {}
                    div { class: "timeline-middle",
                        svg {
                            fill: "currentColor",
                            xmlns: "http://www.w3.org/2000/svg",
                            "viewBox": "0 0 20 20",
                            class: "h-5 w-5",
                            path {
                                "fill-rule": "evenodd",
                                d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z",
                                "clip-rule": "evenodd",
                            }
                        }
                    }
                    div { class: "timeline-start mb-10 md:text-end",
                        time { class: "font-mono italic", "2023" }
                        div { class: "text-lg font-black", "Futuras direções e inovações." }
                        "O desenvolvimento contínuo do Rust se concentra em recursos como melhoria de ergonomia, suporte a novos paradigmas de programação e integração com outras tecnologias emergentes."
                    }
                }
                
            }
        }
    }
}
