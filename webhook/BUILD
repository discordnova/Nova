load("@rules_rust//rust:rust.bzl", "rust_binary", "rust_test")
load("//cargo:crates.bzl", "all_crate_deps")
load("@io_bazel_rules_docker//rust:image.bzl", "rust_image")

exports_files(["Cargo.toml"])

rust_binary(
    name = "webhook",
    srcs = glob(["src/**"]),
    deps = all_crate_deps(),
    visibility = ["//visibility:public"],
)

rust_test(
    name = "webhook_test",
    crate = ":webhook",
    deps = all_crate_deps(),
)

rust_image(
    name = "image",
    srcs = glob(["src/**"]),
    deps = all_crate_deps(),
    visibility = ["//visibility:public"],
)