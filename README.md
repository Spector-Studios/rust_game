# Rust Game

[![Build and Deploy](https://github.com/Spector-Studios/rust_game/actions/workflows/build.yml/badge.svg)](https://github.com/Spector-Studios/rust_game/actions/workflows/build.yml)

Based on the book [Hands-on Rust](https://hands-on-rust.com/2021/07/08/) by Herbert Wolverson, with following changes:
- Using [Macroquad](https://github.com/not-fl3/macroquad) for rendering
- Using [Bevy](https://github.com/bevyengine/bevy) for ECS
- Touch controls (currently no keyboard controls are present)

(This has project primarily targets mobile platforms and may not work on desktop, even the web build)

# Building
## Android
Assuming Android SDK and NDK are setup,
```sh
./gradlew assemble
```
## WASM
```
cd rustlib/rust_workspace
./wasm_build.sh
```
This will produce a folder named `dist` that should be hosted from a http server. To play locally, run `python -m http.server` from that folder.
# Credits
 - [notfl3/miniquad](https://github.com/not-fl3/miniquad) for the java code used for Android build
 - [Kenny](https://kenney.nl) for the pixel art assets
