// <setup>
use ntex::web;

// This struct represents state
struct AppState {
    app_name: String,
}

#[web::get("/")]
async fn index(data: web::types::State<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}
// </setup>

// <start_app>
#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .state(AppState {
                app_name: String::from("Ntex"),
            })
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </start_app>
