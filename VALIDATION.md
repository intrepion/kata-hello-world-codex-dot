# Validation — 2026-10-26

Host: macOS arm64; Node 24.20.0, npm 11.19.0, Rust/Cargo 1.96.0,
Xcode 27.1 with Apple Swift 6.4 and iOS 27.1 SDK, OpenJDK 25.0.2.

| Project | Actual checks | Result / limitation |
| --- | --- | --- |
| web | `npm install --ignore-scripts --no-audit --no-fund`; `npm run build`; localhost preview HTTP request | Production build passed; HTTP 200 and expected Hello World heading. Temporary preview stopped. |
| cli | `cargo test`; `cargo fmt --check`; `cargo run --locked` in an interactive PTY | One TestBackend rendering test passed; greeting and exit hint appeared; q exited successfully and restored terminal. |
| android | Gradle wrapper `--version`; `:app:assembleDebug --no-daemon` with `GRADLE_USER_HOME` in `/tmp` | Gradle 9.6.0 runs on JDK 25; AGP configuration resolved. Build failed because Android SDK location was not found. No APK produced; Kotlin compilation and device execution untested. |
| ios | unsigned Debug build, generic iOS destination, `CODE_SIGNING_ALLOWED=NO`; plist and scheme parsing | BUILD SUCCEEDED. Generated unsigned `ios/build/Build/Products/Debug-iphoneos/HelloWorld.app`. No device/simulator execution; no signing or simulator installation. |
| desktop | `cargo check`; `cargo fmt --check`; `cargo build --locked`; native process smoke | Passed on stable Rust with official GPUI Kit 0.8.0. Executable built successfully; process stayed alive eight seconds without an immediate crash and was then stopped. Window appearance was not visually inspected. Windows/Linux untested. |

Android XML resources/manifest and the shared Xcode scheme parsed successfully.
Git ignore probes passed for node_modules, Astro output, Rust targets, Android
build output/local.properties, Xcode build output, and per-user Xcode data.

## Android blocker

The attempted local `android/:app:assembleDebug` task reports:

> SDK location not found. Define a valid SDK location with an ANDROID_HOME
> environment variable or by setting the sdk.dir path in your project's local
> properties file.

The previously referenced `/Users/intrepion/Library/Android/sdk` does not exist.
Required: Android SDK platform 37 and build tools 36.0.0, with licenses already
accepted. SDK auto-download is disabled. No SDK installation, SDK license
acceptance, emulator installation, persistent release keys, or Play Console
actions were performed. This is an environment blocker, not an approval denial.

## Scope

All changes are local to the new scaffold. No GitHub remote, push, publication,
web deployment, Play app, upload, store release, or retention claim is included.
No changes were made to other projects. No global toolchain was installed.

## GPUI Kit compatibility

The current 0.8.1 release failed in its pinned `gpui-pre-macos 0.3.10` dependency
with E0658 (`result_option_map_or_default`) on installed Rust 1.96.0. The official
preceding release, GPUI Kit 0.8.0, selects its matching GPUI 0.3.9 family and
passes `cargo check`. The manifest pins 0.8.0, and Cargo.lock records all matching
versions. No framework was substituted, no source was vendored or patched,
and no global Rust toolchain was changed. Cargo reports an upstream future
compatibility warning in `block 0.1.6`; this does not fail the current check.
