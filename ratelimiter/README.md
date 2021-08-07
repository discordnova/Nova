# Ratelimiter

This is an implementation of the rate limiting service described in the `proto/nova.ratelimit.v1.proto`.
The library is divided in two part, a Rust library, built as a static library, and a rust executable that implements
the rate limiting algorithm. A  FFI interface is exposed by the Rust static library for use in the nova-lite component.

TODO(n1c00o): Create great README with information
