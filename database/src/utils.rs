#[macro_export]
macro_rules! opt_update {
    ($conn:ident, $tx:ident, $query:expr => ($self:expr, $constraint:expr)) => {
        {
            if let Some(x) = $self.as_ref() {
                let result = ::sqlx::query!($query, x, $constraint)
                    .execute($conn)
                    .await;


                if let ::std::result::Result::Err(e) = result {
                        $tx.rollback().await?;
                        return Err(crate::DatabaseError::DatabaseError(e));
                }
            }
        }
    };
    ($conn:ident, $tx:ident, $query:expr => ($self:expr, $constraint:expr), $($tail:tt)+) => {
        {
            crate::opt_update!($conn, $tx, $query => ($self, $constraint));
            crate::opt_update!($conn, $tx, $($tail)*);
        }
    }
}
