load("@bazel_gazelle//:def.bzl", "gazelle")
load("@rules_pkg//:pkg.bzl", "pkg_tar", "pkg_zip")
load("@io_bazel_rules_docker//container:container.bzl", "container_bundle")
load("@io_bazel_rules_docker//contrib:push-all.bzl", "container_push")

# gazelle:prefix github.com/discordnova/nova
gazelle(name = "gazelle")

exports_files(["go.mod"])

platform(
    name = "aarch64",
)

filegroup(
    name = "binaries",
    srcs = [
        "//cache",
        "//gateway",
        "//novactl",
        "//rest",
        "//webhook",
    ],
)

container_bundle(
    name = "container_images",
    images = {
        "ghcr.io/discordnova/nova/novactl:$(docker_tag)": "//novactl:image",
        "ghcr.io/discordnova/nova/gateway:$(docker_tag)": "//gateway:image",
        "ghcr.io/discordnova/nova/rest:$(docker_tag)": "//rest:image",
        "ghcr.io/discordnova/nova/webhook:$(docker_tag)": "//webhook:image",
        "ghcr.io/discordnova/nova/cache:$(docker_tag)": "//cache:image",
    },
)

container_push(
    name = "container_publish",
    bundle = ":container_images",
    format = "OCI",
)

pkg_tar(
    name = "packages_tar",
    srcs = [
        ":binaries",
    ],
    extension = "tar.gz",
)

pkg_zip(
    name = "packages_zip",
    srcs = [
        ":binaries",
    ],
)

filegroup(
    name = "packages",
    srcs = [
        ":packages_tar",
        ":packages_zip",
    ],
)
