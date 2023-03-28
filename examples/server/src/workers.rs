// <workers>
use ntex::web;

#[ntex::main]
async fn main() {
    web::HttpServer::new(|| {
        web::App::new().route(
            "/",
            web::get().to(|| async { web::HttpResponse::Ok().finish() }),
        )
    })
    .workers(4);
    // <- Start 4 workers
}
// </workers>
