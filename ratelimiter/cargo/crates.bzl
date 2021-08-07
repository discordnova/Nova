"""
@generated
cargo-raze generated Bazel file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""

load("@bazel_tools//tools/build_defs/repo:git.bzl", "new_git_repository")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")  # buildifier: disable=load

def raze_fetch_remote_crates():
    """This function defines a collection of repos and should be called in a WORKSPACE file"""
    maybe(
        http_archive,
        name = "raze__anyhow__1_0_42",
        url = "https://crates.io/api/v1/crates/anyhow/1.0.42/download",
        type = "tar.gz",
        sha256 = "595d3cfa7a60d4555cb5067b99f07142a08ea778de5cf993f7b75c7d8fabc486",
        strip_prefix = "anyhow-1.0.42",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.anyhow-1.0.42.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__async_stream__0_3_2",
        url = "https://crates.io/api/v1/crates/async-stream/0.3.2/download",
        type = "tar.gz",
        sha256 = "171374e7e3b2504e0e5236e3b59260560f9fe94bfe9ac39ba5e4e929c5590625",
        strip_prefix = "async-stream-0.3.2",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.async-stream-0.3.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__async_stream_impl__0_3_2",
        url = "https://crates.io/api/v1/crates/async-stream-impl/0.3.2/download",
        type = "tar.gz",
        sha256 = "648ed8c8d2ce5409ccd57453d9d1b214b342a0d69376a6feda1fd6cae3299308",
        strip_prefix = "async-stream-impl-0.3.2",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.async-stream-impl-0.3.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__async_trait__0_1_51",
        url = "https://crates.io/api/v1/crates/async-trait/0.1.51/download",
        type = "tar.gz",
        sha256 = "44318e776df68115a881de9a8fd1b9e53368d7a4a5ce4cc48517da3393233a5e",
        strip_prefix = "async-trait-0.1.51",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.async-trait-0.1.51.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__autocfg__1_0_1",
        url = "https://crates.io/api/v1/crates/autocfg/1.0.1/download",
        type = "tar.gz",
        sha256 = "cdb031dd78e28731d87d56cc8ffef4a8f36ca26c38fe2de700543e627f8a464a",
        strip_prefix = "autocfg-1.0.1",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.autocfg-1.0.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__base64__0_13_0",
        url = "https://crates.io/api/v1/crates/base64/0.13.0/download",
        type = "tar.gz",
        sha256 = "904dfeac50f3cdaba28fc6f57fdcddb75f49ed61346676a78c4ffe55877802fd",
        strip_prefix = "base64-0.13.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.base64-0.13.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__bitflags__1_2_1",
        url = "https://crates.io/api/v1/crates/bitflags/1.2.1/download",
        type = "tar.gz",
        sha256 = "cf1de2fe8c75bc145a2f577add951f8134889b4795d47466a54a5c846d691693",
        strip_prefix = "bitflags-1.2.1",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.bitflags-1.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__bytes__1_0_1",
        url = "https://crates.io/api/v1/crates/bytes/1.0.1/download",
        type = "tar.gz",
        sha256 = "b700ce4376041dcd0a327fd0097c41095743c4c8af8887265942faf1100bd040",
        strip_prefix = "bytes-1.0.1",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.bytes-1.0.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__cfg_if__1_0_0",
        url = "https://crates.io/api/v1/crates/cfg-if/1.0.0/download",
        type = "tar.gz",
        sha256 = "baf1de4339761588bc0619e3cbc0120ee582ebb74b53b4efbf79117bd2da40fd",
        strip_prefix = "cfg-if-1.0.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.cfg-if-1.0.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__either__1_6_1",
        url = "https://crates.io/api/v1/crates/either/1.6.1/download",
        type = "tar.gz",
        sha256 = "e78d4f1cc4ae33bbfc157ed5d5a5ef3bc29227303d595861deb238fcec4e9457",
        strip_prefix = "either-1.6.1",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.either-1.6.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__fixedbitset__0_2_0",
        url = "https://crates.io/api/v1/crates/fixedbitset/0.2.0/download",
        type = "tar.gz",
        sha256 = "37ab347416e802de484e4d03c7316c48f1ecb56574dfd4a46a80f173ce1de04d",
        strip_prefix = "fixedbitset-0.2.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.fixedbitset-0.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__fnv__1_0_7",
        url = "https://crates.io/api/v1/crates/fnv/1.0.7/download",
        type = "tar.gz",
        sha256 = "3f9eec918d3f24069decb9af1554cad7c880e2da24a9afd88aca000531ab82c1",
        strip_prefix = "fnv-1.0.7",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.fnv-1.0.7.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__futures_channel__0_3_16",
        url = "https://crates.io/api/v1/crates/futures-channel/0.3.16/download",
        type = "tar.gz",
        sha256 = "74ed2411805f6e4e3d9bc904c95d5d423b89b3b25dc0250aa74729de20629ff9",
        strip_prefix = "futures-channel-0.3.16",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.futures-channel-0.3.16.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__futures_core__0_3_16",
        url = "https://crates.io/api/v1/crates/futures-core/0.3.16/download",
        type = "tar.gz",
        sha256 = "af51b1b4a7fdff033703db39de8802c673eb91855f2e0d47dcf3bf2c0ef01f99",
        strip_prefix = "futures-core-0.3.16",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.futures-core-0.3.16.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__futures_sink__0_3_16",
        url = "https://crates.io/api/v1/crates/futures-sink/0.3.16/download",
        type = "tar.gz",
        sha256 = "c0f30aaa67363d119812743aa5f33c201a7a66329f97d1a887022971feea4b53",
        strip_prefix = "futures-sink-0.3.16",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.futures-sink-0.3.16.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__futures_task__0_3_16",
        url = "https://crates.io/api/v1/crates/futures-task/0.3.16/download",
        type = "tar.gz",
        sha256 = "bbe54a98670017f3be909561f6ad13e810d9a51f3f061b902062ca3da80799f2",
        strip_prefix = "futures-task-0.3.16",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.futures-task-0.3.16.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__futures_util__0_3_16",
        url = "https://crates.io/api/v1/crates/futures-util/0.3.16/download",
        type = "tar.gz",
        sha256 = "67eb846bfd58e44a8481a00049e82c43e0ccb5d61f8dc071057cb19249dd4d78",
        strip_prefix = "futures-util-0.3.16",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.futures-util-0.3.16.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__getrandom__0_2_3",
        url = "https://crates.io/api/v1/crates/getrandom/0.2.3/download",
        type = "tar.gz",
        sha256 = "7fcd999463524c52659517fe2cea98493cfe485d10565e7b0fb07dbba7ad2753",
        strip_prefix = "getrandom-0.2.3",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.getrandom-0.2.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__h2__0_3_3",
        url = "https://crates.io/api/v1/crates/h2/0.3.3/download",
        type = "tar.gz",
        sha256 = "825343c4eef0b63f541f8903f395dc5beb362a979b5799a84062527ef1e37726",
        strip_prefix = "h2-0.3.3",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.h2-0.3.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__hashbrown__0_11_2",
        url = "https://crates.io/api/v1/crates/hashbrown/0.11.2/download",
        type = "tar.gz",
        sha256 = "ab5ef0d4909ef3724cc8cce6ccc8572c5c817592e9285f5464f8e86f8bd3726e",
        strip_prefix = "hashbrown-0.11.2",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.hashbrown-0.11.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__heck__0_3_3",
        url = "https://crates.io/api/v1/crates/heck/0.3.3/download",
        type = "tar.gz",
        sha256 = "6d621efb26863f0e9924c6ac577e8275e5e6b77455db64ffa6c65c904e9e132c",
        strip_prefix = "heck-0.3.3",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.heck-0.3.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__hermit_abi__0_1_19",
        url = "https://crates.io/api/v1/crates/hermit-abi/0.1.19/download",
        type = "tar.gz",
        sha256 = "62b467343b94ba476dcb2500d242dadbb39557df889310ac77c5d99100aaac33",
        strip_prefix = "hermit-abi-0.1.19",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.hermit-abi-0.1.19.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__http__0_2_4",
        url = "https://crates.io/api/v1/crates/http/0.2.4/download",
        type = "tar.gz",
        sha256 = "527e8c9ac747e28542699a951517aa9a6945af506cd1f2e1b53a576c17b6cc11",
        strip_prefix = "http-0.2.4",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.http-0.2.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__http_body__0_4_2",
        url = "https://crates.io/api/v1/crates/http-body/0.4.2/download",
        type = "tar.gz",
        sha256 = "60daa14be0e0786db0f03a9e57cb404c9d756eed2b6c62b9ea98ec5743ec75a9",
        strip_prefix = "http-body-0.4.2",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.http-body-0.4.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__httparse__1_4_1",
        url = "https://crates.io/api/v1/crates/httparse/1.4.1/download",
        type = "tar.gz",
        sha256 = "f3a87b616e37e93c22fb19bcd386f02f3af5ea98a25670ad0fce773de23c5e68",
        strip_prefix = "httparse-1.4.1",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.httparse-1.4.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__httpdate__1_0_1",
        url = "https://crates.io/api/v1/crates/httpdate/1.0.1/download",
        type = "tar.gz",
        sha256 = "6456b8a6c8f33fee7d958fcd1b60d55b11940a79e63ae87013e6d22e26034440",
        strip_prefix = "httpdate-1.0.1",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.httpdate-1.0.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__hyper__0_14_11",
        url = "https://crates.io/api/v1/crates/hyper/0.14.11/download",
        type = "tar.gz",
        sha256 = "0b61cf2d1aebcf6e6352c97b81dc2244ca29194be1b276f5d8ad5c6330fffb11",
        strip_prefix = "hyper-0.14.11",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.hyper-0.14.11.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__hyper_timeout__0_4_1",
        url = "https://crates.io/api/v1/crates/hyper-timeout/0.4.1/download",
        type = "tar.gz",
        sha256 = "bbb958482e8c7be4bc3cf272a766a2b0bf1a6755e7a6ae777f017a31d11b13b1",
        strip_prefix = "hyper-timeout-0.4.1",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.hyper-timeout-0.4.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__indexmap__1_7_0",
        url = "https://crates.io/api/v1/crates/indexmap/1.7.0/download",
        type = "tar.gz",
        sha256 = "bc633605454125dec4b66843673f01c7df2b89479b32e0ed634e43a91cff62a5",
        strip_prefix = "indexmap-1.7.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.indexmap-1.7.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__itertools__0_10_1",
        url = "https://crates.io/api/v1/crates/itertools/0.10.1/download",
        type = "tar.gz",
        sha256 = "69ddb889f9d0d08a67338271fa9b62996bc788c7796a5c18cf057420aaed5eaf",
        strip_prefix = "itertools-0.10.1",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.itertools-0.10.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__itoa__0_4_7",
        url = "https://crates.io/api/v1/crates/itoa/0.4.7/download",
        type = "tar.gz",
        sha256 = "dd25036021b0de88a0aff6b850051563c6516d0bf53f8638938edbb9de732736",
        strip_prefix = "itoa-0.4.7",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.itoa-0.4.7.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__lazy_static__1_4_0",
        url = "https://crates.io/api/v1/crates/lazy_static/1.4.0/download",
        type = "tar.gz",
        sha256 = "e2abad23fbc42b3700f2f279844dc832adb2b2eb069b2df918f455c4e18cc646",
        strip_prefix = "lazy_static-1.4.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.lazy_static-1.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__libc__0_2_98",
        url = "https://crates.io/api/v1/crates/libc/0.2.98/download",
        type = "tar.gz",
        sha256 = "320cfe77175da3a483efed4bc0adc1968ca050b098ce4f2f1c13a56626128790",
        strip_prefix = "libc-0.2.98",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.libc-0.2.98.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__log__0_4_14",
        url = "https://crates.io/api/v1/crates/log/0.4.14/download",
        type = "tar.gz",
        sha256 = "51b9bbe6c47d51fc3e1a9b945965946b4c44142ab8792c50835a980d362c2710",
        strip_prefix = "log-0.4.14",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.log-0.4.14.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__memchr__2_4_0",
        url = "https://crates.io/api/v1/crates/memchr/2.4.0/download",
        type = "tar.gz",
        sha256 = "b16bd47d9e329435e309c58469fe0791c2d0d1ba96ec0954152a5ae2b04387dc",
        strip_prefix = "memchr-2.4.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.memchr-2.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__mio__0_7_13",
        url = "https://crates.io/api/v1/crates/mio/0.7.13/download",
        type = "tar.gz",
        sha256 = "8c2bdb6314ec10835cd3293dd268473a835c02b7b352e788be788b3c6ca6bb16",
        strip_prefix = "mio-0.7.13",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.mio-0.7.13.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__miow__0_3_7",
        url = "https://crates.io/api/v1/crates/miow/0.3.7/download",
        type = "tar.gz",
        sha256 = "b9f1c5b025cda876f66ef43a113f91ebc9f4ccef34843000e0adf6ebbab84e21",
        strip_prefix = "miow-0.3.7",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.miow-0.3.7.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__multimap__0_8_3",
        url = "https://crates.io/api/v1/crates/multimap/0.8.3/download",
        type = "tar.gz",
        sha256 = "e5ce46fe64a9d73be07dcbe690a38ce1b293be448fd8ce1e6c1b8062c9f72c6a",
        strip_prefix = "multimap-0.8.3",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.multimap-0.8.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__ntapi__0_3_6",
        url = "https://crates.io/api/v1/crates/ntapi/0.3.6/download",
        type = "tar.gz",
        sha256 = "3f6bb902e437b6d86e03cce10a7e2af662292c5dfef23b65899ea3ac9354ad44",
        strip_prefix = "ntapi-0.3.6",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.ntapi-0.3.6.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__num_cpus__1_13_0",
        url = "https://crates.io/api/v1/crates/num_cpus/1.13.0/download",
        type = "tar.gz",
        sha256 = "05499f3756671c15885fee9034446956fff3f243d6077b91e5767df161f766b3",
        strip_prefix = "num_cpus-1.13.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.num_cpus-1.13.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__percent_encoding__2_1_0",
        url = "https://crates.io/api/v1/crates/percent-encoding/2.1.0/download",
        type = "tar.gz",
        sha256 = "d4fd5641d01c8f18a23da7b6fe29298ff4b55afcccdf78973b24cf3175fee32e",
        strip_prefix = "percent-encoding-2.1.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.percent-encoding-2.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__petgraph__0_5_1",
        url = "https://crates.io/api/v1/crates/petgraph/0.5.1/download",
        type = "tar.gz",
        sha256 = "467d164a6de56270bd7c4d070df81d07beace25012d5103ced4e9ff08d6afdb7",
        strip_prefix = "petgraph-0.5.1",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.petgraph-0.5.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__pin_project__1_0_8",
        url = "https://crates.io/api/v1/crates/pin-project/1.0.8/download",
        type = "tar.gz",
        sha256 = "576bc800220cc65dac09e99e97b08b358cfab6e17078de8dc5fee223bd2d0c08",
        strip_prefix = "pin-project-1.0.8",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.pin-project-1.0.8.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__pin_project_internal__1_0_8",
        url = "https://crates.io/api/v1/crates/pin-project-internal/1.0.8/download",
        type = "tar.gz",
        sha256 = "6e8fe8163d14ce7f0cdac2e040116f22eac817edabff0be91e8aff7e9accf389",
        strip_prefix = "pin-project-internal-1.0.8",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.pin-project-internal-1.0.8.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__pin_project_lite__0_2_7",
        url = "https://crates.io/api/v1/crates/pin-project-lite/0.2.7/download",
        type = "tar.gz",
        sha256 = "8d31d11c69a6b52a174b42bdc0c30e5e11670f90788b2c471c31c1d17d449443",
        strip_prefix = "pin-project-lite-0.2.7",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.pin-project-lite-0.2.7.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__pin_utils__0_1_0",
        url = "https://crates.io/api/v1/crates/pin-utils/0.1.0/download",
        type = "tar.gz",
        sha256 = "8b870d8c151b6f2fb93e84a13146138f05d02ed11c7e7c54f8826aaaf7c9f184",
        strip_prefix = "pin-utils-0.1.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.pin-utils-0.1.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__ppv_lite86__0_2_10",
        url = "https://crates.io/api/v1/crates/ppv-lite86/0.2.10/download",
        type = "tar.gz",
        sha256 = "ac74c624d6b2d21f425f752262f42188365d7b8ff1aff74c82e45136510a4857",
        strip_prefix = "ppv-lite86-0.2.10",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.ppv-lite86-0.2.10.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__proc_macro2__1_0_28",
        url = "https://crates.io/api/v1/crates/proc-macro2/1.0.28/download",
        type = "tar.gz",
        sha256 = "5c7ed8b8c7b886ea3ed7dde405212185f423ab44682667c8c6dd14aa1d9f6612",
        strip_prefix = "proc-macro2-1.0.28",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.proc-macro2-1.0.28.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__prost__0_8_0",
        url = "https://crates.io/api/v1/crates/prost/0.8.0/download",
        type = "tar.gz",
        sha256 = "de5e2533f59d08fcf364fd374ebda0692a70bd6d7e66ef97f306f45c6c5d8020",
        strip_prefix = "prost-0.8.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.prost-0.8.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__prost_build__0_8_0",
        url = "https://crates.io/api/v1/crates/prost-build/0.8.0/download",
        type = "tar.gz",
        sha256 = "355f634b43cdd80724ee7848f95770e7e70eefa6dcf14fea676216573b8fd603",
        strip_prefix = "prost-build-0.8.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.prost-build-0.8.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__prost_derive__0_8_0",
        url = "https://crates.io/api/v1/crates/prost-derive/0.8.0/download",
        type = "tar.gz",
        sha256 = "600d2f334aa05acb02a755e217ef1ab6dea4d51b58b7846588b747edec04efba",
        strip_prefix = "prost-derive-0.8.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.prost-derive-0.8.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__prost_types__0_8_0",
        url = "https://crates.io/api/v1/crates/prost-types/0.8.0/download",
        type = "tar.gz",
        sha256 = "603bbd6394701d13f3f25aada59c7de9d35a6a5887cfc156181234a44002771b",
        strip_prefix = "prost-types-0.8.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.prost-types-0.8.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__quote__1_0_9",
        url = "https://crates.io/api/v1/crates/quote/1.0.9/download",
        type = "tar.gz",
        sha256 = "c3d0b9745dc2debf507c8422de05d7226cc1f0644216dfdfead988f9b1ab32a7",
        strip_prefix = "quote-1.0.9",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.quote-1.0.9.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__rand__0_8_4",
        url = "https://crates.io/api/v1/crates/rand/0.8.4/download",
        type = "tar.gz",
        sha256 = "2e7573632e6454cf6b99d7aac4ccca54be06da05aca2ef7423d22d27d4d4bcd8",
        strip_prefix = "rand-0.8.4",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.rand-0.8.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__rand_chacha__0_3_1",
        url = "https://crates.io/api/v1/crates/rand_chacha/0.3.1/download",
        type = "tar.gz",
        sha256 = "e6c10a63a0fa32252be49d21e7709d4d4baf8d231c2dbce1eaa8141b9b127d88",
        strip_prefix = "rand_chacha-0.3.1",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.rand_chacha-0.3.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__rand_core__0_6_3",
        url = "https://crates.io/api/v1/crates/rand_core/0.6.3/download",
        type = "tar.gz",
        sha256 = "d34f1408f55294453790c48b2f1ebbb1c5b4b7563eb1f418bcfcfdbb06ebb4e7",
        strip_prefix = "rand_core-0.6.3",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.rand_core-0.6.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__rand_hc__0_3_1",
        url = "https://crates.io/api/v1/crates/rand_hc/0.3.1/download",
        type = "tar.gz",
        sha256 = "d51e9f596de227fda2ea6c84607f5558e196eeaf43c986b724ba4fb8fdf497e7",
        strip_prefix = "rand_hc-0.3.1",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.rand_hc-0.3.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__redox_syscall__0_2_10",
        url = "https://crates.io/api/v1/crates/redox_syscall/0.2.10/download",
        type = "tar.gz",
        sha256 = "8383f39639269cde97d255a32bdb68c047337295414940c68bdd30c2e13203ff",
        strip_prefix = "redox_syscall-0.2.10",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.redox_syscall-0.2.10.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__remove_dir_all__0_5_3",
        url = "https://crates.io/api/v1/crates/remove_dir_all/0.5.3/download",
        type = "tar.gz",
        sha256 = "3acd125665422973a33ac9d3dd2df85edad0f4ae9b00dafb1a05e43a9f5ef8e7",
        strip_prefix = "remove_dir_all-0.5.3",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.remove_dir_all-0.5.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__slab__0_4_4",
        url = "https://crates.io/api/v1/crates/slab/0.4.4/download",
        type = "tar.gz",
        sha256 = "c307a32c1c5c437f38c7fd45d753050587732ba8628319fbdf12a7e289ccc590",
        strip_prefix = "slab-0.4.4",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.slab-0.4.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__socket2__0_4_1",
        url = "https://crates.io/api/v1/crates/socket2/0.4.1/download",
        type = "tar.gz",
        sha256 = "765f090f0e423d2b55843402a07915add955e7d60657db13707a159727326cad",
        strip_prefix = "socket2-0.4.1",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.socket2-0.4.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__syn__1_0_74",
        url = "https://crates.io/api/v1/crates/syn/1.0.74/download",
        type = "tar.gz",
        sha256 = "1873d832550d4588c3dbc20f01361ab00bfe741048f71e3fecf145a7cc18b29c",
        strip_prefix = "syn-1.0.74",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.syn-1.0.74.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__tempfile__3_2_0",
        url = "https://crates.io/api/v1/crates/tempfile/3.2.0/download",
        type = "tar.gz",
        sha256 = "dac1c663cfc93810f88aed9b8941d48cabf856a1b111c29a40439018d870eb22",
        strip_prefix = "tempfile-3.2.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.tempfile-3.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__tokio__1_9_0",
        url = "https://crates.io/api/v1/crates/tokio/1.9.0/download",
        type = "tar.gz",
        sha256 = "4b7b349f11a7047e6d1276853e612d152f5e8a352c61917887cc2169e2366b4c",
        strip_prefix = "tokio-1.9.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.tokio-1.9.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__tokio_io_timeout__1_1_1",
        url = "https://crates.io/api/v1/crates/tokio-io-timeout/1.1.1/download",
        type = "tar.gz",
        sha256 = "90c49f106be240de154571dd31fbe48acb10ba6c6dd6f6517ad603abffa42de9",
        strip_prefix = "tokio-io-timeout-1.1.1",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.tokio-io-timeout-1.1.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__tokio_macros__1_3_0",
        url = "https://crates.io/api/v1/crates/tokio-macros/1.3.0/download",
        type = "tar.gz",
        sha256 = "54473be61f4ebe4efd09cec9bd5d16fa51d70ea0192213d754d2d500457db110",
        strip_prefix = "tokio-macros-1.3.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.tokio-macros-1.3.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__tokio_stream__0_1_7",
        url = "https://crates.io/api/v1/crates/tokio-stream/0.1.7/download",
        type = "tar.gz",
        sha256 = "7b2f3f698253f03119ac0102beaa64f67a67e08074d03a22d18784104543727f",
        strip_prefix = "tokio-stream-0.1.7",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.tokio-stream-0.1.7.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__tokio_util__0_6_7",
        url = "https://crates.io/api/v1/crates/tokio-util/0.6.7/download",
        type = "tar.gz",
        sha256 = "1caa0b0c8d94a049db56b5acf8cba99dc0623aab1b26d5b5f5e2d945846b3592",
        strip_prefix = "tokio-util-0.6.7",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.tokio-util-0.6.7.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__tonic__0_5_0",
        url = "https://crates.io/api/v1/crates/tonic/0.5.0/download",
        type = "tar.gz",
        sha256 = "b584f064fdfc50017ec39162d5aebce49912f1eb16fd128e04b7f4ce4907c7e5",
        strip_prefix = "tonic-0.5.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.tonic-0.5.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__tonic_build__0_5_1",
        url = "https://crates.io/api/v1/crates/tonic-build/0.5.1/download",
        type = "tar.gz",
        sha256 = "d12faebbe071b06f486be82cc9318350814fdd07fcb28f3690840cd770599283",
        strip_prefix = "tonic-build-0.5.1",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.tonic-build-0.5.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__tower__0_4_8",
        url = "https://crates.io/api/v1/crates/tower/0.4.8/download",
        type = "tar.gz",
        sha256 = "f60422bc7fefa2f3ec70359b8ff1caff59d785877eb70595904605bcc412470f",
        strip_prefix = "tower-0.4.8",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.tower-0.4.8.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__tower_layer__0_3_1",
        url = "https://crates.io/api/v1/crates/tower-layer/0.3.1/download",
        type = "tar.gz",
        sha256 = "343bc9466d3fe6b0f960ef45960509f84480bf4fd96f92901afe7ff3df9d3a62",
        strip_prefix = "tower-layer-0.3.1",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.tower-layer-0.3.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__tower_service__0_3_1",
        url = "https://crates.io/api/v1/crates/tower-service/0.3.1/download",
        type = "tar.gz",
        sha256 = "360dfd1d6d30e05fda32ace2c8c70e9c0a9da713275777f5a4dbb8a1893930c6",
        strip_prefix = "tower-service-0.3.1",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.tower-service-0.3.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__tracing__0_1_26",
        url = "https://crates.io/api/v1/crates/tracing/0.1.26/download",
        type = "tar.gz",
        sha256 = "09adeb8c97449311ccd28a427f96fb563e7fd31aabf994189879d9da2394b89d",
        strip_prefix = "tracing-0.1.26",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.tracing-0.1.26.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__tracing_attributes__0_1_15",
        url = "https://crates.io/api/v1/crates/tracing-attributes/0.1.15/download",
        type = "tar.gz",
        sha256 = "c42e6fa53307c8a17e4ccd4dc81cf5ec38db9209f59b222210375b54ee40d1e2",
        strip_prefix = "tracing-attributes-0.1.15",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.tracing-attributes-0.1.15.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__tracing_core__0_1_18",
        url = "https://crates.io/api/v1/crates/tracing-core/0.1.18/download",
        type = "tar.gz",
        sha256 = "a9ff14f98b1a4b289c6248a023c1c2fa1491062964e9fed67ab29c4e4da4a052",
        strip_prefix = "tracing-core-0.1.18",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.tracing-core-0.1.18.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__tracing_futures__0_2_5",
        url = "https://crates.io/api/v1/crates/tracing-futures/0.2.5/download",
        type = "tar.gz",
        sha256 = "97d095ae15e245a057c8e8451bab9b3ee1e1f68e9ba2b4fbc18d0ac5237835f2",
        strip_prefix = "tracing-futures-0.2.5",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.tracing-futures-0.2.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__try_lock__0_2_3",
        url = "https://crates.io/api/v1/crates/try-lock/0.2.3/download",
        type = "tar.gz",
        sha256 = "59547bce71d9c38b83d9c0e92b6066c4253371f15005def0c30d9657f50c7642",
        strip_prefix = "try-lock-0.2.3",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.try-lock-0.2.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__unicode_segmentation__1_8_0",
        url = "https://crates.io/api/v1/crates/unicode-segmentation/1.8.0/download",
        type = "tar.gz",
        sha256 = "8895849a949e7845e06bd6dc1aa51731a103c42707010a5b591c0038fb73385b",
        strip_prefix = "unicode-segmentation-1.8.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.unicode-segmentation-1.8.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__unicode_xid__0_2_2",
        url = "https://crates.io/api/v1/crates/unicode-xid/0.2.2/download",
        type = "tar.gz",
        sha256 = "8ccb82d61f80a663efe1f787a51b16b5a51e3314d6ac365b08639f52387b33f3",
        strip_prefix = "unicode-xid-0.2.2",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.unicode-xid-0.2.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__want__0_3_0",
        url = "https://crates.io/api/v1/crates/want/0.3.0/download",
        type = "tar.gz",
        sha256 = "1ce8a968cb1cd110d136ff8b819a556d6fb6d919363c61534f6860c7eb172ba0",
        strip_prefix = "want-0.3.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.want-0.3.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasi__0_10_2_wasi_snapshot_preview1",
        url = "https://crates.io/api/v1/crates/wasi/0.10.2+wasi-snapshot-preview1/download",
        type = "tar.gz",
        sha256 = "fd6fbd9a79829dd1ad0cc20627bf1ed606756a7f77edff7b66b7064f9cb327c6",
        strip_prefix = "wasi-0.10.2+wasi-snapshot-preview1",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.wasi-0.10.2+wasi-snapshot-preview1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__which__4_2_2",
        url = "https://crates.io/api/v1/crates/which/4.2.2/download",
        type = "tar.gz",
        sha256 = "ea187a8ef279bc014ec368c27a920da2024d2a711109bfbe3440585d5cf27ad9",
        strip_prefix = "which-4.2.2",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.which-4.2.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__winapi__0_3_9",
        url = "https://crates.io/api/v1/crates/winapi/0.3.9/download",
        type = "tar.gz",
        sha256 = "5c839a674fcd7a98952e593242ea400abe93992746761e38641405d28b00f419",
        strip_prefix = "winapi-0.3.9",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.winapi-0.3.9.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates.io/api/v1/crates/winapi-i686-pc-windows-gnu/0.4.0/download",
        type = "tar.gz",
        sha256 = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.winapi-i686-pc-windows-gnu-0.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates.io/api/v1/crates/winapi-x86_64-pc-windows-gnu/0.4.0/download",
        type = "tar.gz",
        sha256 = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",
        build_file = Label("//ratelimiter/cargo/remote:BUILD.winapi-x86_64-pc-windows-gnu-0.4.0.bazel"),
    )
