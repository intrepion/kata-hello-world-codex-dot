# GPUI Kit Hello World

Uses the official Longbridge [GPUI Kit](https://github.com/longbridge/gpui-kit)
0.8.0 application facade, including the styled component layer.

```sh
cargo check --locked
cargo run --locked
```

A native window displays Hello, World! Close the window to exit. See the
[official platform requirements](https://gpui-kit.com/docs/installation/)
for macOS, Windows, and Linux. This project does not install global toolchains
or system libraries. Other desktop platforms require their own build validation.

Version 0.8.0 is intentionally pinned: 0.8.1's macOS snapshot uses a Rust API
that is unstable on the installed stable Rust 1.96.0. This official prior release
checks successfully without patches or a toolchain installation. Use `--locked`
to retain the validated matching GPUI dependency family.
