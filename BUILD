load("@bazel_gazelle//:def.bzl", "gazelle")
load("@bazel_tools//tools/build_defs/pkg:pkg.bzl", "pkg_tar")

# gazelle:prefix github.com/discordnova/nova
gazelle(name = "gazelle")

pkg_tar(
    name = "package",
    extension = "tar.gz",
    deps = [
        "//gateway:gateway_pkg",
        "//novactl:novactl_pkg",
        "//ratelimiter:ratelimiter_pkg",
        "//webhook:webhook_pkg",
    ],
)
