pub mod integration_one;
pub mod integration_two;
pub mod stream_response;
use ntex::http;
use ntex::web;

async fn index(req: web::HttpRequest) -> web::HttpResponse {
    if let Some(hdr) = req.headers().get(http::header::CONTENT_TYPE) {
        if let Ok(_s) = hdr.to_str() {
            return web::HttpResponse::Ok().into();
        }
    }
    web::HttpResponse::BadRequest().into()
}

fn main() {
    web::App::new().route("/", web::get().to(index));
}

// <unit-tests>
#[cfg(test)]
mod tests {
    use super::*;
    use ntex::web::test;

    #[ntex::test]
    async fn test_index_ok() {
        let req = test::TestRequest::default()
            .header("content-type", "text/plain")
            .to_http_request();
        let resp = index(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[ntex::test]
    async fn test_index_not_ok() {
        let req = test::TestRequest::default().to_http_request();
        let resp = index(req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }
}
// </unit-tests>
