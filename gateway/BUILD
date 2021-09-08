load("@rules_rust//rust:rust.bzl", "rust_binary")
load("//cargo:crates.bzl", "all_crate_deps", "crate_deps")
load("@io_bazel_rules_docker//container:container.bzl", "container_image")
load("@io_bazel_rules_docker//rust:image.bzl", "rust_image")
load("@io_bazel_rules_docker//docker/package_managers:download_pkgs.bzl", "download_pkgs")
load("@io_bazel_rules_docker//docker/package_managers:install_pkgs.bzl", "install_pkgs")

test_suite(name = "tests")

rust_binary(
    name = "gateway",
    srcs = glob(["src/**/*.rs"]),
    proc_macro_deps = crate_deps(["serde_repr"]),
    deps = all_crate_deps(),
    visibility = ["//visibility:public"],
)

rust_image(
    name = "image",
    binary = ":gateway",
    visibility = ["//visibility:public"],
    base = "@ubuntu1604//image",
)
