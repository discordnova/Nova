load("@rules_rust//rust:rust.bzl", "rust_binary")
load("//cargo:crates.bzl", "all_crate_deps", "crate_deps")
load("@io_bazel_rules_docker//rust:image.bzl", "rust_image")

test_suite(name = "tests")

rust_binary(
    name = "gateway",
    srcs = glob(["src/**/*.rs"]),
    visibility = ["//visibility:public"],
    deps = all_crate_deps() + ["//common/rust:common"],
)

rust_image(
    name = "image",
    base = "//bazel:base",
    binary = ":gateway",
    visibility = ["//visibility:public"],
)
