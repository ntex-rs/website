// <guard>
use ntex::http;
use ntex::web;

struct ContentTypeHeader;

impl web::guard::Guard for ContentTypeHeader {
    fn check(&self, req: &http::RequestHead) -> bool {
        req.headers().contains_key(http::header::CONTENT_TYPE)
    }
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new().route(
            "/",
            web::route()
                .guard(ContentTypeHeader)
                .to(|| async { web::HttpResponse::Ok().finish() }),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </guard>
