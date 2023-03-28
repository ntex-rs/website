// <path>
use ntex::web;

#[web::get("/{username}/{id}/index.html")] // <- define path parameters
async fn index(info: web::types::Path<(String, u32)>) -> Result<String, web::Error> {
    let info = info.into_inner();
    Ok(format!("Welcome {}! id: {}", info.0, info.1))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
// </path>
