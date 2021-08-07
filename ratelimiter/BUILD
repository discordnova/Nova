load("@rules_rust//cargo:cargo_build_script.bzl", "cargo_build_script")
load("@rules_rust//rust:rust.bzl", "rust_binary")
load("@bazel_tools//tools/build_defs/pkg:pkg.bzl", "pkg_tar")

cargo_build_script(
    name = "build_script",
    srcs = ["build.rs"],
    deps = [
        "//ratelimiter/cargo:tonic_build",
    ],
)

rust_binary(
    name = "ratelimiter",
    srcs = ["src/main.rs"],
)

# TODO: Add testing, and the shared library.

pkg_tar(
    name = "ratelimiter_pkg",
    srcs = [":ratelimiter"],
    mode = "0755",
    visibility = ["//visibility:public"],
)
