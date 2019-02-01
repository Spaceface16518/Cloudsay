mod character;
mod statics;
pub use character::{get_character, Character, CharacterLookupError};
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
use std::iter::{once, repeat, FromIterator};

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
        let mut buffer = SmallVec::<[char; BUF_SIZE]>::new();

        // Add the top bar
        buffer.extend(repeat(DASH).take(SIDE_PADDING_TOTAL + self.width));
        buffer.push(NEWLINE);

        let mut last_idx = 0;
        let chunks = {
            self.input
                .char_indices()
                .step_by(self.width)
                .map(Some) // hack to get last slice
                .chain(once(None))
                .filter_map(|next_idx| {
                    match next_idx {
                        Some(next_idx) => {
                            let slice = self.input.get(last_idx..next_idx.0);
                            last_idx = next_idx.0;
                            Some(slice)
                        },
                        None => {
                            // grab last nonconforming slice
                            if last_idx != self.input.len() {
                                Some(self.input.get(last_idx..))
                            } else {
                                None
                            }
                        },
                    }
                })
        };
        chunks.for_each(|chunk| {
            let chunk = chunk.unwrap();
            // Add the left side bar
            buffer.extend(ENDSL.chars());

            // Add the actual text line
            buffer.extend(chunk.chars());

            // Add the extra spaces
            buffer.extend(
                repeat(SPACE)
                    .take(self.width.checked_sub(chunk.len()).unwrap_or(0)),
            );

            // Close with the right side bar (and a newline)
            buffer.extend(ENDSR.chars());
        });

        // Add the bottom bar to the speech bubble
        buffer.extend(repeat(DASH).take(SIDE_PADDING_TOTAL + self.width));

        buffer.push('\n');

        // BUG: this is adding an extra set of spaces and a newline, for some
        // reason Add the character itself
        let offset = (SIDE_PADDING_TOTAL + self.width) / 2;
        self.character.split(|b| b == '\n').for_each(|l| {
            buffer.extend(repeat(SPACE).take(offset));
            buffer.extend(l.chars());
            buffer.push('\n');
        });

        // And we're done! Write it all to output
        String::from_iter(buffer.drain())
    }
}

#[cfg(test)]
mod say_tests {
    use super::*;

    #[test]
    fn test_new() {
        let actual = Output::new(statics::COW, "test".to_string(), 20);
        assert_eq!(
            actual,
            Output {
                character: statics::COW,
                input: "test".to_string(),
                width: 20
            }
        )
    }

    // TODO: add some actual tests for Output::as_string

    #[test]
    fn test_as_string_cow_default() {
        let output = Output {
            character: statics::COW,
            input: "Hello from cloudsay!".to_string(),
            width: 20,
        };

        // For now, just print to stdout. Will add actual tests later
        println!("{}", output.as_string())
    }

    #[test]
    fn test_as_string_ferris_default() {
        let output = Output {
            character: statics::FERRIS,
            input: "Hello from cloudsay!".to_string(),
            width: 20,
        };

        println!("{}", output.as_string())
    }
}
