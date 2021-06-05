use std::{fs, io::Write};

use crate::{row, Position, Row, SearchDirection};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    dirty: bool,
    pub file_name: Option<String>,
}

impl Document {
    /// # Errors
    ///
    /// Will return `std::io::Error` if it fails to get open file
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let mut rows = Vec::new();
        let contents = fs::read_to_string(filename)?;

        for value in contents.lines() {
            let mut row = Row::from(value);
            row.highlight(None);
            rows.push(row);
        }

        Ok(Self {
            rows,
            dirty: false,
            file_name: Some(filename.to_string()),
        })
    }

    pub fn save(&mut self) -> Result<(), std::io::Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }

            self.dirty = false;
        }
        Ok(())
    }

    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.rows.len() {
            return;
        }

        self.dirty = true;

        if c == '\n' {
            return self.insert_newline(at);
        }

        if at.y == self.rows.len() {
            let mut row = Row::default();
            row.insert(0, c);
            row.highlight(None);
            self.rows.push(row);
        } else {
            #[allow(clippy::indexing_slicing)]
            let row = &mut self.rows[at.y];
            row.insert(at.x, c);
            row.highlight(None);
        }
    }

    #[allow(clippy::integer_arithmetic)]
    pub fn delete(&mut self, at: &Position) {
        let len = self.rows.len();

        if at.y >= len {
            return;
        }

        self.dirty = true;

        if at.x == self.rows[at.y].len() && at.y + 1 < len {
            let next_row = self.rows.remove(at.y + 1);
            let row = &mut self.rows[at.y];
            row.highlight(None);
            row.append(&next_row);
        } else {
            let row = &mut self.rows[at.y];
            row.delete(at.x);
            row.highlight(None);
        }
    }

    pub fn insert_newline(&mut self, at: &Position) {
        if at.y > self.rows.len() {
            return;
        }

        if at.y == self.rows.len() {
            return self.rows.push(Row::default());
        }

        #[allow(clippy::indexing_slicing)]
        let current_row = &mut self.rows[at.y];
        let mut new_row = current_row.split(at.x);

        current_row.highlight(None);
        new_row.highlight(None);

        #[allow(clippy::integer_arithmetic)]
        self.rows.insert(at.y + 1, new_row);
    }

    #[must_use]
    pub fn find(&self, query: &str, at: &Position, direction: SearchDirection) -> Option<Position> {
        if at.y >= self.rows.len() {
            return None;
        }

        let mut position = at.clone();

        let (start, end) = if direction == SearchDirection::Forward {
            (at.y, self.rows.len())
        } else {
            (0, at.y.saturating_add(1))
        };

        for _ in start..end {
            if let Some(row) = self.rows.get(position.y) {
                if let Some(x) = row.find(&query, position.x, direction) {
                    position.x = x;
                    return Some(position);
                }
                if direction == SearchDirection::Forward {
                    position.y = position.y.saturating_add(1);
                    position.x = 0;
                } else {
                    position.y = position.y.saturating_sub(1);
                    position.x = self.rows[position.y].len();
                }
            } else {
                return None;
            }
        }
        None
    }

    pub fn highlight(&mut self, word: Option<&str>) {
        for row in &mut self.rows {
            row.highlight(word);
        }
    }

    #[must_use]
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    #[must_use]
    pub fn row_len(&self, index: usize) -> Option<usize> {
        self.rows.get(index).map(row::Row::len)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    #[must_use]
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}
