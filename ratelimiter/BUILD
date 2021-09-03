load("@rules_rust//rust:rust.bzl", "rust_binary")
load("@rules_rust//cargo:cargo_build_script.bzl", "cargo_build_script")
load("@crates//:defs.bzl", "build_crates_from", "crates_from")
load("@bazel_tools//tools/build_defs/pkg:pkg.bzl", "pkg_tar")

exports_files(["Cargo.toml"])

cargo_build_script(
    name = "build_script",
    srcs = ["build.rs"],
    deps = build_crates_from("Cargo.toml"),
    data = [
        "//ratelimiter/proto:nova.ratelimit.v1.proto",
    ] + [
        "@com_google_protobuf//:protoc",
        "@com_google_protobuf//:protobuf_headers"
    ],
    outs = glob(["**"]),
    build_script_env = {
        "PROTOC": "$(location @com_google_protobuf//:protoc)",
    }
)

rust_binary(
    name = "ratelimiter",
    srcs = ["src/main.rs"],
    rustc_env = {
        "OUT_DIR": "$(location :build_script)"
    },
    deps = crates_from("Cargo.toml") + [":build_script"]
)

pkg_tar(
    name = "ratelimiter_pkg",
    srcs = [":webhook"],
    mode = "0755",
    visibility = ["//visibility:public"],
)
