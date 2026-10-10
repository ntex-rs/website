pub mod handlers_arc;
// <data>
use ntex::web;
use std::{cell::Cell, rc::Rc};

#[derive(Clone)]
struct AppState {
    count: Rc<Cell<usize>>,
}

impl web::State for AppState {
    type Error = web::DefaultError;
}

async fn show_count(state: &AppState, _: ()) -> String {
    format!("count: {}", state.count.get())
}

async fn add_one(state: &AppState, _: ()) -> String {
    let count = state.count.get();
    state.count.set(count + 1);

    format!("count: {}", state.count.get())
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::server_with_config(
        // state is created once per worker thread
        async || {
            Ok(AppState {
                count: Rc::new(Cell::new(0)),
            })
        },
        async |_| {
            web::App::new()
                .route("/", web::route().to_with_state(show_count))
                .route("/add", web::route().to_with_state(add_one))
        },
    )
    .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
    .run()
    .await
}
// </data>
