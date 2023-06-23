pub struct Row {
    string: String,
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        Row {
            string: String::from(value),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> &str {
        let end = end.min(self.string.len());
        let start = start.min(end);
        self.string.get(start..end).unwrap_or_default()
    }
}
