// <setup_mutable>
use ntex::web;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct AppStateWithCounter {
    counter: Arc<Mutex<i32>>, // <- Mutex is necessary to mutate safely across threads
}

async fn index(data: web::types::State<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}
// </setup_mutable>

// <make_app_mutable>
#[ntex::main]
async fn main() -> std::io::Result<()> {
    // Note: app state created _outside_ HttpServer::new closure
    let counter = AppStateWithCounter {
        counter: Arc::new(Mutex::new(0)),
    };

    web::HttpServer::new(move || {
        // move counter into the closure
        web::App::new()
            .state(counter.clone()) // <- register the created data
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </make_app_mutable>
