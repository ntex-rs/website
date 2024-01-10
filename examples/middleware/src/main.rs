pub mod default_headers;
pub mod errorhandler;
pub mod logger;
pub mod user_sessions;

// <simple>
use ntex::service::{Middleware, Service, ServiceCtx};
use ntex::web;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct SayHi;

impl<S> Middleware<S> for SayHi {
    type Service = SayHiMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        SayHiMiddleware { service }
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, Err> Service<web::WebRequest<Err>> for SayHiMiddleware<S>
where
    S: Service<web::WebRequest<Err>, Response = web::WebResponse, Error = web::Error>,
    Err: web::ErrorRenderer,
{
    type Response = web::WebResponse;
    type Error = web::Error;

    ntex::forward_poll_ready!(service);

    async fn call<'a>(&self, req: web::WebRequest<Err>, ctx: ServiceCtx<'a, Self>) -> Result<Self::Response, Self::Error> {
        println!("Hi from start. You requested: {}", req.path());
        let res = ctx.call(&self.service, req).await?;
        println!("Hi from response");
        Ok(res)
    }
}
// </simple>

#[ntex::main]
async fn main() -> std::io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new().wrap(SayHi).service(
            web::resource("/").to(|| async {
                "Hello, middleware! Check the console where the server is run."
            }),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
