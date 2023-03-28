use ntex::web;
use serde::{Deserialize, Serialize};

// <powerful-extractors>
#[derive(Serialize, Deserialize)]
struct Event {
    id: Option<i32>,
    timestamp: f64,
    kind: String,
    tags: Vec<String>,
}

async fn capture_event(evt: web::types::Json<Event>) -> impl web::Responder {
    let new_event = store_in_db(evt.timestamp, &evt.kind, &evt.tags);
    format!("got event {}", new_event.id.unwrap())
}
// </powerful-extractors>

fn store_in_db(timestamp: f64, kind: &str, tags: &[String]) -> Event {
    // store item in db and get new_event
    // use id to lookup item
    Event {
        id: Some(1),
        timestamp,
        kind: kind.to_string(),
        tags: tags.to_vec(),
    }
}

async fn index() -> web::HttpResponse {
    web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html"))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .route("/", web::get().to(index))
            .route("/event", web::post().to(capture_event))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
