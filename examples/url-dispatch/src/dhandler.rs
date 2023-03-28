use ntex::web;

#[allow(dead_code)]
async fn index(_req: web::HttpRequest) -> impl web::Responder {
    "Welcome!"
}

// <default>
#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .default_service(
                web::route()
                    .guard(web::guard::Not(web::guard::Get()))
                    .to(|| async { web::HttpResponse::MethodNotAllowed().finish() }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </default>
