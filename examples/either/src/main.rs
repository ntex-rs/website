// <either>
use ntex::util::Either;
use ntex::web;

type RegisterResult = Either<web::HttpResponse, Result<&'static str, web::Error>>;

async fn index() -> RegisterResult {
    if is_a_variant() {
        // choose Left variant
        Either::Left(web::HttpResponse::BadRequest().body("Bad data"))
    } else {
        // choose Right variant
        Either::Right(Ok("Hello!"))
    }
}
// </either>

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().route("/", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

fn is_a_variant() -> bool {
    true
}
