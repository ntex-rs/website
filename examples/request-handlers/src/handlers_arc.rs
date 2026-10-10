#![allow(dead_code)]

// <arc>
use ntex::web;
use std::{
    cell::Cell,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
    sync::Arc,
};

#[derive(Clone)]
struct AppState {
    local_count: Rc<Cell<usize>>,
    global_count: Arc<AtomicUsize>,
}

impl web::State for AppState {
    type Error = web::DefaultError;
}

async fn show_count(state: &AppState, _: ()) -> String {
    format!(
        "global_count: {}\nlocal_count: {}",
        state.global_count.load(Ordering::Relaxed),
        state.local_count.get()
    )
}

async fn add_one(state: &AppState, _: ()) -> String {
    state.global_count.fetch_add(1, Ordering::Relaxed);

    let local_count = state.local_count.get();
    state.local_count.set(local_count + 1);

    format!(
        "global_count: {}\nlocal_count: {}",
        state.global_count.load(Ordering::Relaxed),
        state.local_count.get()
    )
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    // shared between all worker threads
    let global_count = Arc::new(AtomicUsize::new(0));

    web::server_with_config(
        async move || {
            Ok(AppState {
                local_count: Rc::new(Cell::new(0)),
                global_count: global_count.clone(),
            })
        },
        async |_| {
            web::App::new()
                .route("/", web::get().to_with_state(show_count))
                .route("/add", web::get().to_with_state(add_one))
        },
    )
    .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
    .run()
    .await
}
// </arc>
