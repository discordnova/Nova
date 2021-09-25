load("@bazel_gazelle//:def.bzl", "gazelle")
load("@rules_pkg//:pkg.bzl", "pkg_zip", "pkg_tar")
load("@io_bazel_rules_docker//container:container.bzl", "container_bundle")
load("@io_bazel_rules_docker//contrib:push-all.bzl", "container_push")

# gazelle:prefix github.com/discordnova/nova
gazelle(name = "gazelle")

filegroup(
    name = "binaries",
    srcs = [
        "//novactl",
        "//webhook",
        "//gateway",
        "//ratelimiter",
        "//cache"
    ]
)

container_bundle(
  name = "container_images",

  images = {
    "$(docker_repo)/discordnova/nova/novactl:$(docker_tag)": "//novactl:image",
    "$(docker_repo)/discordnova/nova/gateway:$(docker_tag)": "//gateway:image",
    "$(docker_repo)/discordnova/nova/ratelimiter:$(docker_tag)": "//ratelimiter:image",
    "$(docker_repo)/discordnova/nova/webhook:$(docker_tag)": "//webhook:image",
    "$(docker_repo)/discordnova/nova/cache:$(docker_tag)": "//cache:image",
  }
)

container_push(
  name = "container_publish",
  bundle = ":container_images",
  format = "OCI"
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

pkg_tar(
    name = "packages_tar",
    extension = "tar.gz",
    srcs = [
        ":binaries"
    ],
)

pkg_zip(
    name = "packages_zip",
    srcs = [
        ":binaries"
    ],
)

filegroup(
    name = "packages",
    srcs = [
        ":packages_zip",
        ":packages_tar",
    ],
)