
# Updates the deps.bzl file using the go.mod
.PHONY: update-deps
gazelle-update-deps:
	bazel run //:gazelle -- update-repos -from_file=go.mod -to_macro=deps.bzl%go_dependencies

.PHONY: gazzle-sync
gazelle-sync:
	bazel run //:gazelle
