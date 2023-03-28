use std::io;

// <chunked>
use futures::future::ok;
use futures::stream::once;
use ntex::util::Bytes;
use ntex::web;

#[web::get("/")]
async fn index(_req: web::HttpRequest) -> web::HttpResponse {
    web::HttpResponse::Ok().streaming(once(ok::<_, web::Error>(Bytes::from_static(b"data"))))
}
// </chunked>

#[ntex::main]
async fn main() -> io::Result<()> {
    web::HttpServer::new(|| web::App::new().service(index))
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run()
        .await
}
