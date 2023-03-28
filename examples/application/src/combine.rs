#![allow(dead_code)]

use ntex::web;

// <combine>
struct State1;
struct State2;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .service(
                web::scope("/app1")
                    .state(State1)
                    .route("/", web::to(|| async { web::HttpResponse::Ok().finish() })),
            )
            .service(
                web::scope("/app2")
                    .state(State2)
                    .route("/", web::to(|| async { web::HttpResponse::Ok().finish() })),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </combine>
