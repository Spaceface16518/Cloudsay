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

#[cfg(test)]
mod say_query_tests {
    use super::*;

    #[test]
    fn test_get_output_cow_default() {
        let ser = SayQuery {
            character: "cow".to_string(),
            text: None,
            width: None
        };

        assert_eq!(ser.get_output().unwrap(), Output::new(br#"\   ^__^
 \  (oo)\_______
    (__)\       )\/\
        ||----w |
        ||     ||"#, "Hello from cloudsay!".to_string(), 20))
    }

    #[test]
    fn test_get_output_ferris_default() {
        let ser = SayQuery {
            character: "ferris".to_string(),
            text: None,
            width: None
        };

        assert_eq!(ser.get_output().unwrap(), Output::new(br#"\
 \
    _~^~^~_
\) /  o o  \ (/
  '_   -   _'
  / '-----' \"#, "Hello from cloudsay!".to_string(), 20))
    }

    #[test]
    fn test_get_output_cow_custom() {
        let ser = SayQuery {
            character: "cow".to_string(),
            text: Some("Custom text".to_string()),
            width: Some(10)
        };

        assert_eq!(ser.get_output().unwrap(), Output::new(br#"\   ^__^
 \  (oo)\_______
    (__)\       )\/\
        ||----w |
        ||     ||"#, "Custom text".to_string(), 10))
    }

    #[test]
    fn test_get_output_ferris_custom() {
        let ser = SayQuery {
            character: "ferris".to_string(),
            text: Some("Custom text".to_string()),
            width: Some(10)
        };

        assert_eq!(ser.get_output().unwrap(), Output::new(br#"\
 \
    _~^~^~_
\) /  o o  \ (/
  '_   -   _'
  / '-----' \"#, "Custom text".to_string(), 10))
    }
}
