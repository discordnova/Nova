load("@bazel_gazelle//:def.bzl", "gazelle")
load("@rules_pkg//:pkg.bzl", "pkg_zip", "pkg_tar")

# gazelle:prefix github.com/discordnova/nova
gazelle(name = "gazelle")

filegroup(
    name = "package_bin",
    srcs = [
        "//novactl",
    ] + select({
        "@bazel_tools//src/conditions:windows": [],
        "//conditions:default": ["//webhook", "//gateway", "//ratelimiter"],
    }),
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