---
title: Errors
---

import CodeBlock from "@site/src/components/code_block.js";

# Errors

Ntex uses its own [`ntex::web::Error`][ntexerror] type and [`ntex::web::WebResponseError`][responseerror] trait for error handling from web handlers.

If a handler returns an `Error` (referring to the [general Rust trait `std::error::Error`][stderror]) in a `Result` that also implements the `WebResponseError` trait, ntex will render that error as an HTTP response with its corresponding [`ntex::web:StatusCode`][status_code]. An internal server error is generated by default:

```rust
pub trait WebResponseError {
    fn error_response(&self) -> Response<Body>;
    fn status_code(&self) -> StatusCode;
}
```

A `Responder` coerces compatible `Result`s into HTTP responses:

```rust
impl<T: Responder, E: Into<Error>> Responder for Result<T, E>
```

`Error` in the code above is ntex's error definition, and any errors that implement `WebResponseError` can be converted to one automatically.

Ntex provides `WebResponseError` implementations for some common non-ntex errors. For example, if a handler responds with an `io::Error`, that error is converted into an `HttpInternalServerError`:

```rust
use std::io;
use ntex_files::NamedFile;

fn index(_req: HttpRequest) -> io::Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}
```

See [the ntex API documentation][responseerrorimpls] for a full list of foreign implementations for `WebResponseError`.

## An example of a custom error response

Here's an example implementation for `WebResponseError`, using the [derive_more] crate for declarative error enums.

<CodeBlock example="errors" file="main.rs" section="response-error" />

`WebResponseError` has a default implementation for `error_response()` that will render a _500_ (internal server error), and that's what will happen when the `index` handler executes above.

Override `error_response()` to produce more useful results:

<CodeBlock example="errors" file="override_error.rs" section="override" />

## Error helpers

Ntex provides a set of error helper functions that are useful for generating specific HTTP error codes from other errors. Here we convert `MyError`, which doesn't implement the `WebResponseError` trait, to a _400_ (bad request) using `map_err`:

<CodeBlock example="errors" file="helpers.rs" section="helpers" />

See the [API documentation for ntex's `error` module][ntexerror] for a full list of available error helpers.

## Error logging

Ntex logs all errors at the `WARN` log level. If an application's log level is set to `DEBUG` and `RUST_BACKTRACE` is enabled, the backtrace is also logged. These are configurable with environmental variables:

```
>> RUST_BACKTRACE=1 RUST_LOG=ntex::web cargo run
```

The `Error` type uses the cause's error backtrace if available. If the underlying failure does not provide a backtrace, a new backtrace is constructed pointing to the point where the conversion occurred (rather than the origin of the error).

## Recommended practices in error handling

It might be useful to think about dividing the errors an application produces into two broad groups: those which are intended to be user-facing, and those which are not.

An example of the former is that I might use failure to specify a `UserError` enum which encapsulates a `ValidationError` to return whenever a user sends bad input:

<CodeBlock example="errors" file="recommend_one.rs" section="recommend-one" />

This will behave exactly as intended because the error message defined with `display` is written with the explicit intent to be read by a user.

However, sending back an error's message isn't desirable for all errors -- there are many failures that occur in a server environment where we'd probably want the specifics to be hidden from the user. For example, if a database goes down and client libraries start producing connect timeout errors, or if an HTML template was improperly formatted and errors when rendered. In these cases, it might be preferable to map the errors to a generic error suitable for user consumption.

Here's an example that maps an internal error to a user-facing `InternalError` with a custom message:

<CodeBlock example="errors" file="recommend_two.rs" section="recommend-two" />

By dividing errors into those which are user facing and those which are not, we can ensure that we don't accidentally expose users to errors thrown by application internals which they weren't meant to see.

## Error Logging

This is a basic example using `middleware::Logger` which depends on `env_logger` and `log`:

```toml
[dependencies]
env_logger = "0.8"
log = "0.4"
```

<CodeBlock example="errors" file="logging.rs" section="logging" />

[ntexerror]: https://docs.rs/ntex/latest/ntex/web/struct.Error.html
[errorhelpers]: https://docs.rs/ntex/latest/ntex/web/error/trait.WebResponseError.html
[derive_more]: https://crates.io/crates/derive_more
[responseerror]: https://docs.rs/ntex/latest/ntex/web/error/trait.WebResponseError.html
[responseerrorimpls]: https://docs.rs/ntex/latest/ntex/web/error/trait.WebResponseError.html#foreign-impls
[stderror]: https://doc.rust-lang.org/std/error/trait.Error.html
[status_code]: https://docs.rs/ntex/latest/ntex/web/struct.StatusCode.html
