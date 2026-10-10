use ntex::web;

#[rustfmt::skip]
#[ntex::main]
async fn main() -> std::io::Result<()> {

    web::HttpServer::new(async |_| {
// <cfg>
web::App::new().service(
    web::resource("/path").route(
        web::route()
            .guard(web::guard::Get())
            .guard(web::guard::Header("content-type", "text/plain"))
            .to(|| async { web::HttpResponse::Ok().build() }),
    ),
)
// </cfg>
    })
    .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
    .run()
    .await
}
