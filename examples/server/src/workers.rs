// <workers>
use ntex::web;

#[ntex::main]
async fn main() {
    let _server = web::HttpServer::new(async |_| {
        web::App::new().route(
            "/",
            web::get().to(|| async { web::HttpResponse::Ok().build() }),
        )
    })
    .workers(4);
    // <- Start 4 workers
}
// </workers>
