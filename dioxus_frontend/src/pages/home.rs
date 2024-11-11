use dioxus::prelude::*;
use crate::Route;
use crate::pages::news::NewsList;
use crate::pages::form::AppointmentForm;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-slate-100 py-8",
            
            div {
                class: "container mx-auto text-center mb-8",
                h1 {
                    class: "text-4xl font-bold text-slate-800",
                    "Bienvenue sur Rusty"
                }
                p {
                    class: "text-slate-600 mt-2",
                    "Votre portail d'information et de rendez-vous"
                }
            }
            
            div {
                class: "container mx-auto px-4",
                div {
                    class: "grid grid-cols-1 lg:grid-cols-2 gap-8 mb-8",
                    
                    div {
                        NewsList {}
                    }
                    
                    div {
                        AppointmentForm {}
                    }
                }

                div {
                    class: "text-center",
                    Link {
                        to: Route::Faq {},
                        class: "inline-flex items-center px-6 py-2.5 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors duration-200 shadow-sm",
                        "Consulter la FAQ"
                    }
                }
            }
        }
    }
}