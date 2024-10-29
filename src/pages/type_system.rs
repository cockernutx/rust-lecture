use dioxus::prelude::*;
use dioxus_elements::section;
#[component]
pub fn TypeSystem() -> Element {

    
    let move_to_id = |id: &str| {
        let document = gloo::utils::document();
        let el = document.get_element_by_id(&id);
        if let Some(el) = el {
            el.scroll_into_view();
        }
    };

    rsx! {

        div { class: "p-1 ",
            div { class: "hero bg-base-200",
                div { class: "hero-content flex-col lg:flex-row",
                    img {
                        src: "https://rustacean.net/more-crabby-things/gothferris.png",
                        class: "max-w-sm rounded-lg shadow-2xl"
                    }
                    div {
                        h1 { class: "text-5xl font-bold", "Os tipos do Rust!" }
                        p { class: "pt-6 text-lg",
                            "Rust é uma linguagem de programação focada em segurança e performance, e o seu sistema de tipos é uma das suas principais características para garantir esses objetivos."
                        }
                        p { class: "pt-2 text-lg",
                            "A linguagem é fortemente tipada e de tipagem estática, o que significa que os tipos das variáveis são verificados em tempo de compilação, evitando muitos erros comuns."
                        }
                    }
                }
            }
        }
        h1 { class: "text-center text-6xl mt-16 font-black", id: "general", "Visão geral" }
        p { class: "p-2 text-xl",
            "O Rust é uma linguagem de programação que prioriza a segurança e a eficiência. Um dos pilares dessa segurança é seu sistema de tipos robusto e estático, que previne uma série de erros comuns em tempo de compilação, ao invés de deixá-los para serem descobertos em tempo de execução. Aqui abaixo a uma seleção de varios elementos que compõem o sistema de tipos do rust."
        }
       
    }
}
