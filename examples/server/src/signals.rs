// <signals>
use ntex::web;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let srv = web::HttpServer::new(|| {
        web::App::new().route(
            "/",
            web::get().to(|| async { web::HttpResponse::Ok().finish() }),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .shutdown_timeout(ntex::time::Seconds(60)) // <- Set shutdown timeout to 60 seconds
    .run();

    // pause accepting new connections
    srv.pause().await;

    // resume accepting new connections
    srv.resume().await;

    // stop server gracefully
    srv.stop(true).await;

    Ok(())
}
// </signals>

#[allow(dead_code)]
fn run_main() {
    let _ = main();
}
