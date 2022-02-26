use sqlx::database::HasArguments;
use sqlx::query::QueryAs;
use sqlx::Database;
use sqlx::Encode;
use sqlx::Type;

pub trait QueryExt<'a, DB: Database> {
    fn bind_all<Vs, V>(self, values: Vs) -> Self
    where
        V: 'a + Send + Encode<'a, DB> + Type<DB>,
        Vs: IntoIterator<Item = V>;
}

impl<'a, DB: Database, O> QueryExt<'a, DB>
    for QueryAs<'a, DB, O, <DB as HasArguments<'a>>::Arguments>
{
    fn bind_all<Vs, V>(self, values: Vs) -> Self
    where
        V: 'a + Send + Encode<'a, DB> + Type<DB>,
        Vs: IntoIterator<Item = V>,
    {
        let mut this = self;

        for value in values.into_iter() {
            this = this.bind(value);
        }

        this
    }
}
