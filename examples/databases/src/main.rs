use std::io;

use diesel::{prelude::*, r2d2, Insertable, Queryable, SqliteConnection};
use ntex::web;
use serde::Serialize;

mod schema {
    diesel::table! {
        users {
            id -> VarChar,
            name -> VarChar,
        }
    }
}

#[derive(Debug, Serialize, Queryable)]
struct User {
    id: String,
    name: String,
}

// <handler>
#[derive(Debug, Insertable)]
#[diesel(table_name = self::schema::users)]
struct NewUser<'a> {
    id: &'a str,
    name: &'a str,
}

fn insert_new_user(
    conn: &mut SqliteConnection,
    user_name: String,
) -> diesel::QueryResult<User> {
    use crate::schema::users::dsl::*;

    // Create insertion model
    let uid = format!("{}", uuid::Uuid::new_v4());
    let new_user = NewUser {
        id: &uid,
        name: &user_name,
    };

    // normal diesel operations
    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .expect("Error inserting person");

    let user = users
        .filter(id.eq(&uid))
        .first::<User>(conn)
        .expect("Error loading person that was just inserted");

    Ok(user)
}
// </handler>

// <main>
type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[ntex::main]
async fn main() -> io::Result<()> {
    // connect to SQLite DB
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new("app.db");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file");

    // start HTTP server on port 8080
    web::HttpServer::new(move || {
        web::App::new()
            .state(pool.clone())
            .route("/{name}", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
// </main>

// <index>
async fn index(
    pool: web::types::State<DbPool>,
    name: web::types::Path<(String,)>,
) -> Result<impl web::Responder, web::Error> {
    let (name,) = name.into_inner();
    let pool = pool.get_ref().clone();

    let user = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        insert_new_user(&mut conn, name)
    })
    .await
    .map_err(web::error::ErrorInternalServerError)?;

    Ok(web::HttpResponse::Ok().json(&user))
}
// </index>
