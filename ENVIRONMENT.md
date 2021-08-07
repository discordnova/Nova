# Development environment

Nova use Bazel as build manager. Bazel allows us to combine various languages (Rust, Go, ...) in a single pipeline while taking advantage of incremental compilation and remote compilation.

However, some steps are to be follow to update programs dependencies. Nova use Gazelle to generate `BUILD` files of Go projects and Cargo-Raze to generate `BUILD` files of Rust projects.

When you edit project dependencies, you must execute a command tu update these files.

## Go

To update dependencies of Go Project, execute :

```sh
bazel run //:gazelle
```

## Rust

To update or create Rust project, use this command in the folder of your project :

```sh
bazel run @cargo_raze//:raze -- --manifest-path=$(realpath Cargo.toml)
```

If this is a new project, make sure you have configured Cargo-Raze in your `Cargo.toml` and added it to sources in `WORKSPACE`.
