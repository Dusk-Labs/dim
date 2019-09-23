use serde::Serialize;

#[derive(Serialize)]
pub struct Event {
    res: String,
    message: Message,
}

#[derive(Serialize)]
pub struct Message {
    pub id: i32,
    pub event_type: PushEventType,
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum PushEventType {
    EventNewCard,
    EventRemoveCard,
    EventNewLibrary,
    EventRemoveLibrary,
}

impl<'a> Event {
    pub fn new(res: &'a str, message: Message) -> Self {
        Self {
            res: String::from(res),
            message,
        }
    }

    pub fn get_res(&self) -> String {
        self.res.clone()
    }

    pub fn build(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
