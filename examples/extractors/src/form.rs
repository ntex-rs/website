// <form>
use ntex::web;
use std::convert::Infallible;
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    username: String,
}

/// extract form data using serde
/// this handler gets called only if the content type is *x-www-form-urlencoded*
/// and the content of the request could be deserialized to a `FormData` struct
#[web::post("/")]
async fn index(form: web::types::Form<FormData>) -> Result<String, Infallible> {
    Ok(format!("Welcome {}!", form.username))
}
// </form>

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(async |_| web::App::new().service(index))
        .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
        .run()
        .await
}
