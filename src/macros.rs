#[macro_export]
macro_rules! insert {
    ($conn:expr, $table:expr, $aggregate:expr) => {
        diesel::insert_into($table)
            .values(&$aggregate)
            .execute($conn);
    };
}
