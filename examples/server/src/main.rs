pub mod keep_alive;
pub mod keep_alive_tp;
pub mod signals;
pub mod ssl;
pub mod workers;

// <main>
use ntex::web;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(async |_| {
        web::App::new().route(
            "/",
            web::get().to(|| async { web::HttpResponse::Ok().build() }),
        )
    })
    .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
    .run()
    .await
}
// </main>
