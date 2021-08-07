# Ratelimiter

This is an implementation of the ratelimiting service described in the `proto/nova.ratelimit.v1.proto`.
The library is divied in two part, a Rust library, built as a static library, and a rust executable that implements
the rate limiting algorithm. A  FFI interface is exposed by the Rust static library for use in the nova-lite component.