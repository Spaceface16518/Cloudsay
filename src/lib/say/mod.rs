mod character;
mod statics;
pub use character::{Character, CharacterLookupError};
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
use std::{fmt, io, iter::repeat};

pub struct Output<'a> {
    character: Character,
    input: &'a [u8],
    width: usize,
}

impl<'a> Output<'a> {
    pub fn new(
        character: Character,
        input: &'a [u8],
        width: usize,
    ) -> Output<'a> {
        Output {
            character,
            input,
            width,
        }
    }

    pub fn write_to<W: io::Write>(self, mut out: W) -> io::Result<()> {
        let mut buffer = SmallVec::<[u8; BUF_SIZE]>::new();

        let bar = repeat(DASH).take(SIDE_PADDING_TOTAL + self.width);

        // Add the top bar
        buffer.extend(repeat(DASH).take(SIDE_PADDING_TOTAL + self.width));
        buffer.push(NEWLINE);

        self.input.split(|&c| c == b'\n').for_each(|l| {
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
        buffer.extend_from_slice(self.character);

        // And we're done! Write it all to output
        info!("Construction complete; writing to output");
        out.write_all(&buffer)
    }
}
