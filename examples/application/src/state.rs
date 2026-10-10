#![allow(dead_code)]

// <setup>
use ntex::web;

// This struct represents state
#[derive(Clone)]
struct AppState {
    app_name: String,
}

// Application state type must implement `web::State` trait
impl web::State for AppState {
    type Error = web::DefaultError;
}

// Handlers registered with `to_with_state()` receive the application state
// and the request state as their first two arguments
async fn index(state: &AppState, _: ()) -> String {
    let app_name = &state.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}
// </setup>

// <start_app>
#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::server_with_config(
        // state factory, called once for each worker
        async || {
            Ok(AppState {
                app_name: String::from("ntex"),
            })
        },
        async |_| web::App::new().route("/", web::get().to_with_state(index)),
    )
    .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
    .run()
    .await
}
// </start_app>
