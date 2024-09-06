# DejaGnu equivalvent directives for Rustc test headers:

Rust compiletest tool uses a set of directives in the test headers to control the behavior of the test. See rustc dev guide for more information on the [test directives](https://rustc-dev-guide.rust-lang.org/tests/headers.html#header-commands).

Equivalent DejaGnu directives are used to control the behavior of the test in the `gccrs` test suite

### [Controlling pass/fail expectations](https://rustc-dev-guide.rust-lang.org/tests/ui.html#controlling-passfail-expectations)

- **Pass headers**:

  | Rustc Directive | DejaGnu Directive                                    |
  | --------------- | ---------------------------------------------------- |
  | check-pass      |                                                      |
  | build-pass      |                                                      |
  | run-pass        | target (compilation,linking and running should pass) |

- **Fail headers**:

  | Rustc Directive | DejaGnu Directive |
  | --------------- | ----------------- |
  | check-fail      |                   |
  | build-fail      |                   |
  | run-fail        |                   |

The `run-pass` and `run-fail` tests in rust compiletest tool doesn't check the output of program. But dejagnu gives verdict based on the output of the program. So, the `run-pass` and `run-fail` tests in rustc are converted to `target` and `xfail` tests in dejagnu directives.

- **Other headers**:

  | Rustc Directive   | DejaGnu Directive                                         |
  | ----------------- | --------------------------------------------------------- |
  | ignore-pass       |                                                           |
  | check-run-results | xfail (thil will also compare the output of the compiler) |

## [UI](https://rustc-dev-guide.rust-lang.org/tests/ui.html) headers

| Rustc Directive                 | DejaGnu Directive |
| ------------------------------- | ----------------- |
| normalize-stderr-32bit          |                   |
| normalize-stderr-64bit          |                   |
| normalize-stderr-test           |                   |
| normalize-stdout-test           |                   |
| run-rustfix                     |                   |
| rustfix-only-machine-applicable |                   |
| stderr-per-bitwidth             |                   |
| dont-check-compiler-stderr      |                   |
| dont-check-compiler-stdout      |                   |
| compare-output-lines-by-subset  |                   |

## [Building auxiliary crates](https://rustc-dev-guide.rust-lang.org/tests/compiletest.html#building-auxiliary-crates)

| Rustc Directive     | DejaGnu Directive |
| ------------------- | ----------------- |
| aux-build           |                   |
| aux-crate           |                   |
| aux-bin             |                   |
| aux-codegen-backend |                   |

## [Pretty-printer](https://rustc-dev-guide.rust-lang.org/tests/compiletest.html#pretty-printer-tests) headers

| Rustc Directive     | DejaGnu Directive |
| ------------------- | ----------------- |
| pretty-compare-only |                   |
| pretty-expanded     |                   |
| pretty-mode         |                   |
| pp-exact            |                   |

## [Ignoring tests](https://rustc-dev-guide.rust-lang.org/tests/headers.html#ignoring-tests)

| Rustc Directive                    | DejaGnu Directive |
| ---------------------------------- | ----------------- |
| ignore-16bit                       |                   |
| ignore-32bit                       |                   |
| ignore-64bit                       |                   |
| ignore-aarch64                     |                   |
| ignore-aarch64-unknown-linux-gnu   |                   |
| ignore-android                     |                   |
| ignore-apple                       |                   |
| ignore-arm                         |                   |
| ignore-avr                         |                   |
| ignore-beta                        |                   |
| ignore-cdb                         |                   |
| ignore-compare-mode-next-solver    |                   |
| ignore-compare-mode-polonius       |                   |
| ignore-cross-compile               |                   |
| ignore-debug                       |                   |
| ignore-eabi                        |                   |
| ignore-emscripten                  |                   |
| ignore-endian-big                  |                   |
| ignore-freebsd                     |                   |
| ignore-fuchsia                     |                   |
| ignore-gdb                         |                   |
| ignore-gdb-version                 |                   |
| ignore-gnu                         |                   |
| ignore-haiku                       |                   |
| ignore-horizon                     |                   |
| ignore-i686-pc-windows-gnu         |                   |
| ignore-i686-pc-windows-msvc        |                   |
| ignore-illumos                     |                   |
| ignore-ios                         |                   |
| ignore-linux                       |                   |
| ignore-lldb                        |                   |
| ignore-llvm-version                |                   |
| ignore-loongarch64                 |                   |
| ignore-macabi                      |                   |
| ignore-macos                       |                   |
| ignore-mode-assembly               |                   |
| ignore-mode-codegen                |                   |
| ignore-mode-codegen-units          |                   |
| ignore-mode-coverage-map           |                   |
| ignore-mode-coverage-run           |                   |
| ignore-mode-crashes                |                   |
| ignore-mode-debuginfo              |                   |
| ignore-mode-incremental            |                   |
| ignore-mode-js-doc-test            |                   |
| ignore-mode-mir-opt                |                   |
| ignore-mode-pretty                 |                   |
| ignore-mode-run-make               |                   |
| ignore-mode-run-pass-valgrind      |                   |
| ignore-mode-rustdoc                |                   |
| ignore-mode-rustdoc-json           |                   |
| ignore-mode-ui                     |                   |
| ignore-mode-ui-fulldeps            |                   |
| ignore-msp430                      |                   |
| ignore-msvc                        |                   |
| ignore-musl                        |                   |
| ignore-netbsd                      |                   |
| ignore-nightly                     |                   |
| ignore-none                        |                   |
| ignore-nto                         |                   |
| ignore-nvptx64                     |                   |
| ignore-nvptx64-nvidia-cuda         |                   |
| ignore-openbsd                     |                   |
| ignore-pass                        |                   |
| ignore-powerpc                     |                   |
| ignore-remote                      |                   |
| ignore-riscv64                     |                   |
| ignore-s390x                       |                   |
| ignore-sgx                         |                   |
| ignore-sparc64                     |                   |
| ignore-spirv                       |                   |
| ignore-stable                      |                   |
| ignore-stage1                      |                   |
| ignore-stage2                      |                   |
| ignore-test                        |                   |
| ignore-thumb                       |                   |
| ignore-thumbv8m.base-none-eabi     |                   |
| ignore-thumbv8m.main-none-eabi     |                   |
| ignore-tvos                        |                   |
| ignore-unix                        |                   |
| ignore-unknown                     |                   |
| ignore-uwp                         |                   |
| ignore-visionos                    |                   |
| ignore-vxworks                     |                   |
| ignore-wasi                        |                   |
| ignore-wasm                        |                   |
| ignore-wasm32                      |                   |
| ignore-wasm32-bare                 |                   |
| ignore-wasm64                      |                   |
| ignore-watchos                     |                   |
| ignore-windows                     |                   |
| ignore-windows-gnu                 |                   |
| ignore-windows-msvc                |                   |
| ignore-x32                         |                   |
| ignore-x86                         |                   |
| ignore-x86_64                      |                   |
| ignore-x86_64-apple-darwin         |                   |
| ignore-x86_64-pc-windows-gnu       |                   |
| ignore-x86_64-unknown-linux-gnu    |                   |
| only-16bit                         |                   |
| only-32bit                         |                   |
| only-64bit                         |                   |
| only-aarch64                       |                   |
| only-aarch64-unknown-linux-gnu     |                   |
| only-apple                         |                   |
| only-arm                           |                   |
| only-avr                           |                   |
| only-beta                          |                   |
| only-bpf                           |                   |
| only-cdb                           |                   |
| only-gnu                           |                   |
| only-i686-pc-windows-gnu           |                   |
| only-i686-pc-windows-msvc          |                   |
| only-ios                           |                   |
| only-linux                         |                   |
| only-loongarch64                   |                   |
| only-loongarch64-unknown-linux-gnu |                   |
| only-macos                         |                   |
| only-mips                          |                   |
| only-mips64                        |                   |
| only-msp430                        |                   |
| only-msvc                          |                   |
| only-nightly                       |                   |
| only-nvptx64                       |                   |
| only-powerpc                       |                   |
| only-riscv64                       |                   |
| only-s390x                         |                   |
| only-sparc                         |                   |
| only-sparc64                       |                   |
| only-stable                        |                   |
| only-thumb                         |                   |
| only-tvos                          |                   |
| only-unix                          |                   |
| only-visionos                      |                   |
| only-wasm32                        |                   |
| only-wasm32-bare                   |                   |
| only-wasm32-wasip1                 |                   |
| only-watchos                       |                   |
| only-windows                       |                   |
| only-windows-gnu                   |                   |
| only-windows-msvc                  |                   |
| only-x86                           |                   |
| only-x86_64                        |                   |
| only-x86_64-fortanix-unknown-sgx   |                   |
| only-x86_64-pc-windows-gnu         |                   |
| only-x86_64-pc-windows-msvc        |                   |
| only-x86_64-unknown-linux-gnu      |                   |
| needs-asm-support                  |                   |
| needs-deterministic-layouts        |                   |
| needs-dlltool                      |                   |
| needs-dynamic-linking              |                   |
| needs-force-clang-based-tests      |                   |
| needs-git-hash                     |                   |
| needs-llvm-components              |                   |
| needs-llvm-zstd                    |                   |
| needs-profiler-support             |                   |
| needs-relocation-model-pic         |                   |
| needs-run-enabled                  |                   |
| needs-rust-lld                     |                   |
| needs-sanitizer-address            |                   |
| needs-sanitizer-cfi                |                   |
| needs-sanitizer-dataflow           |                   |
| needs-sanitizer-hwaddress          |                   |
| needs-sanitizer-kcfi               |                   |
| needs-sanitizer-leak               |                   |
| needs-sanitizer-memory             |                   |
| needs-sanitizer-memtag             |                   |
| needs-sanitizer-safestack          |                   |
| needs-sanitizer-shadow-call-stack  |                   |
| needs-sanitizer-support            |                   |
| needs-sanitizer-thread             |                   |
| needs-symlink                      |                   |
| needs-threads                      |                   |
| needs-unwind                       |                   |
| needs-wasmtime                     |                   |
| needs-xray                         |                   |
| no-system-llvm                     |                   |
| min-llvm-version                   |                   |
| min-system-llvm-version            |                   |

## [Environment variable headers](https://rustc-dev-guide.rust-lang.org/tests/headers.html#environment-variable-headers)

| Rustc Directive | DejaGnu Directive |
| --------------- | ----------------- |
| rustc-env       |                   |
| exec-env        |                   |
| unset-exec-env  |                   |
| unset-rustc-env |                   |

## [Miscellaneous headers](https://rustc-dev-guide.rust-lang.org/tests/headers.html#miscellaneous-headers)

| Rustc Directive       | DejaGnu Directive                            |
| --------------------- | -------------------------------------------- |
| compile-flags         |                                              |
| run-flags             |                                              |
| edition               | "dg-additional-options "-frust-edition=2018" |
| failure-status        |                                              |
| should-fail           |                                              |
| gate-test-X           |                                              |
| error-pattern         |                                              |
| incremental           |                                              |
| no-prefer-dynamic     |                                              |
| no-auto-check-cfg     |                                              |
| force-host            |                                              |
| revisions             |                                              |
| unused-revision-names |                                              |
| forbid-output         |                                              |
| should-ice            |                                              |
| known-bug             |                                              |

## [Assembly](https://rustc-dev-guide.rust-lang.org/tests/compiletest.html#assembly-tests) headers

| Rustc Directive | DejaGnu Directive |
| --------------- | ----------------- |
| assembly-output |                   |

## [Tool-specific headers](https://rustc-dev-guide.rust-lang.org/tests/headers.html#tool-specific-headers)

| Rustc Directive | DejaGnu Directive |
| --------------- | ----------------- |
| filecheck-flags |                   |
| llvm-cov-flags  |                   |
