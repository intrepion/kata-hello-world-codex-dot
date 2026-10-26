# Kotlin Android Hello World

A minimal native Activity with a centered TextView. No network permission or
external runtime dependencies. Kotlin support is built into AGP 9.4.0.

Requirements: JDK 17+ compatible with Gradle 9.6.0 (host: JDK 25), an installed
Android SDK platform 37 and build tools 36.0.0, and previously accepted SDK
licenses. Set ANDROID_HOME or an ignored local.properties with sdk.dir.

```sh
./gradlew --version
./gradlew :app:assembleDebug
```

Windows: use `gradlew.bat`. Debug APK output, if built, is
`app/build/outputs/apk/debug/app-debug.apk`. Minimum Android API is 24.
SDK automatic downloads are disabled. No release keystore or Play configuration
is supplied. The initial host has no Android SDK, so APK compilation and
emulator/device execution need separate validation once compatible tooling exists.
