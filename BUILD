load("@bazel_gazelle//:def.bzl", "gazelle")
load("@rules_pkg//:pkg.bzl", "pkg_zip", "pkg_tar")
load("@io_bazel_rules_docker//container:container.bzl", "container_bundle")
load("@io_bazel_rules_docker//contrib:push-all.bzl", "container_push")

# gazelle:prefix github.com/discordnova/nova
gazelle(name = "gazelle")

filegroup(
    name = "package_bin",
    srcs = [
        "//novactl",
        "//webhook",
        "//gateway",
        "//ratelimiter"
    ]
)

container_bundle(
  name = "bundle",

  images = {
    "$(docker_repo)/discordnova/nova/novactl:$(docker_tag)": "//novactl:image",
    "$(docker_repo)/discordnova/nova/gateway:$(docker_tag)": "//gateway:image",
    "$(docker_repo)/discordnova/nova/ratelimiter:$(docker_tag)": "//ratelimiter:image",
    "$(docker_repo)/discordnova/nova/webhook:$(docker_tag)": "//webhook:image",
  }
)

test_suite(
    name = "tests",
    tests = [
        "//novactl:tests",
        "//webhook:tests",
        "//gateway:tests",
        "//ratelimiter:tests"
    ],
)

container_push(
  name = "publish",
  bundle = ":bundle",
  format = "OCI"
)

pkg_tar(
    name = "package_tar",
    extension = "tar.gz",
    srcs = [
        ":package_bin"
    ],
)

pkg_zip(
    name = "package_zip",
    srcs = [
        ":package_bin"
    ],
)

filegroup(
    name = "package",
    srcs = [
        ":package_zip",
        ":package_tar",
    ],
)