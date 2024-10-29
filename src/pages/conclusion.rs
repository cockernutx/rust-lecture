use dioxus::prelude::*;
#[component]
pub fn Conclusion() -> Element {
    rsx! {
        document::Title { "Conclusão" }
        div { class: "p-2",
            h1 { class: "text-3xl font-figtree font-bold text-center", "Conclusão" }
            p { class: "mb-4 mt-6 text-xl font-light",
                "O projeto proporcionou uma compreensão aprofundada de suas principais características e benefícios do Rust, destacando o compromisso com a segurança, performance e concorrência. Rust se diferencia por oferecer gerenciamento de memória sem a necessidade de um coletor de lixo, garantindo alta eficiência e evitando falhas comuns, como acessos inválidos ou vazamento de memória. Através do conceito de "ownership
                " e o rigoroso sistema de tipos, Rust promove um ambiente de desenvolvimento que minimiza erros em tempo de execução, tornando-se uma escolha ideal para sistemas de baixo nível e aplicações que exigem alto desempenho."
            }
            p { class: "mt-6 text-lg font-light",
                "Além disso, Rust está se consolidando como uma linguagem versátil, sendo utilizada tanto para o desenvolvimento de sistemas operacionais quanto para o desenvolvimento de web, com frameworks como Rocket e Actix. Ao longo do projeto, foi possível observar o crescente suporte da comunidade e a melhoria contínua de suas ferramentas e bibliotecas, o que reforça sua adoção por grandes empresas e projetos de software de missão crítica."
            }
            p { class: "mt-2 text-lg font-light",
                "Em resumo, o projeto evidenciou que Rust é uma linguagem robusta, moderna e que atende às demandas de segurança e desempenho exigidas no desenvolvimento de software atual. Ela tem potencial para se tornar uma escolha dominante, principalmente em áreas onde a confiabilidade e a performance são prioridades."
            }
        }
    }
}
