"""
"""
load("@io_bazel_rules_docker//toolchains/docker:toolchain.bzl", "toolchain_configure")
load("@io_bazel_rules_docker//repositories:repositories.bzl", "repositories")
load("@io_bazel_rules_docker//repositories:deps.bzl", "deps")
load("@io_bazel_rules_docker//container:container.bzl", "container_pull", "container_image")
load("@io_bazel_rules_docker//docker/package_managers:download_pkgs.bzl", "download_pkgs")
load("@io_bazel_rules_docker//docker/package_managers:install_pkgs.bzl", "install_pkgs")

load(
    "@io_bazel_rules_docker//go:image.bzl",
    _go_image_repos = "repositories",
)
load(
    "@io_bazel_rules_docker//rust:image.bzl",
    _rust_image_repos = "repositories",
)

def images():
    download_pkgs(
        name = "download_base_pkgs",
        image_tar = "@debian//image",
        packages = ["libgcc-10-dev", "libc6"]
    )
    install_pkgs(
        name = "base_pkgs",
        image_tar = "@debian//image",
        installables_tar = ":download_base_pkgs.tar",
        output_image_name = "distroless_base_with_libgcc"
    )
    container_image(
        name = "base",
        base = ":base_pkgs.tar",
        visibility = ["//visibility:public"],
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
        name = "distroless_debian10",
        registry = "gcr.io",
        repository = "distroless/base-debian10",
        tag = "latest",
    )
    container_pull(
        name = "debian",
        registry = "docker.io",
        repository = "library/debian",
        tag = "stable",
    )