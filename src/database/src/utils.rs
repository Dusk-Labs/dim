#[macro_export]
macro_rules! retry_while {
    ($err:pat, $tx:block) => {{
        loop {
            let _result = $tx;

            match _result {
                Err(::tokio_diesel::AsyncError::Error(::diesel::result::Error::DatabaseError(
                    $err,
                    _,
                ))) => {
                    continue;
                }
                _ => break _result,
            }
        }
    }};
}
