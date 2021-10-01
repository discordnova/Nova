gazelle:
	bazel run //:gazelle -- update-repos -build_file_name BUILD.bazel -from_file=go.mod -to_macro=deps.bzl%go_dependencies