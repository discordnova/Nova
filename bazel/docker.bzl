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
        name = "io_docker_index_library_alpine",
        digest = "sha256:69704ef328d05a9f806b6b8502915e6a0a4faa4d72018dc42343f511490daf8a",
        registry = "index.docker.io",
        repository = "library/alpine",
        tag = "lastest",
    )