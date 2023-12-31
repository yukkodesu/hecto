use std::fs;

use crate::{Position, Row};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for line in contents.lines() {
            rows.push(Row::from(line));
        }
        Ok(Document {
            rows,
            file_name: Some(filename.to_string()),
        })
    }
    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y == self.len() {
            let mut row = Row::default();
            row.insert(0,c);
            self.rows.push(row);
        } else if at.y < self.len() {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
        }
    }
    pub fn delete(&mut self, at: &Position) {
        if at.y > self.len() {
            return
        }
        let row = self.rows.get_mut(at.y).unwrap();
        row.delete(at.x);
    }
    pub fn row(&self, idx: usize) -> Option<&Row> {
        self.rows.get(idx)
    }
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }
}
