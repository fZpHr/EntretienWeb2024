use axum::{
    http::{header::CONTENT_TYPE, Method, StatusCode, Uri},
    routing::{get, post},
    Json, Router,
};
use common::{FormContent, News, NewsType};
use tower_http::cors::CorsLayer;
use time::OffsetDateTime;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app().await).await.unwrap();
}

async fn not_found(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

pub async fn app() -> Router {
    let origins = ["http://localhost:8080".parse().unwrap()];
    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::OPTIONS,
            Method::DELETE,
        ])
        .allow_headers([CONTENT_TYPE])
        .allow_credentials(true);

    Router::new()
        .route("/news", get(get_news))
        .route("/appointment", post(post_appointment))
        .layer(cors)
        .fallback(not_found)
}

async fn get_news() -> Result<Json<Vec<News>>, String> {
    Ok(Json(vec![
        News {
            title: "Exam.".to_string(),
            content: "Exam 02, 03, 04, 05, 06".to_string(),
            r#type: NewsType::Intra42,
            date: OffsetDateTime::now_utc() - time::Duration::days(1),
        },
        News {
            title: "Joy a réalisé un salto dans la cour de l'école !!!".to_string(),
            content: "Si vous avez raté cette évènement, pas de chance...".to_string(),
            r#type: NewsType::StaffMsg,
            date: OffsetDateTime::now_utc() - time::Duration::days(2),
        },
        News {
            title: "HTBM".to_string(),
            content: "Internship and Apprenticeship".to_string(),
            r#type: NewsType::StaffMsg,
            date: OffsetDateTime::now_utc() - time::Duration::days(3),
        },
        News {
            title: "Conference YY Company. It's a startup. It'll be big, like unicorn big. Not much to say about this company, no one knows what they do but they've raised a shit load of money by just spitting the AI keyword in every single pitch sentence. It does web, who would've thought. It needs a frontend + backend + devops + blockchain + AI. Multiple experiences required, 600/monthly salary but don't worry, after that you'll not be paid but you'll get 2% of the company and be our CTO. Gosh sorry this title is long, good luck with that.".to_string(),
            content: "YY need to hire new students. Don't miss out the opportunity.".to_string(),
            r#type: NewsType::CampusEvent,
            date: OffsetDateTime::now_utc() - time::Duration::days(4),
        },
        News {
            title: "intra v3 is out again, with new bugs introduced".to_string(),
            content: "🐛".to_string(),
            r#type: NewsType::Intra42,
            date: OffsetDateTime::now_utc() - time::Duration::days(5),
        },
        News {
            title: "rusty v0.0.0.0.0.1 is out, featuring a news page developed by our new team member".to_string(),
            content: "Yesterday, Rusty was a website hastily developed to meet administrative needs and provide basic services.

Today, Rusty is a web platform offering services easing students life and helping them define their professional and educational paths. It is also a means of continuous improvement, giving students a voice heard by the staff.

Tomorrow, Rusty will assist students on all platforms and in all matters related to their journey. It will be a connected hub of information and a comprehensive educational guide, helping them in defining their paths and tracking their progress over time.".to_string(),
            r#type: NewsType::CampusEvent,
            date: OffsetDateTime::now_utc() - time::Duration::days(6),
        },
        News {
            title: "Improving students experience".to_string(),
            content: "Starting next week, the campus will be open 72/7 to provide students the opportunity to validate common core within a month.".to_string(),
            r#type: NewsType::CampusEvent,
            date: OffsetDateTime::now_utc() - time::Duration::days(7),
        },
        News {
            title: "Rusty's Getting Rustier as Rust Celebrates its 42nd Birthday!".to_string(),
            content: "It’s hard to believe, but Rust, the language that was once 'the new kid on the block,' is now officially middle-aged. Yes, folks, Rust has reached the grand milestone of its 42nd birthday—a number so significant that it’s likely to mean *something* deep, even if no one can pinpoint exactly what. For those of you who may be unfamiliar with this programming marvel, Rust is the love child of Mozilla and a bunch of developers who were frankly fed up with memory leaks, dangling pointers, and terrifying debugging sessions at 3 a.m. It's the language that looked at other programming languages and said, 'Hold my beer.'
        
        As part of the festivities, Rustaceans (that’s what hardcore Rust fans call themselves, for the uninitiated) have taken to the internet to reminisce over Rust’s early days. One dedicated fan recounted how Rust 0.1 was basically a glorified scientific experiment, resembling more of a Choose Your Own Adventure game than a language. Back then, if you managed to compile anything, you won the programming equivalent of a gold medal, and you probably took the rest of the day off to celebrate.
        
        For those of us on campus who are die-hard Rust fans, we’re marking the occasion by doing what we do best: writing code that’s as safe as it is cool and having lengthy debates on how memory safety is a spiritual experience. It’s a shame there’s no actual cake to cut, though. To remedy this, someone in the IT department has suggested we symbolically ‘leak memory’ in Rust's honor (don’t worry, we’ll fix it by Monday). Meanwhile, campus administration is still wondering what 'Rust' is and why everyone insists on celebrating it like a holiday.
        
        In the coming week, prepare for Rust-themed posters around the campus, new Rusty stickers for laptops, and 'fun facts' posted randomly on Slack. Did you know the name 'Rust' was partly inspired by the oxide that forms on metals? It’s as if Rust's creators knew from the start it would age like fine iron. By next Friday, we’ll also be hosting a 'Rust-Your-Code' session, where students can bring their messiest projects, and we’ll attempt to convert them to safe Rust code. As a final touch, the WiFi password for the month will be changed to 'NoGCMemorySafe42'. 
        
        Here’s to Rust, our beloved language, and to Rusty, our trusty campus app that reminds us of everything great about modern code... even if it gets rustier by the day. Happy Birthday, Rust! May your enums be exhaustive, your borrows non-dangling, and your syntax ever confusing to beginners.".to_string(),
            r#type: NewsType::Intra42,
            date: OffsetDateTime::now_utc() - time::Duration::days(8),
        },
        News {
            title: "Lost & Found".to_string(),
            content: "A set of keys and a yellow canary were found in the main building.".to_string(),
            r#type: NewsType::StaffMsg,
            date: OffsetDateTime::now_utc() - time::Duration::days(9),
        },
        News {
            title: "Big news: C1 toilets have stayed clean for 10 minutes straight".to_string(),
            content: "That's dope!".to_string(),
            r#type: NewsType::StaffMsg,
            date: OffsetDateTime::now_utc() - time::Duration::days(10),
        },
        News {
            title: "Food Truck Fridays Are Back!".to_string(),
            content: "Every Friday, food trucks will be available outside the main building from 11 AM to 2 PM. Enjoy a variety of cuisines!".to_string(),
            r#type: NewsType::CampusEvent,
            date: OffsetDateTime::now_utc() - time::Duration::days(11),
        },
        News {
            title: "New Study Rooms Available".to_string(),
            content: "We've added more study rooms on the second floor. This feature is now supported thanks to our new elastic and scalable architecture.".to_string(),
            r#type: NewsType::StaffMsg,
            date: OffsetDateTime::now_utc() - time::Duration::days(12),
        },
        News {
            title: "Campus WiFi Upgrade".to_string(),
            content: "We're upgrading the campus WiFi next week. Expect improved speed and coverage across all areas, including under the babyfoot for Fanny losers.".to_string(),
            r#type: NewsType::StaffMsg,
            date: OffsetDateTime::now_utc() - time::Duration::days(13),
        },
        News {
            title: "New Course on 'Advanced Procrastination'".to_string(),
            content: "Enroll now to learn the art of productive delay. Seats are expected to fill up... eventually.".to_string(),
            r#type: NewsType::CampusEvent,
            date: OffsetDateTime::now_utc() - time::Duration::days(14),
        },
        News {
            title: "Mystery of Missing Whiteboard Markers Solved".to_string(),
            content: "Turns out, they've been under the coffee machine the whole time. Case closed!".to_string(),
            r#type: NewsType::StaffMsg,
            date: OffsetDateTime::now_utc() - time::Duration::days(15),
        },
        News {
            title: "AI Program Now Generating Random Staff Messages".to_string(),
            content: "Because the real news was too normal, an AI is now generating daily nonsense to keep things interesting.".to_string(),
            r#type: NewsType::Intra42,
            date: OffsetDateTime::now_utc() - time::Duration::days(16),
        },
        News {
            title: "Campus Will Now Charge projects failures in Cryptocurrency".to_string(),
            content: "Pay your dues in Dogecoin or get ready to have your 42 Student badge repossessed.".to_string(),
            r#type: NewsType::CampusEvent,
            date: OffsetDateTime::now_utc() - time::Duration::days(17),
        },
        News {
            title: "Campus Introduces 'No Coffee' Mondays".to_string(),
            content: "Due to caffeine budget cuts, Mondays will be a coffee-free day. Good luck!".to_string(),
            r#type: NewsType::StaffMsg,
            date: OffsetDateTime::now_utc() - time::Duration::days(18),
        },
        News {
            title: "IT Dept Announces Plans to Make Error Messages 'More Friendly'".to_string(),
            content: "Starting next week, error screens will come with compliments to boost morale.".to_string(),
            r#type: NewsType::StaffMsg,
            date: OffsetDateTime::now_utc() - time::Duration::days(19),
        },
        News {
            title: "New 'Quiet Zone' in C1 Actually Just Soundproof Headphones".to_string(),
            content: "Budget-friendly solution allows students to enter their own world of silence. Bring your own headphones.".to_string(),
            r#type: NewsType::CampusEvent,
            date: OffsetDateTime::now_utc() - time::Duration::days(20),
        },
        News {
            title: "Student Proposes Installing Beds in Study Rooms".to_string(),
            content: "The initiative was denied, but comfy bean bags are being considered as a compromise.".to_string(),
            r#type: NewsType::CampusEvent,
            date: OffsetDateTime::now_utc() - time::Duration::days(21),
        },
        News {
            title: "Introducting Dioxus hooks".to_string(),
            content: "TW9zdCB1c2VkIGhvb2tzOg0KLSB1c2Vfc2lnbmFsKCk6IERlY2xhcmUgYSBuZXcgU2lnbmFsPFQ+LCB0aGUgY2xvc3VyZSBwYXNzZWQgdG8gdGhlIGZ1bmN0aW9uIHdpbGwgYmUgcnVuIG9ubHkgdGhlIGZpcnN0IHRpbWUgdGhlIGNvbXBvbmVudCBpcyByZW5kZXJlZCwgYW5kIHRoaXMgc2FtZSBjbG9zdXJlIHNob3VsZCByZXR1cm4gdGhlIHR5cGUgVCB5b3Ugd2FudCB5b3VyIFNpZ25hbCB0byBiZS4gQSBTaWduYWwgd2lsbCBiZSBkcm9wcGVkIHdoZW4gdGhlIGNvbXBvbmVudCB3aG9tIGRlY2xhcmVkIGlzIGRyb3BwZWQuDQotIHVzZV9lZmZlY3QoKTogQSBjbG9zdXJlIHRoYXQgd2lsbCBydW4gb25jZSB0aGUgZmlyc3QgdGltZSB0aGUgY29tcG9uZW50IGlzIHJlbmRlcmVkLiBJZiB5b3UgLnJlYWQoKSBhIFNpZ25hbCBpbiBpdCwgaXQgd2lsbCByZS1ydW4gZXZlcnkgdGltZSB0aGUgc2FpZCBTaWduYWwgaXMgd3JpdHRlbiB0byAoaWUgU3Vic2NyaWJlciBQYXR0ZXJuKS4gSXQgaXMgYSBnb29kIHdheSB0byB3YXkgdG8gLndyaXRlKCkgdG8gYSBTaWduYWwsIHNpbmNlIGl0IGlzIGV4ZWN1dGVkIGFmdGVyIHRoZSByZW5kZXJpbmcgYW5kIG5vdCBkdXJpbmcsIHdoaWNoIG1ha2UgaXQgaGFyZGVyIHRvIGhhdmUgaW5maW5pdGUgcmVuZGVyaW5nIGxvb3BzLiBJbiAwLjUueCB5b3Ugc2hvdWxkIG5vdCByZWFkIGFuZCB3cml0ZSB0byB0aGUgc2FtZSBTaWduYWwgaW4gdGhlIHNhbWUgdXNlX2VmZmVjdCwgaXQgd2lsbCBjYXVzZSBpbmZpbml0ZSBsb29wcywgYnV0IEkgdGhpbmsgaXQgaXMgZml4ZWQgaW4gMC42LnguDQotIHVzZV9tZW1vKCk6IFZlcnkgbXVjaCBsaWtlIGEgdXNlX3NpZ25hbCgpIGJ1dCBpdCB3aWxsIGdpdmUgeW91IHNvbWV0aGluZyBtb3JlIG9mIGEgUmVhZE9ubHlTaWduYWwsIGFuZCB3aXRoIHRoZSBkaWZmZXJlbmNlIHRoYXQgaXQgd2lsbCBzdWJzY3JpYmUgdG8gZXZlcnkgU2lnbmFsIHlvdSAucmVhZCgpIGZyb20gaW4gdGhlIGNsb3N1cmUgeW91IHBhc3MuIFZlcnkgdXNlZnVsIHdoZW4geW91IHdhbnQgdG8gY29tcHV0ZSBzb21ldGhpbmcgZnJvbSBhIFNpZ25hbCwgd2l0aG91dCBkZWNsYXJpbmcgYW5vdGhlciBTaWduYWwgYW5kIGhhdmluZyBhIHVzZV9lZmZlY3Qgd3JpdGUgdG8gaXQuIFRoaXMgaG9vayBhbHNvIGNvbWVzIHdpdGggc29tZSBzb3J0IG9mIG9wdGltaXphdGlvbiwgaWUgaWYgYSBTaWduYWwgeW91IHJlYWQgZnJvbSBjaGFuZ2VzIGFuZCB0aGUgaG9vayByZXJ1bnMgYnV0IHRoZSB2YWx1ZSBzdGF5cyB0aGUgc2FtZSwgaXQgd2lsbCBub3QgY2F1c2UgYSByZS1yZW5kZXIgb2YgdGhlIGNvbXBvbmVudC4NCi0gdXNlX2Z1dHVyZSgpOiBOb3QgdXNlZCBhbnltb3JlIEkgdGhpbmssIGdldHRpbmcgcmVwbGFjZWQgYnkgdXNlX3Jlc291cmNlKCkgKCBmcm9tIGh0dHBzOi8vZGlveHVzbGFicy5jb20vbGVhcm4vMC41L21pZ3JhdGlvbi9ob29rcyApDQotIHVzZV9yZXNvdXJjZSgpOiBBIGhvb2sgdGhhdCBsZXQgeW91IHJ1biBhbiBhc3luYyBjbG9zdXJlIGFuZCByZXR1cm4gYSBSZXNvdXJjZS4gWW91IGNhbiAucmVhZCgpIGZyb20gdGhpcyBSZXNvdXJjZSB0byBnZXQgdGhlIHJlc3VsdCBvZiB0aGlzIFJlc291cmNlLCBTb21lKE9rKF8pKSBtZWFucyB0aGUgYXN5bmMgY2FsbCBlbmRlZCBzdWNjZXNzZnVsbHksIFNvbWUoRXJyKF8gKSkgbWVhbnMgYW4gZXJyb3IsIE5vbmUgbWVhbnMgaXQgaXMgc3RpbGwgbG9hZGluZy4gVGhlIGNsb3N1cmUgeW91IHBhc3MgdG8gdGhlIGhvb2sgd2lsbCBhbHNvIHN1YnNjcmliZSB0byBhbnkgU2lnbmFsIHlvdSAucmVhZCgpIGluIHRoZSBjbG9zdXJlLCBjYXVzaW5nIGl0IHRvIHJlLXJ1biBldmVyeSB0aW1lIGlzIGl0IHdyaXR0ZW4gdG8uIFlvdSBjYW4gYWxzbyBtYW51YWxseSBjYWxsIC5yZXN0YXJ0KCkgb24gdGhlIFJlc291cmNlIHRvIG1hbnVhbGx5IGRvIGl0Lg0KTW9yZSBzcGVjaWZpYyBob29rczoNCi0gdXNlX2NvbnRleHRfcHJvdmlkZXIoKTogSWYgcGFzc2luZyBhIFNpZ25hbCBhcyBwYXJhbWV0ZXIgb2YgYSBjb21wb25lbnQgaXMgbm90IGZlYXNpYmxlIG9yIGp1c3Qgbm90IG5pY2UgdG8gZG8sIHlvdSBjYW4gZGVjbGFyZSBhIGNvbnRleHQgaW4gdGhlIHBhcmVudCBhbmQgYW55IGNoaWxkIHdpbGwgYmUgYWJsZSB0byByZXRyaWV2ZSB0aGlzIGNvbnRleHQuIE5vdGU6IGl0IGlzIG5vdCBtYW5kYXRvcnkgbWFrZSB0aGlzIG9uZSBhIFNpZ25hbCwgYnV0IEkgdGhpbmsgaXQgc2hvdWxkIGFsbW9zdCBhbHdheXMgYmUgdGhlIGNhc2UgKGllIDogdXNlX2NvbnRleHRfcHJvdmlkZXIofHwgZmFsc2UpIHdpbGwgcmV0dXJuIGEgYm9vbCB2ZXJzdXMgdXNlX2NvbnRleHRfcHJvdmlkZXIofHwgU2lnbmFsOjpuZXcoZmFsc2UpIHdpbGwgcmV0dXJuIGEgU2lnbmFsPGJvb2w+KQ0KLSB1c2VfY29udGV4dCgpOiBVc2VkIHRvIHJldHJpZXZlIGEgY29udGV4dCBmcm9tIGEgcGFyZW50IGNvbXBvbmVudCwgd2lsbCBwYW5pYyBpZiB0aGUgY29udGV4dCBpcyBub3QgZm91bmQNCi0gdHJ5X3VzZV9jb250ZXh0KCk6IFNhbWUgYXMgYWJvdmUgYnV0IHdpbGwgcmV0dXJuIGEgT3B0aW9uIGFuZCBub3QgcGFuaWMhDQotIHVzZV9jb3JvdXRpbmUoKTogVGhpcyBob29rIGlzIHZlcnkgdXNlZnVsIHdoZW4geW91IHdhbnQgdG8gY3JlYXRlIHNvbWUgc29ydCBhIGJhY2tncm91bmQgdGFzay4gSW4gb3VyIGNhc2Ugd2UgdXNlIGl0IHRvIHVwZGF0ZSBvdXIgZ2xvYmFsIFVTRVIuIFlvdSBjYW4gc2VlIGluIHRoZSBmbiBnbG9iYWxfc2VydmljZSgpIHRoYXQgaXQgLmF3YWl0IG9uIGEgbWVzc2FnZSwgYW5kIHdoZW4gYSBtZXNzYWdlIGNvbWVzIGl0IHdpbGwgZG8gc29tZSBhY3Rpb24uIE91ciBtZXNzYWdlIGFyZSBpbiBmb3JtIG9mIEVudW0uIEJvdGggdXNlX2Nvcm91dGluZSBhbmQgdXNlX2Nvcm91dGluZV9oYW5kbGUgd29ya3MgdmVyeSBtdWNoIGxpa2UgdXNlX2NvbnRleHRfcHJvdmlkZXIgYW5kIHVzZV9jb250ZXh0LCBtZWFuaW5nIHRoYXQgaWYgeW91IHdhbnQgdG8gc2VuZCBhIG1lc3NhZ2UgeW91IGhhdmUgdG8gcmV0cmlldmUgdGhlIGhhbmRsZSB2aWEgdXNlX2Nvcm91dGluZV9oYW5kbGUgZnJvbSBhIHBhcmVudCBjb21wb25lbnQuIFRoZW4geW91IGNhbiBjYWxsIC5zZW5kKCkgb24gdGhpcyBoYW5kbGUgd2l0aCB0aGUgbWVzc2FnZSB5b3Ugd2FudCB0byBiZSBzZW50IGFuZCB0cmVhdGVkLg0KLSB1c2VfbmF2aWdhdG9yKCk6IFVzZWQgdG8gcHJvZ2FtbWF0aWNhbHkgbmF2aWdhdGUgdGhlIHVzZXIsIGllIHRvIG1ha2UgaXQgY2hhbmdlIHBhZ2Ugd2hlbiBhbiBhY3Rpb24gaXMgZG9uZSBvciB0aGluZ3MgbGlrZSB0aGF0DQotIHVzZV9yb3V0ZSgpOiBVc2VkIHRvIGdldCBpbmZvcm1hdGlvbiBhYm91dCB0aGUgY3VycmVudCByb3V0ZSwgbGlrZSB3aGF0IGlzIHRoZSBjdXJyZW50IHJvdXRlIHRoZSB1c2VyIGlzIHJlbmRlcmluZw==".to_string(),
            r#type: NewsType::StaffMsg,
            date: OffsetDateTime::now_utc() - time::Duration::days(22),
        },
        News {
            title: "Announcement: All IT Issues Will Now be Blamed on Mercury Retrograde".to_string(),
            content: "Until further notice, all outages and bugs are attributed to celestial interference.".to_string(),
            r#type: NewsType::Intra42,
            date: OffsetDateTime::now_utc() - time::Duration::days(23),
        },
    ]))
}

async fn post_appointment(Json(_form_content): Json<FormContent>) -> Result<(), String> {
    Ok(())
}
