use super::statics;
use log::error;

pub type Character = &'static [u8];

#[inline]
pub const fn cow() -> Character { statics::COW }

#[inline]
pub const fn ferris() -> Character { statics::FERRIS }

#[inline]
fn get_character<'a>(
    name: &'a str,
) -> Result<Character, CharacterLookupError<'a>> {
    match name {
        "cow" => Ok(cow()),
        "ferris" => Ok(ferris()),
        _ => {
            error!("Character lookup failed");
            Err(CharacterLookupError {
                attempt: name,
            })
        },
    }
}

#[derive(Debug, Clone)]
pub struct CharacterLookupError<'a> {
    attempt: &'a str,
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
