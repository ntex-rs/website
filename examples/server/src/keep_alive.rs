// <keep-alive>
use ntex::{http, time::Seconds, web};

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let app_factory = async || web::App::new();
    // Set keep-alive to 75 seconds
    let _one =
        web::HttpServer::new(app_factory).config(
        ntex::SharedCfg::new("MY-SERVER")
            .add(http::HttpServiceConfig::new().set_keepalive(http::KeepAlive::Timeout(Seconds(75)))),
    );

    // Use OS's keep-alive (usually quite long)
    let _two =
        web::HttpServer::new(app_factory).config(
        ntex::SharedCfg::new("MY-SERVER")
            .add(http::HttpServiceConfig::new().set_keepalive(http::KeepAlive::Os)),
    );

    // Disable keep-alive
    let _three = web::HttpServer::new(app_factory).config(
        ntex::SharedCfg::new("MY-SERVER")
            .add(http::HttpServiceConfig::new().set_keepalive(http::KeepAlive::Disabled)),
    );

    Ok(())
}
// </keep-alive>
