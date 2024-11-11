use dioxus::prelude::*;
use common::FormContent;
use gloo_net::http::Request;

#[component]
pub fn AppointmentForm() -> Element {
    let mut selected_date = use_signal(String::new);
    let mut selected_time = use_signal(String::new);
    let mut reason = use_signal(String::new);
    let status = use_signal(|| None::<String>);

    let handle_submit = move |ev: Event<FormData>| {
        ev.stop_propagation();
        
        let form_content = FormContent {
            datetime: format!("{} {}", selected_date.read(), selected_time.read()),
            message: Some(reason.read().clone()),
            email: String::from("user@42.fr"),
            login: String::from("student"),
        };

        to_owned![status, selected_date, selected_time, reason];
        spawn(async move {
            match Request::post("http://localhost:8000/appointment")
                .json(&form_content)
                .unwrap()
                .send()
                .await
            {
                Ok(_) => {
                    let mut s = status.write();
                    *s = Some("Rendez-vous enregistré avec succès!".to_string());
                    
                    let mut sd = selected_date.write();
                    *sd = String::new();
                    
                    let mut st = selected_time.write();
                    *st = String::new();
                    
                    let mut r = reason.write();
                    *r = String::new();
                }
                Err(e) => {
                    let mut s = status.write();
                    *s = Some(format!("Erreur: {}", e));
                }
            }
        });
    };

    rsx! {
        div { class: "max-w-2xl mx-auto p-4",
            div { class: "bg-white rounded-xl shadow-lg",
                div { 
                    class: "p-4 border-b bg-gradient-to-r from-green-400 via-blue-500 to-purple-600",
                    h2 { 
                        class: "text-2xl font-bold text-white",
                        "Prendre un rendez-vous"
                    }
                }

                form { 
                    class: "p-6 space-y-4",
                    onsubmit: handle_submit,
                    prevent_default: "onsubmit",

                    // Date
                    div { 
                        label { 
                            class: "block text-sm font-medium text-gray-700",
                            "Date"
                        }
                        input {
                            class: "mt-1 block w-full rounded-md border p-2",
                            r#type: "date",
                            value: "{selected_date}",
                            oninput: move |ev| {
                                let mut date = selected_date.write();
                                *date = ev.value().to_string();
                            },
                            required: true
                        }
                    }

                    // Heure
                    div {
                        label { 
                            class: "block text-sm font-medium text-gray-700",
                            "Heure"
                        }
                        input {
                            class: "mt-1 block w-full rounded-md border p-2",
                            r#type: "time",
                            value: "{selected_time}",
                            oninput: move |ev| {
                                let mut time = selected_time.write();
                                *time = ev.value().to_string();
                            },
                            required: true
                        }
                    }

                    // Motif
                    div {
                        label { 
                            class: "block text-sm font-medium text-gray-700",
                            "Motif"
                        }
                        textarea {
                            class: "mt-1 block w-full rounded-md border p-2",
                            rows: "3",
                            value: "{reason}",
                            oninput: move |ev| {
                                let mut r = reason.write();
                                *r = ev.value().to_string();
                            },
                            required: true,
                            placeholder: "Raison du rendez-vous..."
                        }
                    }

                    // Bouton
                    button {
                        class: "w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700",
                        r#type: "submit",
                        "Confirmer"
                    }

                    // Message de status
                    {status.read().as_ref().map(|msg| rsx! {
                        div {
                            class: "mt-4 p-4 rounded-md bg-green-50 text-green-700",
                            "{msg}"
                        }
                    })}
                }
            }
        }
    }
}