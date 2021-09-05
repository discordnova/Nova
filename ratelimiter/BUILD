load("@rules_rust//rust:rust.bzl", "rust_binary")
load("@rules_rust//cargo:cargo_build_script.bzl", "cargo_build_script")
load("@crates//:defs.bzl", "build_crates_from", "crates_from")

exports_files(["Cargo.toml"])

cargo_build_script(
    name = "build_script",
    srcs = ["build.rs"],
    build_script_env = {
        "PROTOC": "$(location @com_google_protobuf//:protoc)",
        "PROTO": "$(location //ratelimiter/proto:nova.ratelimit.v1.proto)"
    },
    data = [
        "//ratelimiter/proto:nova.ratelimit.v1.proto",
    ] + [
        "@com_google_protobuf//:protoc",
        "@com_google_protobuf//:protobuf_headers",
    ],
    deps = build_crates_from("Cargo.toml"),
)

rust_binary(
    name = "ratelimiter",
    srcs = ["src/main.rs"],
    rustc_env = {},
    deps = crates_from("Cargo.toml") + [":build_script"],
    visibility = ["//visibility:public"],
)
