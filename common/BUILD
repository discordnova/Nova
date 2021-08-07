load("@io_bazel_rules_go//go:def.bzl", "go_library")

go_library(
    name = "common",
    srcs = [
        "logger.go",
        "prometheus.go",
    ],
    importpath = "github.com/discordnova/nova/common",
    visibility = ["//visibility:public"],
    deps = [
        "@com_github_prometheus_client_golang//prometheus/promhttp",
        "@com_github_rs_zerolog//:zerolog",
        "@com_github_rs_zerolog//log",
    ],
)
