use crate::lib::say::{get_character, CharacterLookupError, Output};
use actix_web::{http::StatusCode, Query};
use log::trace;
use serde::Deserialize;

const DEF_WIDTH: usize = 20;
const DEF_TEXT: &str = "Hello from cloudsay!";

#[derive(Deserialize, Debug)]
pub struct SayQuery {
    character: String,
    text: Option<String>,
    width: Option<usize>,
}

impl SayQuery {
    pub fn get_output(&self) -> Result<Output, CharacterLookupError> {
        match get_character(&self.character) {
            Ok(c) => {
                Ok(Output::new(
                    c,
                    match &self.text {
                        Some(t) => t.clone(),
                        None => DEF_TEXT.to_string(),
                    },
                    self.width.unwrap_or(DEF_WIDTH),
                ))
            },
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
