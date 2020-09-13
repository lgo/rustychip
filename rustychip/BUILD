load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "rustychip",
    srcs = [
        "cpu.rs",
        "display.rs",
        "keypad.rs",
        "loader.rs",
        "main.rs",
        "opcode.rs",
        "util.rs",
    ],
    deps = [
        "//third_party/cargo:sdl",
        "//third_party/cargo:getopts",
        "//third_party/cargo:time",
        "@rules_libsdl12//:libsdl12",
        "@rules_libsdl12//:libsdl12-main",
    ]
)