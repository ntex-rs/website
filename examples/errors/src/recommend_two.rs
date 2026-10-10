// <recommend-two>
use derive_more::{Display, Error};
use ntex::http;
use ntex::web;

#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
}

impl<St> web::error::WebResponseError<St, web::DefaultError> for UserError {
    fn error_response(&self, _: &St) -> web::HttpResponse {
        web::HttpResponse::builder(self.status_code())
            .set_header("content-type", "text/html; charset=utf-8")
            .body(self.to_string())
    }
}

impl UserError {
    fn status_code(&self) -> http::StatusCode {
        match *self {
            UserError::InternalError => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[web::get("/")]
async fn index() -> Result<&'static str, UserError> {
    do_thing_that_fails().map_err(|_e| UserError::InternalError)?;
    Ok("success!")
}
// </recommend-two>

fn do_thing_that_fails() -> Result<(), std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "some error"))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(async |_| web::App::new().service(index))
        .bind(("127.0.0.1", 8080), ntex::SharedCfg::default())?
        .run()
        .await
}
