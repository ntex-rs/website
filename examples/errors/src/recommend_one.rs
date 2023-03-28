// <recommend-one>
use derive_more::{Display, Error};
use ntex::http;
use ntex::web;

#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
}

impl web::error::WebResponseError for UserError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        web::HttpResponse::build(self.status_code())
            .set_header("content-type", "text/html; charset=utf-8")
            .body(self.to_string())
    }
    fn status_code(&self) -> http::StatusCode {
        match *self {
            UserError::ValidationError { .. } => http::StatusCode::BAD_REQUEST,
        }
    }
}
// </recommend-one>

#[web::get("/")]
async fn index() -> Result<&'static str, UserError> {
    Err(UserError::ValidationError {
        field: "bad stuff".to_string(),
    })
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| web::App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
