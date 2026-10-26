# SwiftUI iOS Hello World

Open HelloWorld.xcodeproj in Xcode, select the shared HelloWorld scheme, and
choose your own physical device/signing team or an already installed simulator.
The app targets iOS 17+ and uses no external packages.

Unsigned compile with the installed iOS device SDK:

```sh
xcodebuild -project HelloWorld.xcodeproj -scheme HelloWorld   -configuration Debug -destination 'generic/platform=iOS'   -derivedDataPath build CODE_SIGNING_ALLOWED=NO build
```

This compiles without provisioning or installing simulator runtimes. Device
installation and runtime UI checks require a suitable destination and signing.
