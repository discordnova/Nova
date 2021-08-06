
load("@rules_rust//cargo:cargo_build_script.bzl", "cargo_build_script")
load("@rules_rust//rust:rust.bzl", "rust_binary")

cargo_build_script(
    name = "build_script",
    srcs = ["build.rs"],
    deps = [
        "//rest-ratelimiter/cargo:tonic_build",
    ],
)

rust_binary(
    name = "rest-ratelimiter",
    srcs = ["src/main.rs"]
)