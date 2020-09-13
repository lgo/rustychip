"""
@generated
cargo-raze crate workspace functions

DO NOT EDIT! Replaced on runs of cargo-raze
"""
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "new_git_repository")

def _new_http_archive(name, **kwargs):
    if not native.existing_rule(name):
        http_archive(name=name, **kwargs)

def _new_git_repository(name, **kwargs):
    if not native.existing_rule(name):
        new_git_repository(name=name, **kwargs)

def raze_fetch_remote_crates():

    _new_http_archive(
        name = "raze__aho_corasick__0_6_10",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/aho-corasick/aho-corasick-0.6.10.crate",
        type = "tar.gz",
        strip_prefix = "aho-corasick-0.6.10",
        build_file = Label("//third_party/cargo/remote:aho-corasick-0.6.10.BUILD"),
    )

    _new_http_archive(
        name = "raze__aho_corasick__0_7_13",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/aho-corasick/aho-corasick-0.7.13.crate",
        type = "tar.gz",
        strip_prefix = "aho-corasick-0.7.13",
        build_file = Label("//third_party/cargo/remote:aho-corasick-0.7.13.BUILD"),
    )

    _new_http_archive(
        name = "raze__ansi_term__0_11_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ansi_term/ansi_term-0.11.0.crate",
        type = "tar.gz",
        strip_prefix = "ansi_term-0.11.0",
        build_file = Label("//third_party/cargo/remote:ansi_term-0.11.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__atty__0_2_14",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/atty/atty-0.2.14.crate",
        type = "tar.gz",
        strip_prefix = "atty-0.2.14",
        build_file = Label("//third_party/cargo/remote:atty-0.2.14.BUILD"),
    )

    _new_http_archive(
        name = "raze__autocfg__1_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/autocfg/autocfg-1.0.1.crate",
        type = "tar.gz",
        strip_prefix = "autocfg-1.0.1",
        build_file = Label("//third_party/cargo/remote:autocfg-1.0.1.BUILD"),
    )

    _new_http_archive(
        name = "raze__bindgen__0_35_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bindgen/bindgen-0.35.0.crate",
        type = "tar.gz",
        strip_prefix = "bindgen-0.35.0",
        build_file = Label("//third_party/cargo/remote:bindgen-0.35.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__bitflags__0_7_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bitflags/bitflags-0.7.0.crate",
        type = "tar.gz",
        strip_prefix = "bitflags-0.7.0",
        build_file = Label("//third_party/cargo/remote:bitflags-0.7.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__bitflags__1_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/bitflags/bitflags-1.2.1.crate",
        type = "tar.gz",
        strip_prefix = "bitflags-1.2.1",
        build_file = Label("//third_party/cargo/remote:bitflags-1.2.1.BUILD"),
    )

    _new_http_archive(
        name = "raze__cc__1_0_59",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cc/cc-1.0.59.crate",
        type = "tar.gz",
        strip_prefix = "cc-1.0.59",
        build_file = Label("//third_party/cargo/remote:cc-1.0.59.BUILD"),
    )

    _new_http_archive(
        name = "raze__cexpr__0_2_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cexpr/cexpr-0.2.3.crate",
        type = "tar.gz",
        strip_prefix = "cexpr-0.2.3",
        build_file = Label("//third_party/cargo/remote:cexpr-0.2.3.BUILD"),
    )

    _new_http_archive(
        name = "raze__cfg_if__0_1_10",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cfg-if/cfg-if-0.1.10.crate",
        type = "tar.gz",
        strip_prefix = "cfg-if-0.1.10",
        build_file = Label("//third_party/cargo/remote:cfg-if-0.1.10.BUILD"),
    )

    _new_http_archive(
        name = "raze__clang_sys__0_22_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/clang-sys/clang-sys-0.22.0.crate",
        type = "tar.gz",
        strip_prefix = "clang-sys-0.22.0",
        build_file = Label("//third_party/cargo/remote:clang-sys-0.22.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__clap__2_33_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/clap/clap-2.33.3.crate",
        type = "tar.gz",
        strip_prefix = "clap-2.33.3",
        build_file = Label("//third_party/cargo/remote:clap-2.33.3.BUILD"),
    )

    _new_http_archive(
        name = "raze__env_logger__0_5_13",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/env_logger/env_logger-0.5.13.crate",
        type = "tar.gz",
        strip_prefix = "env_logger-0.5.13",
        build_file = Label("//third_party/cargo/remote:env_logger-0.5.13.BUILD"),
    )

    _new_http_archive(
        name = "raze__fuchsia_cprng__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/fuchsia-cprng/fuchsia-cprng-0.1.1.crate",
        type = "tar.gz",
        strip_prefix = "fuchsia-cprng-0.1.1",
        build_file = Label("//third_party/cargo/remote:fuchsia-cprng-0.1.1.BUILD"),
    )

    _new_http_archive(
        name = "raze__getopts__0_2_21",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/getopts/getopts-0.2.21.crate",
        type = "tar.gz",
        strip_prefix = "getopts-0.2.21",
        build_file = Label("//third_party/cargo/remote:getopts-0.2.21.BUILD"),
    )

    _new_http_archive(
        name = "raze__glob__0_2_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/glob/glob-0.2.11.crate",
        type = "tar.gz",
        strip_prefix = "glob-0.2.11",
        build_file = Label("//third_party/cargo/remote:glob-0.2.11.BUILD"),
    )

    _new_http_archive(
        name = "raze__hermit_abi__0_1_15",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/hermit-abi/hermit-abi-0.1.15.crate",
        type = "tar.gz",
        strip_prefix = "hermit-abi-0.1.15",
        build_file = Label("//third_party/cargo/remote:hermit-abi-0.1.15.BUILD"),
    )

    _new_http_archive(
        name = "raze__humantime__1_3_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/humantime/humantime-1.3.0.crate",
        type = "tar.gz",
        strip_prefix = "humantime-1.3.0",
        build_file = Label("//third_party/cargo/remote:humantime-1.3.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__lazy_static__0_2_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazy_static/lazy_static-0.2.11.crate",
        type = "tar.gz",
        strip_prefix = "lazy_static-0.2.11",
        build_file = Label("//third_party/cargo/remote:lazy_static-0.2.11.BUILD"),
    )

    _new_http_archive(
        name = "raze__lazy_static__1_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazy_static/lazy_static-1.4.0.crate",
        type = "tar.gz",
        strip_prefix = "lazy_static-1.4.0",
        build_file = Label("//third_party/cargo/remote:lazy_static-1.4.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__libc__0_2_77",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libc/libc-0.2.77.crate",
        type = "tar.gz",
        strip_prefix = "libc-0.2.77",
        build_file = Label("//third_party/cargo/remote:libc-0.2.77.BUILD"),
    )

    _new_http_archive(
        name = "raze__libloading__0_5_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libloading/libloading-0.5.2.crate",
        type = "tar.gz",
        strip_prefix = "libloading-0.5.2",
        build_file = Label("//third_party/cargo/remote:libloading-0.5.2.BUILD"),
    )

    _new_http_archive(
        name = "raze__log__0_4_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/log/log-0.4.6.crate",
        type = "tar.gz",
        strip_prefix = "log-0.4.6",
        build_file = Label("//third_party/cargo/remote:log-0.4.6.BUILD"),
    )

    _new_http_archive(
        name = "raze__memchr__1_0_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memchr/memchr-1.0.2.crate",
        type = "tar.gz",
        strip_prefix = "memchr-1.0.2",
        build_file = Label("//third_party/cargo/remote:memchr-1.0.2.BUILD"),
    )

    _new_http_archive(
        name = "raze__memchr__2_3_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memchr/memchr-2.3.3.crate",
        type = "tar.gz",
        strip_prefix = "memchr-2.3.3",
        build_file = Label("//third_party/cargo/remote:memchr-2.3.3.BUILD"),
    )

    _new_http_archive(
        name = "raze__nom__3_2_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/nom/nom-3.2.1.crate",
        type = "tar.gz",
        strip_prefix = "nom-3.2.1",
        build_file = Label("//third_party/cargo/remote:nom-3.2.1.BUILD"),
    )

    _new_http_archive(
        name = "raze__num__0_1_42",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num/num-0.1.42.crate",
        type = "tar.gz",
        strip_prefix = "num-0.1.42",
        build_file = Label("//third_party/cargo/remote:num-0.1.42.BUILD"),
    )

    _new_http_archive(
        name = "raze__num_integer__0_1_43",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num-integer/num-integer-0.1.43.crate",
        type = "tar.gz",
        strip_prefix = "num-integer-0.1.43",
        build_file = Label("//third_party/cargo/remote:num-integer-0.1.43.BUILD"),
    )

    _new_http_archive(
        name = "raze__num_iter__0_1_41",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num-iter/num-iter-0.1.41.crate",
        type = "tar.gz",
        strip_prefix = "num-iter-0.1.41",
        build_file = Label("//third_party/cargo/remote:num-iter-0.1.41.BUILD"),
    )

    _new_http_archive(
        name = "raze__num_traits__0_2_12",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/num-traits/num-traits-0.2.12.crate",
        type = "tar.gz",
        strip_prefix = "num-traits-0.2.12",
        build_file = Label("//third_party/cargo/remote:num-traits-0.2.12.BUILD"),
    )

    _new_http_archive(
        name = "raze__peeking_take_while__0_1_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/peeking_take_while/peeking_take_while-0.1.2.crate",
        type = "tar.gz",
        strip_prefix = "peeking_take_while-0.1.2",
        build_file = Label("//third_party/cargo/remote:peeking_take_while-0.1.2.BUILD"),
    )

    _new_http_archive(
        name = "raze__quick_error__1_2_3",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quick-error/quick-error-1.2.3.crate",
        type = "tar.gz",
        strip_prefix = "quick-error-1.2.3",
        build_file = Label("//third_party/cargo/remote:quick-error-1.2.3.BUILD"),
    )

    _new_http_archive(
        name = "raze__quote__0_3_15",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quote/quote-0.3.15.crate",
        type = "tar.gz",
        strip_prefix = "quote-0.3.15",
        build_file = Label("//third_party/cargo/remote:quote-0.3.15.BUILD"),
    )

    _new_http_archive(
        name = "raze__rand__0_3_23",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rand/rand-0.3.23.crate",
        type = "tar.gz",
        strip_prefix = "rand-0.3.23",
        build_file = Label("//third_party/cargo/remote:rand-0.3.23.BUILD"),
    )

    _new_http_archive(
        name = "raze__rand__0_4_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rand/rand-0.4.6.crate",
        type = "tar.gz",
        strip_prefix = "rand-0.4.6",
        build_file = Label("//third_party/cargo/remote:rand-0.4.6.BUILD"),
    )

    _new_http_archive(
        name = "raze__rand_core__0_3_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rand_core/rand_core-0.3.1.crate",
        type = "tar.gz",
        strip_prefix = "rand_core-0.3.1",
        build_file = Label("//third_party/cargo/remote:rand_core-0.3.1.BUILD"),
    )

    _new_http_archive(
        name = "raze__rand_core__0_4_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rand_core/rand_core-0.4.2.crate",
        type = "tar.gz",
        strip_prefix = "rand_core-0.4.2",
        build_file = Label("//third_party/cargo/remote:rand_core-0.4.2.BUILD"),
    )

    _new_http_archive(
        name = "raze__rdrand__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/rdrand/rdrand-0.4.0.crate",
        type = "tar.gz",
        strip_prefix = "rdrand-0.4.0",
        build_file = Label("//third_party/cargo/remote:rdrand-0.4.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__regex__0_2_11",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex/regex-0.2.11.crate",
        type = "tar.gz",
        strip_prefix = "regex-0.2.11",
        build_file = Label("//third_party/cargo/remote:regex-0.2.11.BUILD"),
    )

    _new_http_archive(
        name = "raze__regex__1_3_9",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex/regex-1.3.9.crate",
        type = "tar.gz",
        strip_prefix = "regex-1.3.9",
        build_file = Label("//third_party/cargo/remote:regex-1.3.9.BUILD"),
    )

    _new_http_archive(
        name = "raze__regex_syntax__0_5_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex-syntax/regex-syntax-0.5.6.crate",
        type = "tar.gz",
        strip_prefix = "regex-syntax-0.5.6",
        build_file = Label("//third_party/cargo/remote:regex-syntax-0.5.6.BUILD"),
    )

    _new_http_archive(
        name = "raze__regex_syntax__0_6_18",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex-syntax/regex-syntax-0.6.18.crate",
        type = "tar.gz",
        strip_prefix = "regex-syntax-0.6.18",
        build_file = Label("//third_party/cargo/remote:regex-syntax-0.6.18.BUILD"),
    )

    _new_http_archive(
        name = "raze__sdl2__0_31_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/sdl2/sdl2-0.31.0.crate",
        type = "tar.gz",
        strip_prefix = "sdl2-0.31.0",
        build_file = Label("//third_party/cargo/remote:sdl2-0.31.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__sdl2_sys__0_31_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/sdl2-sys/sdl2-sys-0.31.0.crate",
        type = "tar.gz",
        strip_prefix = "sdl2-sys-0.31.0",
        build_file = Label("//third_party/cargo/remote:sdl2-sys-0.31.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__strsim__0_8_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/strsim/strsim-0.8.0.crate",
        type = "tar.gz",
        strip_prefix = "strsim-0.8.0",
        build_file = Label("//third_party/cargo/remote:strsim-0.8.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__termcolor__1_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/termcolor/termcolor-1.1.0.crate",
        type = "tar.gz",
        strip_prefix = "termcolor-1.1.0",
        build_file = Label("//third_party/cargo/remote:termcolor-1.1.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__textwrap__0_11_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/textwrap/textwrap-0.11.0.crate",
        type = "tar.gz",
        strip_prefix = "textwrap-0.11.0",
        build_file = Label("//third_party/cargo/remote:textwrap-0.11.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__thread_local__0_3_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/thread_local/thread_local-0.3.6.crate",
        type = "tar.gz",
        strip_prefix = "thread_local-0.3.6",
        build_file = Label("//third_party/cargo/remote:thread_local-0.3.6.BUILD"),
    )

    _new_http_archive(
        name = "raze__thread_local__1_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/thread_local/thread_local-1.0.1.crate",
        type = "tar.gz",
        strip_prefix = "thread_local-1.0.1",
        build_file = Label("//third_party/cargo/remote:thread_local-1.0.1.BUILD"),
    )

    _new_http_archive(
        name = "raze__time__0_1_44",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/time/time-0.1.44.crate",
        type = "tar.gz",
        strip_prefix = "time-0.1.44",
        build_file = Label("//third_party/cargo/remote:time-0.1.44.BUILD"),
    )

    _new_http_archive(
        name = "raze__ucd_util__0_1_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ucd-util/ucd-util-0.1.8.crate",
        type = "tar.gz",
        strip_prefix = "ucd-util-0.1.8",
        build_file = Label("//third_party/cargo/remote:ucd-util-0.1.8.BUILD"),
    )

    _new_http_archive(
        name = "raze__unicode_width__0_1_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-width/unicode-width-0.1.8.crate",
        type = "tar.gz",
        strip_prefix = "unicode-width-0.1.8",
        build_file = Label("//third_party/cargo/remote:unicode-width-0.1.8.BUILD"),
    )

    _new_http_archive(
        name = "raze__utf8_ranges__1_0_4",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/utf8-ranges/utf8-ranges-1.0.4.crate",
        type = "tar.gz",
        strip_prefix = "utf8-ranges-1.0.4",
        build_file = Label("//third_party/cargo/remote:utf8-ranges-1.0.4.BUILD"),
    )

    _new_http_archive(
        name = "raze__vec_map__0_8_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/vec_map/vec_map-0.8.2.crate",
        type = "tar.gz",
        strip_prefix = "vec_map-0.8.2",
        build_file = Label("//third_party/cargo/remote:vec_map-0.8.2.BUILD"),
    )

    _new_http_archive(
        name = "raze__wasi__0_10_0_wasi_snapshot_preview1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/wasi/wasi-0.10.0+wasi-snapshot-preview1.crate",
        type = "tar.gz",
        strip_prefix = "wasi-0.10.0+wasi-snapshot-preview1",
        build_file = Label("//third_party/cargo/remote:wasi-0.10.0+wasi-snapshot-preview1.BUILD"),
    )

    _new_http_archive(
        name = "raze__which__1_0_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/which/which-1.0.5.crate",
        type = "tar.gz",
        strip_prefix = "which-1.0.5",
        build_file = Label("//third_party/cargo/remote:which-1.0.5.BUILD"),
    )

    _new_http_archive(
        name = "raze__winapi__0_3_9",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi/winapi-0.3.9.crate",
        type = "tar.gz",
        strip_prefix = "winapi-0.3.9",
        build_file = Label("//third_party/cargo/remote:winapi-0.3.9.BUILD"),
    )

    _new_http_archive(
        name = "raze__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-i686-pc-windows-gnu/winapi-i686-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",
        build_file = Label("//third_party/cargo/remote:winapi-i686-pc-windows-gnu-0.4.0.BUILD"),
    )

    _new_http_archive(
        name = "raze__winapi_util__0_1_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-util/winapi-util-0.1.5.crate",
        type = "tar.gz",
        strip_prefix = "winapi-util-0.1.5",
        build_file = Label("//third_party/cargo/remote:winapi-util-0.1.5.BUILD"),
    )

    _new_http_archive(
        name = "raze__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/winapi-x86_64-pc-windows-gnu/winapi-x86_64-pc-windows-gnu-0.4.0.crate",
        type = "tar.gz",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",
        build_file = Label("//third_party/cargo/remote:winapi-x86_64-pc-windows-gnu-0.4.0.BUILD"),
    )

