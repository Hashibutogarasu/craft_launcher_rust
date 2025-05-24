/// Legacy version (Before 1.12)
pub mod legacy;

/// Modern version (After 1.13)
pub mod modern;

/// A base version information.
/// This module has the struct which can extend version.
pub mod base_version;

/// An interface for handling different Minecraft version behaviors.
/// This trait defines methods for installing a version and validating its manifest.
pub mod version_handler;

/// A perser of Minecraft version information.
/// This module can parse the json file which likes vanilla, forge and other mod loaders.
pub mod version_parser;
