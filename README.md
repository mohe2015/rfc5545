# rfc5545

https://datatracker.ietf.org/doc/html/rfc5545

## Building

https://doc.rust-lang.org/rustc/linker-plugin-lto.html

RUSTFLAGS="-C target-cpu=native -Clinker-plugin-lto" cargo build --release

RUSTFLAGS="-C target-cpu=native -Clinker-plugin-lto -Clinker=clang -Clink-arg=-fuse-ld=lld" cargo build --release

TODO https://doc.rust-lang.org/rustc/profile-guided-optimization.html

https://github.com/rust-lang/rfcs/pull/3140/files

https://github.com/rust-lang/rust/blob/7b0e554ee2c94e9b3865a8c2d24d720224512dec/library/std/src/error.rs#L7 sad

https://github.com/rust-lang/rust/issues/62502

https://github.com/rust-lang/rust/issues/53487

https://github.com/rust-lang/project-error-handling/issues/11

## Goals

The main goal is that we create as efficient code as possible.

Currently it doesn't seem like e.g. write_all calls are merged.

Parsing should have an event-based parser and a simpler getter based interface.