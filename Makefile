
# Updates the deps.bzl file using the go.mod
.PHONY: update-deps
gazzle-update-deps:
	bazel run //:gazelle -- update-repos -from_file=go.mod -to_macro=deps.bzl%go_dependencies

.PHONY: gazzle-sync
gazzle-sync:
	bazel run //:gazelle
