use std::fs;

use crate::Row;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let mut rows = Vec::new();
        let contents = fs::read_to_string(filename)?;

        for value in contents.lines() {
            rows.push(Row::from(value));
        }

        Ok(Self { rows })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}
