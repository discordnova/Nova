"""
Utilities used by the workspace to load the rust toolchain
"""
load("//cargo:crates.bzl", "raze_fetch_remote_crates")
load("@rules_rust//rust:repositories.bzl", "rust_repositories")

def load_rust_toolchains():
    """
    A simple macro that loads the rust toolchain
    """
    rust_repositories(
        edition = "2018",
        iso_date = "2021-06-16",
        version = "nightly",
    )

    raze_fetch_remote_crates()
