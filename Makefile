gazelle:
	bazel run //:gazelle -- update-repos -from_file=go.mod -to_macro=//bazel:deps.bzl%go_dependencies