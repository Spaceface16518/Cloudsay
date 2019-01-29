use super::statics;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use log::{error, trace};

pub type Character = &'static [u8];

lazy_static! {
    static ref CHAR_TABLE: HashMap<&'static str, &'static [u8]> = {
        let mut tmp = HashMap::with_capacity(2);
        tmp.insert("cow", statics::COW);
        tmp.insert("ferris", statics::FERRIS);
        tmp
    };
}

#[inline]
pub fn get_character(name: &str) -> Result<Character, CharacterLookupError> {
    trace!("Looking up character");
    // match name {
    //     "cow" => Ok(cow()),
    //     "ferris" => Ok(ferris()),
    //     _ => {
    //         error!("Character lookup failed");
    //         Err(CharacterLookupError {
    //             attempt: name.to_string(),
    //         })
    //     },
    // }

    if let Some(&c) = CHAR_TABLE.get(name) {
        Ok(c)
    } else {
        error!("Character lookup failed");
        Err(CharacterLookupError {
            attempt: name.to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct CharacterLookupError {
    attempt: String,
}

#[cfg(test)]
mod character_tests {
    use super::*;

    #[test]
    fn test_get_character_positive() {
        get_character("cow").unwrap();
        get_character("ferris").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_get_character_negative() {
        get_character("not_a_character").unwrap();
    }

    #[test]
    fn test_get_character_input_cow() {
        assert_eq!(get_character("cow").unwrap(), statics::COW);
    }

    #[test]
    fn test_get_character_input_ferris() {
        assert_eq!(get_character("ferris").unwrap(), statics::FERRIS);
    }
}
