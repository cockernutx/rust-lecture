use crate::Route;
use dioxus::prelude::*;
use std::fmt::Display;
#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { class: "flex flex-col justify-between h-screen",
            div { class: " ",
                div { class: "navbar shadow-xl  ",
                    div { class: "flex-none ",
                        Link {
                            to: Route::Home {},
                            class: "btn btn-square btn-ghost",
                            img {
                                src: "https://www.rust-lang.org/logos/rust-logo-blk.svg",
                                width: "600vw",
                                class: "invert",
                            }
                        }
                    }
                    div { class: "flex-1",
                        ul { class: "menu menu-horizontal px-1",
                            li {
                                Link {
                                    to: Route::Examples {},
                                    class: "btn btn-ghost ",
                                    "Examples"
                                }
                            }
                            li {
                                Link {
                                    to: Route::AboutUs {},
                                    class: "btn btn-ghost ",
                                    "About us"
                                }
                            }
                            li {
                                Link {
                                    to: Route::Sources {},
                                    class: "btn btn-ghost ",
                                    "Source"
                                }
                            }
                        }
                    }
                    div { class: "flex-none",
                        ul { class: "menu menu-horizontal px-1",
                            li {
                                details {
                                    summary { "Content index" }
                                    ul { class: "rounded-t-none p-2",
                                        li {
                                            Link { to: Route::BorrowChecker {}, "Borrow checker" }
                                        }
                                        li {
                                            Link { to: Route::TypeSystem {}, "Type System" }
                                        }
                                        li {
                                            Link { to: Route::RustHistory {}, "Rust history" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            main { class: "mb-auto ", Outlet::<Route> {} }
        }
    }
}
