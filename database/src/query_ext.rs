use sqlx::database::HasArguments;
use sqlx::query::QueryAs;
use sqlx::Database;
use sqlx::Encode;
use sqlx::Type;

/// Trait contains some extensions for `sqlx`.
pub trait QueryExt<'a, DB: Database> {
    /// Method which allows you to bind several values in one go. This method will accept any
    /// container which can be turned into an Iterator of values.
    fn bind_all<Vs, V>(self, values: Vs) -> Self
    where
        V: Send + Encode<'a, DB> + Type<DB> + 'a,
        Vs: IntoIterator<Item = V>;
}

impl<'a, DB: Database, O> QueryExt<'a, DB>
    for QueryAs<'a, DB, O, <DB as HasArguments<'a>>::Arguments>
{
    fn bind_all<Vs, V>(self, values: Vs) -> Self
    where
        V: Send + Encode<'a, DB> + Type<DB> + 'a,
        Vs: IntoIterator<Item = V>,
    {
        let mut this = self;

        for value in values.into_iter() {
            this = this.bind(value);
        }

        this
    }
}
