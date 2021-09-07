load("@rules_rust//rust:rust.bzl", "rust_binary", "rust_library")
load("@crates//:defs.bzl", "crates_from")

exports_files(["Cargo.toml"])

rust_binary(
    name = "gateway",
    srcs = glob(["src/**/*.rs"]),
    rustc_env = {},
    deps = crates_from("Cargo.toml"),
    visibility = ["//visibility:public"],
)
