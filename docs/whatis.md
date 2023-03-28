---
title: What is Ntex
---
import { rustVersion } from "@site/vars";

# Ntex is part of an Ecosystem of Crates

Ntex is a framework for composable network services.

We call Ntex a powerful and pragmatic framework. For all intents and purposes it's a micro-framework with a few twists. If you are already a Rust programmer you will probably find yourself at home quickly, but even if you are coming from another programming language you should find Ntex easy to pick up.

An application developed with Ntex will expose an HTTP server contained within a native executable. You can either put this behind another HTTP server like nginx or serve it up as-is. Even in the complete absence of another HTTP server Ntex is powerful enough to provide HTTP/1 and HTTP/2 support as well as TLS (HTTPS). This makes it useful for building small services ready for production.

Ntex have several other implementations such as:

- [ntex-grpc][ntex-grpc] A GRPC Client/Server framework
- [ntex-redis][ntex-redis] A redis client
- [ntex-mqtt][ntex-mqtt] A MQTT Client/Server framework for v5 and v3.1.1 protocols

Starting ntex v0.5 async runtime must be selected as a feature, the options are:

- [Tokio][tokio] is an asynchronous runtime for the Rust programming language. It provides the building blocks needed for writing network applications. It gives the flexibility to target a wide range of systems, from large servers with dozens of cores to small embedded devices. 
- [Glommio][glommio] is a thread-per-core crate that makes writing highly parallel asynchronous applications in a thread-per-core architecture easier for rustaceans. Powered by DataDog
- [Async Std][async-std] Async version of the Rust standard library

<p>
Most importantly: Ntex runs on Rust { rustVersion } or later and it works with stable releases.
</p>

<!-- LINKS -->


[ntex-grpc]: https://github.com/ntex-rs/ntex-grpc
[ntex-redis]: https://github.com/ntex-rs/ntex-redis
[ntex-mqtt]: https://github.com/ntex-rs/ntex-mqtt
[tokio]: https://github.com/tokio-rs/tokio
[glommio]: https://github.com/DataDog/glommio
[async-std]: https://github.com/async-rs/async-std
