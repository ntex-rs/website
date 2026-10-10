#![allow(dead_code)]

// <setup_mutable>
use ntex::web;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct AppStateWithCounter {
    counter: Arc<Mutex<i32>>, // <- Mutex is necessary to mutate safely across threads
}

impl web::State for AppStateWithCounter {
    type Error = web::DefaultError;
}

async fn index(state: &AppStateWithCounter, _: ()) -> String {
    let mut counter = state.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}
// </setup_mutable>

// <make_app_mutable>
#[ntex::main]
async fn main() -> std::io::Result<()> {
    // Note: counter is created _outside_ of the state factory
    let counter = Arc::new(Mutex::new(0));

    web::server_with_config(
        // every worker gets a state with a clone of the same counter
        async move || {
            Ok(AppStateWithCounter {
                counter: counter.clone(),
            })
        },
        async |_| web::App::new().route("/", web::get().to_with_state(index)),
    )
    .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
    .run()
    .await
}
// </make_app_mutable>
