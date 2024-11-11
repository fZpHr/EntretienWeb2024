use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Faq() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-slate-100 py-8 relative",
            
            div {
                class: "absolute top-4 left-4",
                Link {
                    to: Route::Home {},
                    class: "inline-flex items-center px-4 py-2 bg-white text-slate-700 rounded-lg shadow-sm hover:bg-slate-50 transition-colors duration-200 border border-slate-200",
                    span {
                        class: "mr-2",
                        "←"
                    }
                    "Retour"
                }
            }

            div {
                class: "container mx-auto px-4",
                div {
                    class: "text-center mb-8",
                    h1 {
                        class: "text-4xl font-bold text-slate-800",
                        "FAQ"
                    }
                    p {
                        class: "text-slate-600 mt-2",
                        "Questions fréquentes sur le projet"
                    }
                }

                div {
                    class: "max-w-3xl mx-auto space-y-4",

                    div {
                        class: "bg-white rounded-lg shadow-md border border-slate-200 p-4 hover:shadow-lg transition-shadow duration-300",
                        h2 {
                            class: "text-lg font-semibold text-slate-800 mb-2",
                            "🌱 Quel est votre niveau en Rust ?"
                        }
                        p {
                            class: "text-slate-600",
                            "Débutant. J'apprends le langage à travers ce projet qui est ma première expérience avec Dioxus et Axum."
                        }
                    }

                    div {
                        class: "bg-white rounded-lg shadow-md border border-slate-200 p-4 hover:shadow-lg transition-shadow duration-300",
                        h2 {
                            class: "text-lg font-semibold text-slate-800 mb-2",
                            "🤖 Quelle a été l'approche pour ce projet ?"
                        }
                        p {
                            class: "text-slate-600",
                            "J'ai utilisé une IA comme assistant pour m'aider à comprendre et écrire l'architecture Rust. Elle m'a guidé dans l'apprentissage tout en m'assurant de bien comprendre les concepts (cependant, la limite de temps ne m'a pas permis de comprendre toutes les notions en profondeur...)."
                        }
                    }

                    div {
                        class: "bg-white rounded-lg shadow-md border border-slate-200 p-4 hover:shadow-lg transition-shadow duration-300",
                        h2 {
                            class: "text-lg font-semibold text-slate-800 mb-2",
                            "🛠 Quels outils utilisez-vous ?"
                        }
                        p {
                            class: "text-slate-600",
                            "Dioxus pour le frontend, Axum pour le backend et Tailwind CSS pour le style. Le tout en Rust pour une expérience d'apprentissage complète."
                        }
                    }

                    div {
                        class: "bg-white rounded-lg shadow-md border border-slate-200 p-4 hover:shadow-lg transition-shadow duration-300",
                        h2 {
                            class: "text-lg font-semibold text-slate-800 mb-2",
                            "💡 Qu'avez-vous appris ?"
                        }
                        p {
                            class: "text-slate-600",
                            "Les bases de Rust / Dioxus / Axum / Tailwind CSS. J'ai également appris à structurer un projet complet en Rust, de la gestion des dépendances à la publication."
                        }
                    }
                }
            }
        }
    }
}