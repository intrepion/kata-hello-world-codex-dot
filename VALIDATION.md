# Validation — 2026-10-26–27

Host: macOS arm64; Node 24.20.0, npm 11.19.0, Rust/Cargo 1.96.0,
Xcode 27.1 with Apple Swift 6.4 and iOS 27.1 SDK, OpenJDK 25.0.2.

| Project | Actual checks | Result / limitation |
| --- | --- | --- |
| web | `npm install --ignore-scripts --no-audit --no-fund`; `npm run build`; localhost preview HTTP request | Production build passed; HTTP 200 and expected Hello World heading. Temporary preview stopped. |
| cli | `cargo test`; `cargo fmt --check`; `cargo run --locked` in an interactive PTY | One TestBackend rendering test passed; greeting and exit hint appeared; q exited successfully and restored terminal. |
| android | `:app:assembleDebug :app:testDebugUnitTest :app:lintDebug --no-daemon`; APK metadata/signature checks | Debug APK built successfully with approved SDK. Lint: zero errors, one pinned-version warning. Unit task: NO-SOURCE; no unit tests ran. No device/emulator execution. |
| ios | unsigned Debug build, generic iOS destination, `CODE_SIGNING_ALLOWED=NO`; plist and scheme parsing | BUILD SUCCEEDED. Generated unsigned `ios/build/Build/Products/Debug-iphoneos/HelloWorld.app`. No device/simulator execution; no signing or simulator installation. |
| desktop | `cargo check`; `cargo fmt --check`; `cargo build --locked`; native process smoke | Passed on stable Rust with official GPUI Kit 0.8.0. Executable built successfully; process stayed alive eight seconds without an immediate crash and was then stopped. Window appearance was not visually inspected. Windows/Linux untested. |

Android XML resources/manifest and the shared Xcode scheme parsed successfully.
Git ignore probes passed for node_modules, Astro output, Rust targets, Android
build output/local.properties, Xcode build output, and per-user Xcode data.

## Android SDK and build verification — 2026-10-27

The previous missing-SDK blocker is resolved. After explicit user approval, only
these official SDK packages were installed at `/Users/intrepion/Library/Android/sdk`:

| Package | Installed version | Official archive SHA-1 | Installed archive files checked |
| --- | --- | --- | --- |
| `cmdline-tools;23.0` (macOS ARM64) | 23.0.0 | `ad03dc49bfacfd52c110b14104ea548b8a07e830` | 139 |
| `platforms;android-37.0` | revision 2 | `ed8ebf7f8822a4de5686d427f237d2fa30ff7410` | 11,384 |
| `build-tools;36.0.0` | 36.0.0 | `199ae0047ee61e842f8ee0c6d3918e44fb9a1f83` | 169 |

Each archive matched its published size and SHA-1 from
[Google's official repository metadata](https://dl.google.com/android/repository/repository2-3.xml).
The installed archive payloads were compared byte-for-byte, including symlinks.
The official installed-package list confirms exactly these three packages.

The standard [Android SDK License Agreement](https://developer.android.com/studio/terms.html)
was accepted for these approved packages. The only license record is
`android-sdk-license`; the repository's agreement text matches the previously
reviewed text. No blanket acceptance of unrelated licenses was performed.

Measured installation: approximately 513 MiB in the SDK directory. Command-line
tools 23.0 automatically bootstrap their official Android CLI runtime in
`~/.android/cli` (approximately 150 MiB), for approximately 663 MiB combined.
Verification commands used `--no-metrics`. Temporary verification ZIPs were removed.
Free space measured approximately 68.3 GiB before and 67.1 GiB after cleanup;
the difference also includes Gradle cache growth, build output, and host activity.
No Android Studio, emulator, system image, platform-tools, NDK, or CMake was installed.

The ignored `android/local.properties` points to this SDK; no global PATH or JDK
setting was changed. SDK auto-download remains disabled. The existing debug
keystore was used by the normal debug build, and its size/modification time are
unchanged. Its contents were not directly inspected; no new release key was generated.

Reproduction on this host:

```sh
cd android
GRADLE_USER_HOME=/tmp/kata-hello-world-gradle ./gradlew \
  :app:assembleDebug :app:testDebugUnitTest :app:lintDebug --no-daemon
```

Build passed using JDK 25.0.2 and Gradle 9.6.0. Lint has zero errors and one warning
that newer Gradle 9.8.2 is available; the tested compatible 9.6.0 stays pinned.
The launcher vector icon and explicit modern/legacy backup-disabled rules resolve
the scaffold's actionable lint warnings. AGP also warns that platform-tools are
absent; they are unnecessary for this compile-only validation and were not installed.
The unit-test task reports NO-SOURCE: no unit or instrumentation tests ran.

APK: `android/app/build/outputs/apk/debug/app-debug.apk` (880,080 bytes).

SHA-256: `7df58ed7d5490b1631360d036783fb823b0ece6dc7ad3e265b0cbe32a059f59b`

Verified package `com.intrepion.katahelloworld`, version code 1 / name 1.0,
minimum API 24, target/compile API 37, debuggable, launcher MainActivity, and a DEX
payload. `apksigner verify --verbose` passed with APK Signature Scheme v2 and one
signer. The APK is for local debug testing and cannot be published to Play as-is.
Lint reports are in ignored `android/app/build/reports/lint-results-debug.*`.
No device/emulator execution, installation on a device, Play app creation,
upload, store release, or account-retention validation occurred.

## Scope

All changes are local to the new scaffold. No GitHub remote, push, publication,
web deployment, Play app, upload, store release, or retention claim is included.
No changes were made to other projects. Only the explicitly approved Android SDK
and its required CLI runtime were installed; other global toolchains are unchanged.

## GPUI Kit compatibility

The current 0.8.1 release failed in its pinned `gpui-pre-macos 0.3.10` dependency
with E0658 (`result_option_map_or_default`) on installed Rust 1.96.0. The official
preceding release, GPUI Kit 0.8.0, selects its matching GPUI 0.3.9 family and
passes `cargo check`. The manifest pins 0.8.0, and Cargo.lock records all matching
versions. No framework was substituted, no source was vendored or patched,
and no global Rust toolchain was changed. Cargo reports an upstream future
compatibility warning in `block 0.1.6`; this does not fail the current check.
