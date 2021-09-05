load("@rules_rust//rust:rust.bzl", "rust_binary", "rust_library", "rust_test")
load("@crates//:defs.bzl", "crates_from")

exports_files(["Cargo.toml"])

rust_binary(
    name = "webhook",
    srcs = glob(["src/**"]),
    deps = crates_from("Cargo.toml"),
    visibility = ["//visibility:public"],
)

rust_test(
    name = "webhook_test",
    crate = ":webhook",
    deps = crates_from("Cargo.toml"),
)
