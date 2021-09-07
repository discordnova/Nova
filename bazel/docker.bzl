"""
"""
load("@io_bazel_rules_docker//toolchains/docker:toolchain.bzl", "toolchain_configure")
load("@io_bazel_rules_docker//repositories:repositories.bzl", "repositories")
load("@io_bazel_rules_docker//repositories:deps.bzl", "deps")
load(
    "@io_bazel_rules_docker//go:image.bzl",
    _go_image_repos = "repositories",
)
load(
    "@io_bazel_rules_docker//rust:image.bzl",
    _rust_image_repos = "repositories",
)


def load_docker():
    """
    Loads all the docker credentials and pull the needed images
    """
    toolchain_configure(
        name = "docker_config"
    )
    repositories()
    deps()
    _go_image_repos()
    _rust_image_repos()