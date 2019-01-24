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
