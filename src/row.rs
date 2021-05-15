use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let mut row = Self {
            string: String::from(slice),
            len: 0,
        };
        row.update_len();

        row
    }
}

impl Row {
    #[must_use]
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);

        let mut result = String::new();

        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                result.push(' ')
            } else {
                result.push_str(grapheme);
            }
        }

        result
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
        } else {
            let mut result: String = self.substr(None, Some(at));
            let remainder: String = self.substr(Some(at), None);
            result.push(c);
            result.push_str(&remainder);
            self.string = result;
        }
        self.update_len();
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        }

        let mut result: String = self.substr(None, Some(at));
        let remainder: String = self.substr(Some(at + 1), None);
        result.push_str(&remainder);
        self.string = result;

        self.update_len();
    }

    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.update_len();
    }

    pub fn split(&mut self, at: usize) -> Self {
        let beginning: String = self.substr(None, Some(at));
        let remainder: String = self.substr(Some(at), None);

        self.string = beginning;
        self.update_len();
        Self::from(&remainder[..])
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    pub fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count()
    }

    fn substr(&self, skip: Option<usize>, take: Option<usize>) -> String {
        let graphemes = self.string[..].graphemes(true);

        match (skip, take) {
            (None, None) => graphemes.collect(),
            (Some(s), None) => graphemes.skip(s).collect(),
            (None, Some(t)) => graphemes.take(t).collect(),
            (Some(s), Some(t)) => graphemes.skip(s).take(t).collect(),
        }
    }
}