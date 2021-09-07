load("@rules_rust//rust:rust.bzl", "rust_binary")
load("//cargo:crates.bzl", "all_crate_deps", "crate_deps")
load("@io_bazel_rules_docker//rust:image.bzl", "rust_image")

exports_files(["Cargo.toml"])

rust_binary(
    name = "gateway",
    srcs = glob(["src/**/*.rs"]),
    proc_macro_deps = crate_deps(["serde_repr"]),
    deps = all_crate_deps(),
    visibility = ["//visibility:public"],
)

rust_image(
    name = "image",
    srcs = glob(["src/**/*.rs"]),
    proc_macro_deps = crate_deps(["serde_repr"]),
    deps = all_crate_deps(),
    visibility = ["//visibility:public"],
    base = "@io_docker_index_library_debian_stable_slim//image",
)