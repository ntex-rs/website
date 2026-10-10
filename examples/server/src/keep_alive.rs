// <keep-alive>
use ntex::{http, time::Seconds, web, SharedCfg};

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let app = async |_: &()| web::App::new().route("/", web::get().to(async || "Hello"));

    // Set keep-alive to 75 seconds
    let _one = web::HttpServer::new(app).bind(
        ("127.0.0.1", 8080),
        SharedCfg::new("MY-SERVER").add(
            http::HttpServiceConfig::new()
                .set_keepalive(http::KeepAlive::Timeout(Seconds(75))),
        ),
    )?;

    // Use OS's keep-alive (usually quite long)
    let _two = web::HttpServer::new(app).bind(
        ("127.0.0.1", 8081),
        SharedCfg::new("MY-SERVER")
            .add(http::HttpServiceConfig::new().set_keepalive(http::KeepAlive::Os)),
    )?;

    // Disable keep-alive
    let _three = web::HttpServer::new(app).bind(
        ("127.0.0.1", 8082),
        SharedCfg::new("MY-SERVER")
            .add(http::HttpServiceConfig::new().set_keepalive(http::KeepAlive::Disabled)),
    )?;

    Ok(())
}
// </keep-alive>
