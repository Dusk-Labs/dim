#[macro_export]
macro_rules! opt_update {
    ($conn:ident, $query:expr => ($self:expr, $constraint:expr)) => {
        {
            if let Some(x) = $self.as_ref() {
                ::sqlx::query!($query, x, $constraint)
                    .execute(&mut **$conn)
                    .await?;
            }
        }
    };
    ($conn:ident, $query:expr => ($self:expr, $constraint:expr), $($tail:tt)+) => {
        {
            crate::opt_update!($conn, $query => ($self, $constraint));
            crate::opt_update!($conn, $($tail)*);
        }
    }
}

#[cfg(not(debug_assertions))]
pub fn ffpath(bin: impl AsRef<str>) -> &'static str {
    let mut path = std::env::current_exe().expect("Failed to grab path to the `dim` binary.");
    path.pop(); // remove the dim bin to get the dir of `dim`
    path.push(bin.as_ref());

    Box::leak(path.to_string_lossy().to_string().into_boxed_str())
}

#[cfg(debug_assertions)]
pub fn ffpath(bin: impl AsRef<str>) -> &'static str {
    Box::leak(bin.as_ref().to_string().into_boxed_str())
}
