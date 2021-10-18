load("@rules_rust//rust:rust.bzl", "rust_binary", "rust_test")
load("//cargo:crates.bzl", "all_crate_deps", "crate_deps")
load("@io_bazel_rules_docker//rust:image.bzl", "rust_image")

test_suite(name = "tests")

rust_binary(
    name = "webhook",
    srcs = glob(["src/**"]),
    proc_macro_deps = crate_deps(["ctor"]),
    visibility = ["//visibility:public"],
    deps = all_crate_deps() + ["//common/rust:common"],
)

rust_test(
    name = "webhook_test",
    crate = ":webhook",
)

rust_image(
    name = "image",
    base = "//bazel:base",
    binary = ":webhook",
    visibility = ["//visibility:public"],
)
