"""
Loads the utilities for all the projects
"""
load("@com_google_protobuf//:protobuf_deps.bzl", "protobuf_deps")
load("@rules_pkg//:deps.bzl", "rules_pkg_dependencies")

def get_toolchain_utils_protocolbuffers():
    """
    Loads protocolbuffers
    """
    protobuf_deps()

def get_toolchain_utils_rules_pkg():
    """
    Load the utilities for packaging the build results
    """
    rules_pkg_dependencies()
