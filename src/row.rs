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
    pub fn len(&self) -> usize {
        self.string.len()
    }
    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }
    pub fn len_no_whitespace(&self) -> usize {
        self.string.trim_start().len()
    }
}
