// <stream-response>
use std::task::Poll;

use futures::stream;
use ntex::http;
use ntex::util::Bytes;
use ntex::web;

async fn sse(_req: web::HttpRequest) -> web::HttpResponse {
    let mut counter: usize = 5;

    // yields `data: N` where N in [5; 1]
    let server_events =
        stream::poll_fn(move |_cx| -> Poll<Option<Result<Bytes, web::Error>>> {
            if counter == 0 {
                return Poll::Ready(None);
            }
            let payload = format!("data: {}\n\n", counter);
            counter -= 1;
            Poll::Ready(Some(Ok(Bytes::from(payload))))
        });

    web::HttpResponse::build(http::StatusCode::OK)
        .set_header(http::header::CONTENT_TYPE, "text/event-stream")
        .set_header(http::header::CONTENT_ENCODING, "identity")
        .streaming(server_events)
}

pub fn main() {
    web::App::new().route("/", web::get().to(sse));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ntex::util::BytesMut;
    use ntex::web::test;

    use futures::{future, Stream, StreamExt};

    #[ntex::test]
    async fn test_stream_chunk() {
        let app = test::init_service(web::App::new().route("/", web::get().to(sse))).await;
        let req = test::TestRequest::get().to_request();

        let mut resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let mut body = Box::pin(resp.take_body().into_body::<Bytes>());

        // first chunk
        let bytes = future::poll_fn(|cx| body.as_mut().poll_next(cx)).await;
        assert_eq!(bytes.unwrap().unwrap(), Bytes::from_static(b"data: 5\n\n"));

        // second chunk
        let bytes = future::poll_fn(|cx| body.as_mut().poll_next(cx)).await;
        assert_eq!(bytes.unwrap().unwrap(), Bytes::from_static(b"data: 4\n\n"));

        // remaining part
        for i in 0..3 {
            let expected_data = format!("data: {}\n\n", 3 - i);
            let bytes = future::poll_fn(|cx| body.as_mut().poll_next(cx)).await;
            assert_eq!(bytes.unwrap().unwrap(), Bytes::from(expected_data));
        }
    }

    #[ntex::test]
    async fn test_stream_full_payload() {
        let app = test::init_service(web::App::new().route("/", web::get().to(sse))).await;
        let req = test::TestRequest::get().to_request();

        let mut resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = resp.take_body().into_body::<Bytes>();
        let bytes = body
            .fold(BytesMut::new(), |mut acc, chunk| async move {
                acc.extend_from_slice(&chunk.unwrap());
                acc
            })
            .await;
        assert_eq!(
            bytes,
            Bytes::from_static(b"data: 5\n\ndata: 4\n\ndata: 3\n\ndata: 2\n\ndata: 1\n\n")
        );
    }
}
// </stream-response>
