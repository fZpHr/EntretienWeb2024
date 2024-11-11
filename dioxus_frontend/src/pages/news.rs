use dioxus::prelude::*;
use common::{News, NewsType};
use gloo_net::http::Request;
use time::{OffsetDateTime, format_description};

fn format_date(date: OffsetDateTime) -> String {
    let format = format_description::parse(" [hour]:[minute] [day]/[month]/[year]")
        .unwrap_or_default();
    date.format(&format).unwrap_or_default()
}

#[component]
pub fn NewsList() -> Element {
    let news = use_resource(move || async move {
        Request::get("http://localhost:8000/news")
            .send()
            .await
            .unwrap()
            .json::<Vec<News>>()
            .await
            .unwrap()
    });

    rsx! {
        div {
            class: "max-w-2xl mx-auto p-4",
            
            div {
                class: "bg-white rounded-xl shadow-lg",
                
                div {
                    class: "p-4 border-b sticky top-0 bg-white rounded-t-xl z-10 bg-gradient-to-r from-green-400 via-blue-500 to-purple-600",
                    h2 { 
                        class: "text-4xl font-extrabold text-white",
                        "Dernières actualités" 
                    }
                }

                div {
                    class: "max-h-[600px] overflow-y-auto",
                    
                    {match news.read().as_ref() {
                        None => rsx! {
                            div { 
                                class: "flex justify-center p-4",
                                span {
                                    class: "text-gray-600",
                                    "Chargement des actualités..." 
                                }
                            }
                        },
                        Some(news_list) => {
                            let mut news_vec = news_list.clone();
                            news_vec.sort_by(|a, b| b.date.cmp(&a.date));

                            rsx! {
                                div {
                                    class: "divide-y",
                                    {news_vec.iter().map(|news| rsx! {
                                        div {
                                            key: "{news.title}",
                                            class: "p-4 hover:bg-gray-50 transition-colors",
                                            
                                            div {
                                                class: "flex justify-between items-center mb-2",
                                                span {
                                                    class: match news.r#type {
                                                        NewsType::Intra42 => "text-xs bg-purple-100 text-purple-700 px-2 py-1 rounded-full",
                                                        NewsType::StaffMsg => "text-xs bg-green-100 text-green-700 px-2 py-1 rounded-full",
                                                        NewsType::CampusEvent => "text-xs bg-orange-100 text-orange-700 px-2 py-1 rounded-full"
                                                    },
                                                    "{news.r#type:?}"
                                                }
                                                span {
                                                    class: "text-xs text-gray-500",
                                                    "{format_date(news.date)}"
                                                }
                                            }
                                            
                                            h3 {
                                                class: "font-medium text-gray-800 mb-1",
                                                "{news.title}"
                                            }
                                            
                                            p {
                                                class: "text-sm text-gray-600 line-clamp-2",
                                                "{news.content}"
                                            }
                                        }
                                    })}
                                }
                            }
                        }
                    }}
                }
            }
        }
    }
}