---
title: Connection Lifecycle
---

# Architecture overview

After Server has started listening to all sockets, [`Accept`][accept] and [`Worker`][worker] are two main loops responsible for processing incoming client connections.

Once connection accepted Application level protocol processing happens in a protocol specific [`Dispatcher`][dispatcher] loop spawned from [`Worker`][worker].

    Please note, below diagrams are outlining happy-path scenarios only.

![](/img/diagrams/connection_overview.svg)

## Accept loop in more detail

![](/img/diagrams/connection_accept.svg)

Most of code implementation resides in [`ntex`][server] crate for struct [`Accept`][accept].

## Worker loop in more detail

![](/img/diagrams/connection_worker.svg)

Most of code implementation resides in [`ntex`][server] crate for struct [`Worker`][worker].

## Request loop roughly

![](/img/diagrams/connection_request.svg)

Most of code implementation for request loop resides in [`ntex`][ntex] crates.

[server]: https://crates.io/crates/ntex
[ntex]: https://crates.io/crates/ntex
[http]: https://crates.io/crates/ntex
[accept]: https://github.com/ntex-rs/ntex/blob/master/ntex/src/accept.rs
[worker]: https://github.com/ntex-rs/ntex/blob/master/ntex/src/worker.rs
[dispatcher]: https://github.com/ntex-rs/ntex/blob/master/ntex-h1/src/h1/dispatcher.rs
