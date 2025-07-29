use rusqlite::hooks::Action;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Table {
    Library,
    Media,
    Assets,
}

impl TryFrom<&str> for Table {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "library" => Ok(Self::Library),
            "_tblmedia" => Ok(Self::Media),
            "assets" => Ok(Self::Assets),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum EventType {
    Insert,
    Update,
    Delete,
}

impl From<Action> for EventType {
    fn from(action: Action) -> EventType {
        match action {
            Action::SQLITE_INSERT => EventType::Insert,
            Action::SQLITE_UPDATE => EventType::Update,
            Action::SQLITE_DELETE => EventType::Delete,
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Event {
    pub id: i64,
    pub event_type: EventType,
    pub table: Table,
}
