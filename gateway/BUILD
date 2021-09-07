load("@rules_rust//rust:rust.bzl", "rust_binary")
load("//cargo:crates.bzl", "all_crate_deps", "crate_deps")

exports_files(["Cargo.toml"])

rust_binary(
    name = "gateway",
    srcs = glob(["src/**/*.rs"]),
    proc_macro_deps = crate_deps(["serde_repr"]),
    deps = all_crate_deps(),
    visibility = ["//visibility:public"],
)
