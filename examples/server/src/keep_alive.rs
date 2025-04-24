// <keep-alive>
use ntex::web;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let app_factory = web::App::new;
    // Set keep-alive to 75 seconds
    let _one =
        web::HttpServer::new(app_factory).keep_alive(ntex::time::Seconds(75));

    // Use OS's keep-alive (usually quite long)
    let _two =
        web::HttpServer::new(app_factory).keep_alive(ntex::http::KeepAlive::Os);

    // Disable keep-alive
    let _three = web::HttpServer::new(app_factory).keep_alive(None);

    Ok(())
}
// </keep-alive>
