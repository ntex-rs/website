pub mod handlers_arc;
// <data>
use ntex::web;
use std::cell::Cell;

#[derive(Clone)]
struct AppState {
    count: Cell<usize>,
}

async fn show_count(data: web::types::State<AppState>) -> impl web::Responder {
    format!("count: {}", data.count.get())
}

async fn add_one(data: web::types::State<AppState>) -> impl web::Responder {
    let count = data.count.get();
    data.count.set(count + 1);

    format!("count: {}", data.count.get())
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let data = AppState {
        count: Cell::new(0),
    };

    web::HttpServer::new(move || {
        web::App::new()
            .state(data.clone())
            .route("/", web::to(show_count))
            .route("/add", web::to(add_one))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </data>
