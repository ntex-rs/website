#![allow(dead_code)]

// <user-session>
use ntex::web;
use ntex::web::middleware::Logger;
use ntex_session::{CookieSession, Session};

/// simple index handler with session
#[web::get("/")]
async fn index(session: Session, req: web::HttpRequest) -> Result<&'static str, web::error::JsonError> {
    println!("{:?}", req);

    // RequestSession trait is used for session access
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
        session.set("counter", counter)?;
    } else {
        session.set("counter", counter)?;
    }

    Ok("welcome!")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    println!("Starting http server: 127.0.0.1:8080");

    web::server(async |_| {
        web::App::new()
            // enable logger
            .middleware(Logger::default())
            // cookie session middleware
            .middleware(CookieSession::signed(&[0; 32]).secure(false))
            .service(index)
    })
    .bind("127.0.0.1:8080", ntex::SharedCfg::default())?
    .run()
    .await
}
// </user-session>
