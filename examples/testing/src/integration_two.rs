use ntex::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct AppState {
    count: i32,
}

#[allow(dead_code)]
async fn index(data: web::types::State<AppState>) -> impl web::Responder {
    web::HttpResponse::Ok().json(data.get_ref())
}

// <integration-two>
#[cfg(test)]
mod tests {
    use super::*;
    use ntex::web;
    use ntex::web::test;

    #[ntex::test]
    async fn test_index_get() {
        let app = test::init_service(
            web::App::new()
                .state(AppState { count: 4 })
                .route("/", web::get().to(index)),
        )
        .await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp: AppState = test::read_response_json(&app, req).await;

        assert_eq!(resp.count, 4);
    }
}
// </integration-two>
