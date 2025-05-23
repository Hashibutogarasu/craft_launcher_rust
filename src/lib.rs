/// A library of craft launcher core functionalities.
pub mod craft_launcher {
    /// An utility functions, structs, logics, etc...
    pub mod utils {
        /// This module can modify directory.
        pub mod directory_operations;

        /// This module can modify file.
        pub mod file_operations;

        /// This module can get a file from internet.
        pub mod networking;

        /// This module can get temporal directory.
        pub mod path_operations;
    }

    /// Launcher core module.
    /// This module can allows launching Minecraft from other programming languages.
    /// For example, if this library was built as a Windows DLL, you can use it from Flutter,
    /// C# Windows Applications, and other environments
    pub mod core {
        pub mod disposable;

        /// Disposable interface
        pub mod engine;

        /// 🚧 Work In Progress: Experimental module.
        /// JSON serializable structs
        pub mod json_structs;

        /// Version information of Minecraft
        pub mod version {
            /// Legacy version (Before 1.12)
            pub mod legacy {
                pub mod legacy_forge;
                pub mod legacy_vanilla;
            }

            /// Modern version (After 1.13)
            pub mod modern {
                pub mod modern_vanilla;
            }

            /// A base version information.
            /// This module has the struct which can extend version.
            pub mod base_version;

            /// An interface for handling different Minecraft version behaviors.
            /// This trait defines methods for installing a version and validating its manifest.
            pub mod version_handler;
        }
    }
}
