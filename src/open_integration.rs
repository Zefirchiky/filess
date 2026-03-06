use std::{io, path::Path, thread};

pub trait OpenTrait: AsRef<Path> {
    fn open(&self) -> io::Result<()> {
        open::that(self.as_ref())
    }

    fn open_detached(&self) -> io::Result<()> {
        open::that_detached(self.as_ref())
    }

    fn open_in_background(&self) -> thread::JoinHandle<io::Result<()>> {
        open::that_in_background(self.as_ref())
    }

    fn open_with(&self, app: impl AsRef<Path>) -> io::Result<()> {
        // TODO: Use Binary for app
        open::with(self.as_ref(), app.as_ref().to_string_lossy())
    }

    fn open_detached_with(&self, app: impl AsRef<Path>) -> io::Result<()> {
        open::with_detached(self.as_ref(), app.as_ref().to_string_lossy())
    }

    fn open_in_background_with(&self, app: impl AsRef<Path>) -> thread::JoinHandle<io::Result<()>> {
        open::with_in_background(self.as_ref(), app.as_ref().to_string_lossy())
    }
}
