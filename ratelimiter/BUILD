load("@rules_rust//rust:rust.bzl", "rust_binary")
load("@rules_rust//cargo:cargo_build_script.bzl", "cargo_build_script")
load("//cargo:crates.bzl", "all_crate_deps", "crate_deps")
load("@io_bazel_rules_docker//rust:image.bzl", "rust_image")

test_suite(name = "tests")

cargo_build_script(
    name = "build_script",
    srcs = ["build.rs"],
    build_script_env = {
        "PROTOC": "$(location @com_google_protobuf//:protoc)",
    },
    data = [
        "//ratelimiter/proto:nova.ratelimit.v1.proto",
    ] + [
        "@com_google_protobuf//:protoc",
        "@com_google_protobuf//:protobuf_headers",
    ],
    deps = all_crate_deps() + crate_deps(["tonic-build"]),
)

rust_binary(
    name = "ratelimiter",
    srcs = ["src/main.rs"],
    deps = all_crate_deps() + [":build_script"],
    visibility = ["//visibility:public"],
)

rust_image(
    name = "image",
    srcs = ["src/main.rs"],
    deps = all_crate_deps() + [":build_script"],
    visibility = ["//visibility:public"],
)