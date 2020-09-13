load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")

# Load bazel_skylib.
http_archive(
    name = "bazel_skylib",
    urls = [
        "https://github.com/bazelbuild/bazel-skylib/releases/download/1.0.3/bazel-skylib-1.0.3.tar.gz",
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.0.3/bazel-skylib-1.0.3.tar.gz",
    ],
    sha256 = "1c531376ac7e5a180e0237938a2536de0c54d93f5c278634818e0efc952dd56c",
)

load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")
bazel_skylib_workspace()

# Load rules_rust.
git_repository(
    name = "io_bazel_rules_rust",
    remote = "https://github.com/bazelbuild/rules_rust",
    commit = "7d9e890c58ca00eacf8dd4a2ba991cfe9c3f44e4",
    shallow_since = "1599678901 +0200",
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories()

load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")
bazel_version(name = "bazel_version")

# Load remote Cargo repositories.
load("//third_party/cargo:crates.bzl", "raze_fetch_remote_crates")

raze_fetch_remote_crates()