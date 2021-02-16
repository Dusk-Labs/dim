pub struct MovieScanner {
    conn: DbConnection,
    lib: Library,
    log: Logger,
    event_tx: EventTx,
}

impl MediaScanner for MovieScanner {
    const MEDIA_TYPE: library::MediaType = library::MediaType::Movie;

    fn new_unchecked(conn: DbConnection, lib: Library, log: Logger, event_tx: EventTx) -> Self {
        Self {
            conn,
            lib,
            log,
            event_tx,
        }
    }

    fn logger_ref(&self) -> &Logger {
        &self.logger
    }
}
