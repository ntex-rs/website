use ntex::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct AppState {
    count: i32,
}

impl web::State for AppState {
    type Error = web::DefaultError;
}

#[allow(dead_code)]
async fn index(state: &AppState, _: ()) -> web::HttpResponse {
    web::HttpResponse::Ok().json(state)
}

// <integration-two>
#[cfg(test)]
mod tests {
    use super::*;
    use ntex::web;
    use ntex::web::test;

    #[ntex::test]
    async fn test_index_get() {
        let app = test::init_service_st(
            AppState { count: 4 },
            web::App::new().route("/", web::get().to_with_state(index)),
        )
        .await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp: AppState = test::read_response_json(&app, req).await;

        assert_eq!(resp.count, 4);
    }
}
// </integration-two>
