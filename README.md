# kata-hello-world-codex-dot

Five independent, minimal Hello World applications, licensed under MIT.

| Folder | Stack | Run / build from that folder |
| --- | --- | --- |
| `web/` | Astro 7.4.2 | `npm ci && npm run dev`; production: `npm run build` |
| `cli/` | Rust + Ratatui 0.30.2 | `cargo run --locked`; quit with q or Esc |
| `android/` | Kotlin + Android Gradle Plugin 9.4.0 | `./gradlew :app:assembleDebug` with an installed Android SDK |
| `ios/` | Swift 6 + SwiftUI | Open `HelloWorld.xcodeproj` in Xcode and run the HelloWorld scheme |
| `desktop/` | Rust + GPUI Kit 0.8.0 | `cargo run --locked` on a supported desktop |

## Requirements and reproducibility

Validated tooling on the scaffold host: Node 24.20.0, npm 11.19.0, Rust/Cargo
1.96.0, Xcode 27.1 (iOS SDK 27.1), OpenJDK 25.0.2. Web includes
`package-lock.json`; both Rust applications include `Cargo.lock`. Android pins
AGP and Gradle, includes the official Gradle 9.6.0 wrapper, and verifies its
distribution SHA-256. AGP supplies built-in Kotlin. SwiftUI has no external
package dependencies.

Android requires Android SDK platform 37 and build tools 36.0.0. Point
`ANDROID_HOME` to an existing compatible SDK, or create an ignored
`android/local.properties` containing `sdk.dir=/absolute/path/to/sdk`.
SDK auto-download is disabled. This scaffold does not accept SDK licenses.
The existing host debug keystore may be used for a debug build; no release
signing configuration is included. Review the application/bundle identifiers
before any eventual store registration.

For an unsigned iOS compile using the installed device SDK, without a simulator:

```sh
cd ios
xcodebuild -project HelloWorld.xcodeproj -scheme HelloWorld   -configuration Debug -destination 'generic/platform=iOS'   -derivedDataPath build CODE_SIGNING_ALLOWED=NO build
```

An unsigned build cannot be installed on a device. Running on a physical device
requires configuring your own signing team in Xcode. No simulator runtimes are
installed by this project.

The `.gitignore` combines the official GitHub `Global/macOS.gitignore`,
`Global/Windows.gitignore`, and `Global/Linux.gitignore` templates, plus generated
files for these projects. Locks, Gradle wrapper files, source, and Xcode project
configuration remain tracked.

## Validation

See [VALIDATION.md](VALIDATION.md) for actual checks and platform limitations.

## Official references

- [Astro installation](https://docs.astro.build/en/install-and-setup/)
- [Ratatui Hello World](https://ratatui.rs/tutorials/hello-ratatui/)
- [Android Gradle Plugin compatibility](https://developer.android.com/build/releases/agp-9-4-0-release-notes)
- [AGP built-in Kotlin](https://developer.android.com/build/migrate-to-built-in-kotlin)
- [SwiftUI tutorial](https://developer.apple.com/tutorials/swiftui/creating-and-combining-views)
- [GPUI Kit getting started](https://gpui-kit.com/docs/getting-started/)
- [GPUI Kit upstream](https://github.com/longbridge/gpui-kit)
- [GitHub OS gitignore templates](https://github.com/github/gitignore/tree/main/Global)

GPUI Kit is Longbridge's `gpui-kit` crate, formerly GPUI Component; this project
uses its application facade and default GPUI backend.

Deployment and store publication configuration are outside this scaffold.

## Continuous integration

GitHub Actions builds the Astro site, runs Rust CLI formatting and its rendering
unit test, compiles the GPUI desktop app on macOS, compiles iOS without signing,
and builds/lints the Android debug APK. Android currently has no unit-test source;
its unit-test Gradle task must not be interpreted as executed test coverage.
Native jobs are compile checks, not interactive UI, simulator, or device tests.
No workflow deploys or publishes an application. SDK installation does not
blanket-accept licenses; it relies on the hosted runner's existing license records.

Workflow actions are recorded in `.github/workflows/actions.lock`. After changing
an action, run `gh actions-lock` (official `github/gh-actions-lock` v0.4.1), then
`gh actions-lock --verify`, and commit the workflow and lockfile together.
