use dioxus::prelude::*;
use common::FormContent;
use gloo_net::http::Request;

#[component]
pub fn AppointmentForm() -> Element {
    let mut selected_email = use_signal(String::new);
    let mut selected_name = use_signal(String::new);
    let mut selected_date = use_signal(String::new);
    let mut selected_time = use_signal(String::new);
    let mut reason = use_signal(String::new);
    let status = use_signal(|| None::<String>);

    let handle_submit = move |_ev: Event<FormData>| {
        let form_content = FormContent {
            datetime: format!("{} {}", selected_date.read(), selected_time.read()),
            message: Some(reason.read().clone()),
            email: selected_email.read().clone(),
            login: selected_name.read().clone(),
        };

        to_owned![status];
        spawn(async move {
            match Request::post("http://localhost:8000/appointment")
                .json(&form_content)
                .unwrap()
                .send()
                .await 
            {
                Ok(_) => *status.write() = Some("Point pédago enregistré avec succès!".to_string()),
                Err(e) => *status.write() = Some(format!("Erreur: {}", e))
            }
        });
    };

    rsx! {
        div { class: "max-w-2xl mx-auto p-4",
            div { class: "bg-white rounded-xl shadow-lg",
                div { 
                    class: "p-4 border-b sticky top-0 rounded-t-xl z-10 bg-gradient-to-r from-green-400 via-blue-500 to-purple-600",
                    h2 { class: "text-2xl font-bold text-white", "Prendre un point pédago avec Doc" }
                }

                form { 
                    class: "p-6 space-y-4",
                    onsubmit: handle_submit,
                    prevent_default: "submit",
                    
                    div { 
                        label { class: "block text-sm font-medium text-gray-700", "Nom" }
                        input {
                            class: "mt-1 block w-full rounded-md border p-2",
                            r#type: "text",
                            value: "{selected_name}",
                            oninput: move |ev| *selected_name.write() = ev.value().to_string(),
                            required: true
                        }
                    }
                    
                    div { 
                        label { class: "block text-sm font-medium text-gray-700", "Email" }
                        input {
                            class: "mt-1 block w-full rounded-md border p-2",
                            r#type: "email",
                            value: "{selected_email}",
                            oninput: move |ev| *selected_email.write() = ev.value().to_string(),
                            required: true
                        }
                    }

                    div { 
                        label { class: "block text-sm font-medium text-gray-700", "Date" }
                        input {
                            class: "mt-1 block w-full rounded-md border p-2",
                            r#type: "date",
                            value: "{selected_date}",
                            oninput: move |ev| *selected_date.write() = ev.value().to_string(),
                            required: true
                        }
                    }

                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Heure" }
                        input {
                            class: "mt-1 block w-full rounded-md border p-2",
                            r#type: "time",
                            value: "{selected_time}",
                            oninput: move |ev| *selected_time.write() = ev.value().to_string(),
                            required: true
                        }
                    }

                    div {
                        label { class: "block text-sm font-medium text-gray-700", "Motif" }
                        textarea {
                            class: "mt-1 block w-full rounded-md border p-2",
                            rows: "3",
                            value: "{reason}",
                            oninput: move |ev| *reason.write() = ev.value().to_string(),
                            required: true,
                            placeholder: "Raison du point pédago..."
                        }
                    }

                    button {
                        class: "w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700",
                        r#type: "submit",
                        "Confirmer"
                    }

                    {status.read().as_ref().map(|msg| rsx! {
                        div { class: "mt-4 p-4 rounded-md bg-green-50 text-green-700", "{msg}" }
                    })}
                }
            }
        }
    }
}