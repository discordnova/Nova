"""
Utilities used by the workspace to load the golang toolchain
"""
load("//bazel:deps.bzl", "go_dependencies")
load("@io_bazel_rules_go//go:deps.bzl", "go_register_toolchains", "go_rules_dependencies")
load("@bazel_gazelle//:deps.bzl", "gazelle_dependencies", "go_repository")

def load_golang_toolchains():
    """
    Loads the golang toolchain
    """

    go_register_toolchains(version = "1.16.5")
    go_rules_dependencies()

    # Used to generate the go dependencies & build files
    go_dependencies()
    gazelle_dependencies()

    # Fixes a bug with the protobuf generation
    go_repository(
        name = "org_golang_google_grpc",
        build_file_proto_mode = "disable",
        importpath = "google.golang.org/grpc",
        sum = "h1:2dTRdpdFEEhJYQD8EMLB61nnrzSCTbG38PhqdhvOltg=",
        version = "v1.26.0",
    )
