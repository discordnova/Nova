"""
"""
load("@io_bazel_rules_docker//toolchains/docker:toolchain.bzl", "toolchain_configure")
load("@io_bazel_rules_docker//repositories:repositories.bzl", "repositories")
load("@io_bazel_rules_docker//repositories:deps.bzl", "deps")
load("@io_bazel_rules_docker//container:container.bzl", "container_pull")
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
    container_pull(
        name = "io_docker_index_library_debian_stable_slim",
        digest = "sha256:7bb9de2067f4e4e3e2377070e180a05d33a0bc4289c66b9e71504063cf18da15",
        registry = "index.docker.io",
        repository = "library/debian",
        tag = "stable-slim",
    )