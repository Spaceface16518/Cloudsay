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

#[derive(Debug)]
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

        // Add the character itself
        let offset = (SIDE_PADDING_TOTAL + self.width) / 2;
        self.character.split(|&b| b == b'\n').for_each(|l| {
            buffer.extend(repeat(SPACE).take(offset));
            buffer.extend_from_slice(l);
            buffer.push(b'\n');
        });

        // And we're done! Write it all to output
        info!("Construction complete; writing to output");

        String::from_utf8_lossy(buffer.as_slice()).to_string()
    }
}