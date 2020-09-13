"""
@generated
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//third_party/cargo", which limits external
  # visibility to explicit Cargo.toml dependencies.
  "//visibility:public",
])

licenses([
  "notice", # BSD-3-Clause from expression "BSD-3-Clause"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)

load(
    "@io_bazel_rules_rust//cargo:cargo_build_script.bzl",
    "cargo_build_script",
)

cargo_build_script(
    name = "bindgen_build_script",
    srcs = glob(["**/*.rs"]),
    crate_root = "build.rs",
    edition = "2015",
    deps = [
        "@raze__clang_sys__0_22_0//:clang_sys",
    ],
    rustc_flags = [
        "--cap-lints=allow",
    ],
    crate_features = [
      "default",
      "env_logger",
      "log",
      "logging",
    ],
    build_script_env = {
    },
    data = glob(["**"]),
    tags = ["cargo-raze"],
    version = "0.35.0",
    visibility = ["//visibility:private"],
)

rust_binary(
    # Prefix bin name to disambiguate from (probable) collision with lib name
    # N.B.: The exact form of this is subject to change.
    name = "cargo_bin_bindgen",
    deps = [
        # Binaries get an implicit dependency on their crate's lib
        ":bindgen",
        ":bindgen_build_script",
        "@raze__cexpr__0_2_3//:cexpr",
        "@raze__cfg_if__0_1_10//:cfg_if",
        "@raze__clang_sys__0_22_0//:clang_sys",
        "@raze__clap__2_33_3//:clap",
        "@raze__env_logger__0_5_13//:env_logger",
        "@raze__lazy_static__1_4_0//:lazy_static",
        "@raze__log__0_4_6//:log",
        "@raze__peeking_take_while__0_1_2//:peeking_take_while",
        "@raze__quote__0_3_15//:quote",
        "@raze__regex__0_2_11//:regex",
        "@raze__which__1_0_5//:which",
    ],
    srcs = glob(["**/*.rs"]),
    crate_root = "src/main.rs",
    edition = "2015",
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.35.0",
    tags = ["cargo-raze"],
    crate_features = [
        "default",
        "env_logger",
        "log",
        "logging",
    ],
)


rust_library(
    name = "bindgen",
    crate_type = "lib",
    deps = [
        ":bindgen_build_script",
        "@raze__cexpr__0_2_3//:cexpr",
        "@raze__cfg_if__0_1_10//:cfg_if",
        "@raze__clang_sys__0_22_0//:clang_sys",
        "@raze__clap__2_33_3//:clap",
        "@raze__env_logger__0_5_13//:env_logger",
        "@raze__lazy_static__1_4_0//:lazy_static",
        "@raze__log__0_4_6//:log",
        "@raze__peeking_take_while__0_1_2//:peeking_take_while",
        "@raze__quote__0_3_15//:quote",
        "@raze__regex__0_2_11//:regex",
        "@raze__which__1_0_5//:which",
    ],
    srcs = glob(["**/*.rs"]),
    crate_root = "src/lib.rs",
    edition = "2015",
    rustc_flags = [
        "--cap-lints=allow",
    ],
    version = "0.35.0",
    tags = ["cargo-raze"],
    crate_features = [
        "default",
        "env_logger",
        "log",
        "logging",
    ],
)

