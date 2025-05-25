/// A library of craft launcher core functionalities.
pub mod craft_launcher;

// Utils module exports
pub use crate::craft_launcher::utils::*;

// Core module exports
pub use crate::craft_launcher::core::*;

// Assets module exports
pub use crate::craft_launcher::core::assets::*;

// Version manifest parser
pub use crate::craft_launcher::core::manifest::*;

// Version module exports
pub use crate::craft_launcher::core::version::*;

// Legacy version exports
pub use crate::craft_launcher::core::version::legacy::*;

// Modern version exports
pub use crate::craft_launcher::core::version::modern::*;

// Java module exports (if any are available)
pub use crate::craft_launcher::java::*;
