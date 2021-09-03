load("@rules_rust//rust:rust.bzl", "rust_binary")
load("@crates//:defs.bzl", "crates_from")
load("@bazel_tools//tools/build_defs/pkg:pkg.bzl", "pkg_tar")

exports_files(["Cargo.toml"])

rust_binary(
    name = "webhook",
    srcs = glob(["src/**"]),
    deps = crates_from("Cargo.toml")
)

pkg_tar(
    name = "webhook_pkg",
    srcs = [":webhook"],
    mode = "0755",
    visibility = ["//visibility:public"],
)
