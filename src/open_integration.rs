use std::{io, path::Path, thread};

/// Trait for opening files/dirs with the system default or a specific application.
pub trait OpenTrait: AsRef<Path> {
    /// Opens the file/dir with the default application (blocking).
    fn open(&self) -> io::Result<()> {
        open::that(self.as_ref())
    }

    /// Opens the file/dir with the default application (non-blocking).
    fn open_detached(&self) -> io::Result<()> {
        open::that_detached(self.as_ref())
    }

    /// Opens the file/dir in a background thread.
    fn open_in_background(&self) -> thread::JoinHandle<io::Result<()>> {
        open::that_in_background(self.as_ref())
    }

    /// Opens the file/dir with a specific application (blocking).
    fn open_with(&self, app: impl AsRef<Path>) -> io::Result<()> {
        // TODO: Use Binary for app
        open::with(self.as_ref(), app.as_ref().to_string_lossy())
    }

    /// Opens the file/dir with a specific application (non-blocking).
    fn open_detached_with(&self, app: impl AsRef<Path>) -> io::Result<()> {
        open::with_detached(self.as_ref(), app.as_ref().to_string_lossy())
    }

    /// Opens the file/dir with a specific application in a background thread.
    fn open_in_background_with(&self, app: impl AsRef<Path>) -> thread::JoinHandle<io::Result<()>> {
        open::with_in_background(self.as_ref(), app.as_ref().to_string_lossy())
    }
}
