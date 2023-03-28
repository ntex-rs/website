#![allow(dead_code)]

// <user-session>
use ntex::web;
use ntex::web::middleware::Logger;
use ntex_session::{CookieSession, Session};

/// simple index handler with session
#[web::get("/")]
async fn index(session: Session, req: web::HttpRequest) -> Result<&'static str, web::Error> {
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

    web::server(|| {
        web::App::new()
            // enable logger
            .wrap(Logger::default())
            // cookie session middleware
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// </user-session>
