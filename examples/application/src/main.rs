use ntex::web;

pub mod app;
pub mod combine;
pub mod config;
pub mod scope;
pub mod state;
pub mod vh;

// <multi>
#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .service(
                web::scope("/app1")
                    .route("/", web::to(|| async { web::HttpResponse::Ok().finish() })),
            )
            .service(
                web::scope("/app2")
                    .route("/", web::to(|| async { web::HttpResponse::Ok().finish() })),
            )
            .route("/", web::to(|| async { web::HttpResponse::Ok().finish() }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </multi>
