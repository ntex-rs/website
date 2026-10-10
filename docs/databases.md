---
title: Databases
---

import CodeBlock from "@site/src/components/code_block.js";

# Async Options

We have several example projects showing use of async database adapters:

- R2D2: https://github.com/ntex-rs/examples/tree/master/r2d2
- Diesel: https://github.com/ntex-rs/examples/tree/master/diesel

# Diesel

The current versions of Diesel (v1/v2) does not support asynchronous operations, so it is important to use the [`web::block`][web-block] function to offload your database operations to the Ntex runtime thread-pool.

You can create action functions that correspond to all the operations your app will perform on the database.

<CodeBlock example="databases" file="main.rs" section="handler" />

Now you should set up the database pool using a crate such as `r2d2`, which makes many DB connections available to your app. This means that multiple handlers can manipulate the DB at the same time, and still accept new connections. Simply store the pool in your app state. The state factory is called for each worker, every worker gets a clone of the pool, and the pool handles shared access for you.

<CodeBlock example="databases" file="main.rs" section="main" />

Now, register the request handler with `to_with_state()`, the handler receives a reference to the app state as its first argument. Clone the pool and get a connection from it inside of a [`web::block`][web-block] closure. Then just call the action function with the necessary arguments and `.await` the result.

This example uses the `?` operator, the `BlockingError` type implements [`WebResponseError`][response-error] and is rendered as a _500_ (internal server error) response.

<CodeBlock example="databases" file="main.rs" section="index" />

That's it! See the full example here: https://github.com/ntex-rs/examples/tree/master/diesel

[web-block]: https://docs.rs/ntex/latest/ntex/web/fn.block.html
[response-error]: https://docs.rs/ntex/latest/ntex/web/error/trait.WebResponseError.html
