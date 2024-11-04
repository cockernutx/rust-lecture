

use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn BorrowChecker() -> Element {
    const BORROW_CHECKER_REPRESENTATION: manganis::Asset = asset!(
        "/src/pages/borrow_checker/borrow_checker_representation.webp"
    );
 
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/pages/borrow_checker/borrow_checker.css"),
        }

        div {
            style: "background-image: url({BORROW_CHECKER_REPRESENTATION});",
            class: "hero min-h-screen",
            div { class: "hero-overlay bg-opacity-60" }
            div { class: "hero-content text-neutral-content text-center",
                div { class: "max-w-md",
                    h1 { class: "mb-5 text-5xl font-bold",
                        p { "BORROW CHECKER" }
                    }
                    p { class: "mb-5",
                        "O borrow checker é uma das funções mais importandes do Rust, todo o sistema de tipagem e paradigma da linguagem orbita em torno dele."
                    }
                    Link {
                        class: "btn btn-primary",
                        to: "#what_is_the_borrow_checker",
                        "Continuar"
                    }
                }
            }
        }
        div { class: "relative -mt-64 h-64 bg-gradient-to-t from-english-violet-200 to-transparent" }
        div { class: "mt-64 font-figtree",
            h1 {
                class: "text-center text-8xl font-black",
                id: "what_is_the_borrow_checker",
                "O QUE É O BORROW CHECKER?"
            }
            p { class: "mt-8 p-4 text-xl font-semibold",
                "O borrow checker do Rust é um sistema que garante segurança de memória ao verificar e validar como variáveis são acessadas e usadas no código. Ele faz parte do mecanismo de "ownership
                " da linguagem, que impede que dados sejam modificados ou acessados de forma incorreta, prevenindo problemas como condições de corrida e acessos nulos. O borrow checker verifica se há tentativas de acessar dados de forma simultânea ou insegura em tempo de compilação, o que permite que o Rust ofereça alta segurança de memória sem a necessidade de um coletor de lixo (garbage collector)."
            }
        }
        div { class: "mt-20 text-4xl font-semibold",
            div { class: "flex w-full",
                div { class: " rounded-box grid h-20 flex-grow place-items-center",
                    "BORROW CHECKER"
                }
                div { class: "divider divider-horizontal text-xl", "VS" }
                div { class: "rounded-box grid h-20 flex-grow place-items-center", "GARBAGE COLECTOR" }
            }
        }
        div { class: "mt-8",
            p { class: "p-4 text-xl font-semibold text-center",
                "Então quais são as diferenças entre o borrow checker e um coletor de lixo?"
            }
        }
        div { class: "mt-6 p-4",
            div { class: "overflow-x-auto",
                table { class: "table table-lg",
                    tr {
                        th { "Característica" }
                        th { "Borrow Checker (Rust)" }
                        th { "Garbage Collector (Java, Python, etc.)" }
                    }
                    tr {
                        td { "Tempo de operação" }
                        td { "Tempo de compilação" }
                        td { "Tempo de execução" }
                    }
                    tr {
                        td { "Gerenciamento de memória" }
                        td { "Baseado em regras de ownership e borrowing" }
                        td { "Rastreia e libera automaticamente a memória não utilizada" }
                    }
                    tr {
                        td { "Controle de memória" }
                        td { "Controle manual e preciso pelo desenvolvedor, sem coleta automática" }
                        td { "Gerenciamento automático, menos controle direto sobre alocação" }
                    }
                    tr {
                        td { "Impacto na performance" }
                        td { "Sem pausas em tempo de execução; otimizado para alta performance" }
                        td { "Pode introduzir pausas imprevisíveis (stop-the-world)" }
                    }
                    tr {
                        td { "Segurança de memória" }
                        td {
                            "Garantida por regras de compilação; evita vazamentos e condições de corrida"
                        }

                        td { "Segurança de memória garantida, mas sujeito a pausas de coleta" }
                    }
                    tr {
                        td { "Sobrecarga de runtime" }
                        td { "Baixa, pois não há monitoramento contínuo da memória" }
                        td { "Maior, devido ao monitoramento e liberação periódica da memória" }
                    }
                    tr {
                        td { "Adequação para sistemas" }
                        td { "Ideal para sistemas de tempo real e aplicações de alta performance" }
                        td {
                            "Mais comum em aplicações onde pausas são toleráveis (ex.: interfaces gráficas)"
                        }
                    }
                    tr {
                        td { "Facilidade de uso" }
                        td { "Exige aprendizado das regras de ownership e borrowing" }
                        td { "Mais fácil de usar, pois a coleta de lixo é automática" }
                    }
                }
            }
        }
        div { class: "mt-12 divider p-4",
            p { class: "p-4 text-4xl font-bold text-center", "TIPOS E O BORROW CHECKER" }
        }
        div { class: "mt-6 p-4",
            p { class: "text-lg font-semibold",
                "O borrow checker do Rust é um dos componentes fundamentais que afeta e estende o sistema de tipos da linguagem, proporcionando uma garantia adicional de segurança em relação ao uso de referências e à propriedade de dados. Essa integração entre borrow checker e o sistema de tipos faz do Rust uma linguagem segura e eficiente, muito bem vista para sistemas de alto desempenho."
            }
        }
        div { class: "mt-4 p-4 flex flex-col items-center justify-center ",
            Link {
                class: "text-2xl font-semibold text-center btn btn-outline btn-secondary",
                to: Route::TypeSystem {},
                "Entenda mais sobre o sistemas de tipos do Rust"
            }
        }
    }
}
