load("@io_bazel_rules_go//go:def.bzl", "go_binary", "go_library")
load("@bazel_tools//tools/build_defs/pkg:pkg.bzl", "pkg_tar")

go_library(
    name = "gateway_lib",
    srcs = ["main.go"],
    importpath = "github.com/discordnova/nova/gateway",
    visibility = ["//visibility:public"],
    deps = [
        "//common",
        "//gateway/lib/gateway",
        "//gateway/lib/gateway/compression",
        "//gateway/lib/gateway/transporters",
        "@com_github_rs_zerolog//log",
    ],
)

go_binary(
    name = "gateway",
    embed = [":gateway_lib"],
    visibility = ["//visibility:public"],
)

pkg_tar(
    name = "gateway_pkg",
    srcs = [":gateway"],
    mode = "0755",
    visibility = ["//visibility:public"],
)
