// <urlencoded>
use ntex::web;
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    username: String,
}

#[web::post("/")]
async fn index(form: web::types::Form<FormData>) -> web::HttpResponse {
    web::HttpResponse::Ok().body(format!("username: {}", form.username))
}
// </urlencoded>

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
