# Kotlin Android Hello World

A minimal native Activity with a centered TextView and vector launcher icon.
Uses platform UI widgets and the Kotlin standard library; no network permission.
Kotlin support is built into AGP 9.4.0. Backup and transfer are explicitly disabled.

Requirements: JDK 17+ compatible with Gradle 9.6.0 (host: JDK 25), an installed
Android SDK platform 37.0 revision 2 and build tools 36.0.0, and accepted SDK
licenses. Set ANDROID_HOME or an ignored local.properties with sdk.dir.

```sh
./gradlew --version
./gradlew :app:assembleDebug :app:testDebugUnitTest :app:lintDebug
```

Windows: use `gradlew.bat`. Debug APK output, if built, is
`app/build/outputs/apk/debug/app-debug.apk`. Minimum Android API is 24.
SDK automatic downloads are disabled. No release keystore or Play configuration
is supplied. On 2026-10-27 the approved SDK was installed, the debug APK built,
its metadata/signature verified, and lint completed with zero errors and one
pinned-Gradle-version warning. No unit tests exist (NO-SOURCE), and device or
emulator execution remains untested. See [validation evidence](../VALIDATION.md).
