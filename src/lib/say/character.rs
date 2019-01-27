use super::statics;
use log::{error, trace};

pub type Character = &'static [u8];

#[inline]
pub const fn cow() -> Character { statics::COW }

#[inline]
pub const fn ferris() -> Character { statics::FERRIS }

#[inline]
pub fn get_character(name: &str) -> Result<Character, CharacterLookupError> {
    trace!("Looking up character");
    match name {
        "cow" => Ok(cow()),
        "ferris" => Ok(ferris()),
        _ => {
            error!("Character lookup failed");
            Err(CharacterLookupError {
                attempt: name.to_string(),
            })
        },
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
        assert_eq!(get_character("cow").unwrap(), cow());
    }

    #[test]
    fn test_get_character_input_ferris() {
        assert_eq!(get_character("ferris").unwrap(), ferris());
    }
}
