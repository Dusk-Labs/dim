use sqlx::Pool;
use sqlx::Sqlite;

#[derive(Clone)]
pub struct SqlitePool {
    writer: Pool<Sqlite>,
    reader: Pool<Sqlite>,
}

impl SqlitePool {
    pub fn new(writer: Pool<Sqlite>, reader: Pool<Sqlite>) -> Self {
        Self { writer, reader }
    }

    pub fn read(&self) -> Pool<Sqlite> {
        self.reader.clone()
    }

    pub fn write(&self) -> Pool<Sqlite> {
        self.writer.clone()
    }

    pub fn read_ref(&self) -> &Pool<Sqlite> {
        &self.reader
    }

    pub fn write_ref(&self) -> &Pool<Sqlite> {
        &self.writer
    }
}
