use ntex::web;

#[rustfmt::skip]
#[ntex::main]
async fn main() -> std::io::Result<()> {

    web::HttpServer::new(|| {
// <cfg>
web::App::new().service(
    web::resource("/path").route(
        web::route()
            .guard(web::guard::Get())
            .guard(web::guard::Header("content-type", "text/plain"))
            .to(|| async { web::HttpResponse::Ok().finish() }),
    ),
)
// </cfg>
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
