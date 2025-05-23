/// An interface for handling different Minecraft version behaviors.
/// This trait defines methods for installing a version and validating its manifest.
pub trait VersionHandler {
    /// Attempts to install the version by downloading the client JAR
    /// based on the provided version information JSON.
    /// Returns an error message if the installation fails.
    fn install(&self) -> Result<(), String>;

    /// Validates the version's manifest information.
    /// Returns `false` if the validation fails.
    fn validate(&self) -> bool;
}
