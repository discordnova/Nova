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
        digest = "sha256:2c4bb6b7236db0a55ec54ba8845e4031f5db2be957ac61867872bf42e56c4deb",
        registry = "gcr.io",
        repository = "distroless/cc-debian10",
        tag = "latest",
    )
    container_pull(
        name = "ubuntu1604",
        registry = "l.gcr.io",
        repository = "google/ubuntu1604",
        tag = "latest",
    )
