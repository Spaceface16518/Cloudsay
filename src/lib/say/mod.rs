mod character;
mod statics;
pub use character::{get_character, Character, CharacterLookupError};
use log::info;
use smallvec::SmallVec;
use statics::{
    BUF_SIZE,
    DASH,
    ENDSL,
    ENDSR,
    NEWLINE,
    SIDE_PADDING_TOTAL,
    SPACE,
};
use std::iter::repeat;

#[derive(Debug, PartialEq, Clone)]
pub struct Output {
    character: Character,
    input: String,
    width: usize,
}

impl Output {
    pub fn new(character: Character, input: String, width: usize) -> Output {
        Output {
            character,
            input,
            width,
        }
    }

    pub fn as_string(self) -> String {
        let mut buffer = SmallVec::<[u8; BUF_SIZE]>::new();

        // Add the top bar
        buffer.extend(repeat(DASH).take(SIDE_PADDING_TOTAL + self.width));
        buffer.push(NEWLINE);

        self.input.lines().map(str::as_bytes).for_each(|l| {
            l.chunks(self.width).for_each(|chunk| {
                // Add the left side bar
                buffer.extend_from_slice(ENDSL);

                // Add the actual text line
                buffer.extend_from_slice(chunk);

                // Add the extra spaces
                buffer.extend(repeat(SPACE).take(self.width - chunk.len()));

                // Close with the right side bar (and a newline)
                buffer.extend_from_slice(ENDSR);
            })
        });

        // Add the bottom bar to the speech bubble
        buffer.extend(repeat(DASH).take(SIDE_PADDING_TOTAL + self.width));

        buffer.push(b'\n');

        // BUG: this is adding an extra set of spaces and a newline, for some reason
        // Add the character itself
        let offset = (SIDE_PADDING_TOTAL + self.width) / 2;
        self.character.split(|&b| b == b'\n').for_each(|l| {
            buffer.extend(repeat(SPACE).take(offset));
            buffer.extend_from_slice(l);
            buffer.push(b'\n');
        });

        // And we're done! Write it all to output
        String::from_utf8_lossy(buffer.as_slice()).to_string()
    }
}

#[cfg(test)]
mod say_tests {
    use super::*;

    #[test]
    fn test_new() {
        let actual = Output::new(statics::COW, "test".to_string(), 20);
        assert_eq!(actual, Output {
            character: statics::COW,
            input: "test".to_string(),
            width: 20
        })
    }

    // TODO: add some actual tests for Output::as_string

    #[test]
    fn test_as_string_cow_default() {
        let output = Output {
            character: statics::COW,
            input: "Hello from cloudsay!".to_string(),
            width: 20
        };

        // For now, just print to stdout. Will add actual tests later
        println!("{}", output.as_string())
    }

    #[test]
    fn test_as_string_ferris_default() {
        let output = Output {
            character: statics::FERRIS,
            input: "Hello from cloudsay!".to_string(),
            width: 20
        };

        println!("{}", output.as_string())
    }
}