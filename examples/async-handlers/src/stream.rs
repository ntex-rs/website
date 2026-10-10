// <stream>
use futures::{future::ok, stream::once};
use ntex::util::Bytes;
use ntex::web;

#[web::get("/stream")]
async fn stream() -> web::HttpResponse {
    let body = once(ok::<_, std::io::Error>(Bytes::from_static(b"test")));

    web::HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(async |_| web::App::new().service(stream))
        .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
        .run()
        .await
}
// </stream>
