/// A disposable interface
/// For example, this trait used for launcher module, asset downloader, library exporter and other modules.
pub trait Disposable {
    /// Dispose a self module.
    /// This function will dispose used resources.
    fn dispose(&mut self);
}