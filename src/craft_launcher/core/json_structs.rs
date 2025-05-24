pub mod json_structs {
    // Re-export version modules
    pub mod versions {
        // Modern version modules
        pub use crate::craft_launcher::core::version::modern::modern_fabric::modern_fabric;
        pub use crate::craft_launcher::core::version::modern::modern_forge::modern_forge;
        pub use crate::craft_launcher::core::version::modern::modern_neoforge::modern_neoforge;
        pub use crate::craft_launcher::core::version::modern::modern_vanilla::modern_vanilla;

        // Legacy version modules
        pub use crate::craft_launcher::core::version::legacy::legacy_fabric::legacy_fabric;
        pub use crate::craft_launcher::core::version::legacy::legacy_forge::legacy_forge;
        pub use crate::craft_launcher::core::version::legacy::legacy_vanilla::legacy_vanilla;
    }
}
