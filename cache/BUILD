load("@rules_rust//rust:rust.bzl", "rust_binary", "rust_test")
load("//cargo:crates.bzl", "all_crate_deps")
load("@io_bazel_rules_docker//rust:image.bzl", "rust_image")

test_suite(name = "tests")

rust_binary(
    name = "cache",
    srcs = glob(["src/**"]),
    visibility = ["//visibility:public"],
    deps = all_crate_deps() + ["//common/rust:common"],
)

rust_test(
    name = "cache_test",
    crate = ":cache",
)

rust_image(
    name = "image",
    base = "//bazel:base",
    binary = ":cache",
    visibility = ["//visibility:public"],
)
