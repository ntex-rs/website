// <guard2>
use ntex::web;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(async |_| {
        web::App::new().route(
            "/",
            web::route()
                .guard(web::guard::Not(web::guard::Get()))
                .to(|| async { web::HttpResponse::MethodNotAllowed().build() }),
        )
    })
    .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
    .run()
    .await
}
// </guard2>
