use ntex::web;

#[allow(dead_code)]
async fn index(_req: web::HttpRequest) -> impl web::Responder {
    "Hello world!"
}

// <integration-one>
#[cfg(test)]
mod tests {
    use ntex::web;
    use ntex::web::test;

    use super::*;

    #[ntex::test]
    async fn test_index_get() {
        let app = test::init_service(web::App::new().route("/", web::get().to(index))).await;
        let req = test::TestRequest::default()
            .header("content-type", "text/plain")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[ntex::test]
    async fn test_index_post() {
        let app = test::init_service(web::App::new().route("/", web::get().to(index))).await;
        let req = test::TestRequest::post().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }
}
// </integration-one>
