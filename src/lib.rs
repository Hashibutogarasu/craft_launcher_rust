/// A library of craft launcher core functionalities.
pub mod craft_launcher;

// Utils module exports
pub use crate::craft_launcher::utils::directory_operations;
pub use crate::craft_launcher::utils::file_operations;
pub use crate::craft_launcher::utils::networking;
pub use crate::craft_launcher::utils::path_operations;

// Core module exports
pub use crate::craft_launcher::core::disposable;
pub use crate::craft_launcher::core::engine;
pub use crate::craft_launcher::core::json_structs;

// Assets module exports
pub use crate::craft_launcher::core::assets::assets_parser;

// Version module exports
pub use crate::craft_launcher::core::version::base_version;
pub use crate::craft_launcher::core::version::version_handler;
pub use crate::craft_launcher::core::version::version_parser;

// Legacy version exports
pub use crate::craft_launcher::core::version::legacy::legacy_fabric;
pub use crate::craft_launcher::core::version::legacy::legacy_forge;
pub use crate::craft_launcher::core::version::legacy::legacy_vanilla;

// Modern version exports
pub use crate::craft_launcher::core::version::modern::modern_fabric;
pub use crate::craft_launcher::core::version::modern::modern_forge;
pub use crate::craft_launcher::core::version::modern::modern_neoforge;
pub use crate::craft_launcher::core::version::modern::modern_vanilla;

// Java module exports (if any are available)
// pub use crate::craft_launcher::java::*;
