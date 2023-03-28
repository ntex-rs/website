// <helpers>
use ntex::web;

#[derive(Debug)]
struct MyError {
    name: &'static str,
}

#[web::get("/")]
async fn index() -> Result<String, web::Error> {
    let result = Err(MyError { name: "test error" });

    result.map_err(|err| web::error::ErrorBadRequest(err.name).into())
}
// </helpers>

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
