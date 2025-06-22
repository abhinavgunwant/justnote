use std::string::FromUtf8Error;

use crate::SpecVersion;

#[derive(Debug, Default, Clone)]
pub struct Note {
    /// The id of the note
    pub id: u32,

    /// The version of the note (default: 0)
    pub spec_version: SpecVersion,

    /// Whether the text is encrypted or is plain utf-8 string
    pub enc: bool,

    /// The title of the note (stored encrypted)
    pub title: Vec<u8>,

    /// The text of the note (stored encrypted)
    pub text: Vec<u8>,
}

impl Note {
    pub fn new(
        id: u32, title_str: String, text_str: String, enc: bool,
    ) -> Self {
        let title = Vec::from(title_str.as_bytes());
        let text = Vec::from(text_str.as_bytes());

        if enc {
            Self { id, title, text, enc: true, ..Self::default() }
        } else {
            Self { id, title, text, ..Self::default() }
        }
    }

    /// Gets the text from the bytes
    pub fn text(&self) -> Result<String, FromUtf8Error> { b2s(&self.text) }

    /// Gets the title from the bytes
    pub fn title(&self) -> Result<String, FromUtf8Error> { b2s(&self.title) }

    pub fn set_text(&mut self, text_str: String) {
        self.text = Vec::from(text_str.as_bytes());
    }

    pub fn set_title(&mut self, title_str: String) {
        self.title = Vec::from(title_str.as_bytes());
    }
}

/// Bytes to String.
///
/// Takes bytes as input as outputs the string trimming all the null characters
/// at the end.
///
/// These null characters are added by the function
/// `vault::files::notes::create_blocks`.
fn b2s(data: &Vec<u8>) -> Result<String, FromUtf8Error> {
    match String::from_utf8(data.to_vec()) {
        Ok(text) => Ok(text.trim_end_matches('\0').to_owned()),
        Err(e) => Err(e),
    }
}

