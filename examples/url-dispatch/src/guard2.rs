// <guard2>
use ntex::web;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new().route(
            "/",
            web::route()
                .guard(web::guard::Not(web::guard::Get()))
                .to(|| async { web::HttpResponse::MethodNotAllowed().finish() }),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </guard2>
