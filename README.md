Craft Launcher Core

Craft Launcher Core is a Rust-powered cross-platform launcher core library for Minecraft.
It supports all Minecraft versions, including modded environments, and provides utilities for profile creation and management. This library is designed to be embedded into desktop applications via FFI (e.g., Flutter, C#, etc.) using a dynamic library (DLL, dylib, so).


---

Features

Supports all Minecraft versions (vanilla, Forge, Fabric, legacy to modern)

Modded instance support (modpacks, custom jars, etc.)

Cross-platform: Windows / macOS / Linux

FFI-compatible: Use from Flutter, C#, C++, etc.

Profile management: Create, delete, modify launch profiles

Download & cache assets: Handles Minecraft manifests, assets, libraries, versions

Safe and performant: Rust safety + multithreaded performance



---

Example Use Case

Embed this library into a Flutter-based desktop Minecraft launcher.

Flutter UI (Windows, macOS, Linux)
           |
           ↓
   [Craft Launcher Core (Rust DLL)]
           |
    - Launch Minecraft
    - Manage profiles
    - Download game files


---

Build Instructions

Requirements

Rust (stable)

CMake (for certain dependencies)

Visual Studio Build Tools (Windows)

Flutter (optional, for integration testing)


Build DLL (Windows example)

cargo build --release

The DLL will be located at:

target/release/craft_launcher_core.dll

Use cargo doc --open to view documentation.


---

FFI Usage

This feature is not supported yet.

---

License

MIT

---

Disclaimer

This library is not affiliated with Mojang or Microsoft.
Use it responsibly and always comply with Minecraft's EULA and distribution rules.

