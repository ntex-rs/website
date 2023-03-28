// <default-headers>
use ntex::http;
use ntex::web;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .wrap(web::middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .service(
                web::resource("/test")
                    .route(web::get().to(|| async { web::HttpResponse::Ok().finish() }))
                    .route(
                        web::method(http::Method::HEAD)
                            .to(|| async { web::HttpResponse::MethodNotAllowed().finish() }),
                    ),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </default-headers>
