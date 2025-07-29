pub trait ResultExt<T, E> {
    fn inspect_err(self, f: impl FnOnce(&E)) -> Result<T, E>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn inspect_err(self, f: impl FnOnce(&E)) -> Result<T, E> {
        if let Err(ref e) = self {
            f(e);
        }

        self
    }
}
