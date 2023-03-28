// <override>
use derive_more::{Display, Error};
use ntex::http;
use ntex::web;

#[derive(Debug, Display, Error)]
enum MyError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}

impl web::error::WebResponseError for MyError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        web::HttpResponse::build(self.status_code())
            .set_header("content-type", "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> http::StatusCode {
        match *self {
            MyError::InternalError => http::StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => http::StatusCode::BAD_REQUEST,
            MyError::Timeout => http::StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

#[web::get("/")]
async fn index() -> Result<&'static str, MyError> {
    Err(MyError::BadClientData)
}
// </override>

#[web::get("/e2")]
async fn error2() -> Result<&'static str, MyError> {
    Err(MyError::InternalError)
}

#[web::get("/e3")]
async fn error3() -> Result<&'static str, MyError> {
    Err(MyError::Timeout)
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .service(index)
            .service(error2)
            .service(error3)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
