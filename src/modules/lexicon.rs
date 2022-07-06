use serde::Serialize;
use crate::modules::parts_of_speech::PartOfSpeech;

struct Word {
    spelling: String,
    pos: String,
    definitions: Vec<String>,
}
impl Word {
    pub fn pos_from_string() {} // TODO
}