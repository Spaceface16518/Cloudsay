use crate::lib::say::{get_character, CharacterLookupError, Output};
use actix_web::{http::StatusCode, Query};
use log::trace;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SayQuery {
    character: String,
    text: String,
    width: usize,
}

impl SayQuery {
    pub fn get_output(&self) -> Result<Output, CharacterLookupError> {
        // let lookup = self.character.clone();
        match get_character(&self.character) {
            Ok(c) => Ok(Output::new(c, self.text.clone(), self.width)),
            Err(e) => Err(e),
        }
    }
}

pub fn handler(params: Query<SayQuery>) -> String {
    trace!("Request recieved");
    match params.get_output() {
        Ok(output_struct) => output_struct.as_string(),
        Err(_) => StatusCode::NOT_FOUND.to_string(),
    }
}
