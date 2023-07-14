use unicode_segmentation::UnicodeSegmentation;

pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        let mut row = Self {
            string: String::from(value),
            len: 0,
        };
        row.update_len();
        row
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = end.min(self.string.len());
        let start = start.min(end);
        // self.string.get(start..end).unwrap_or_default()
        let mut res = String::new();
        for grapheme in self.string.graphemes(true).skip(start).take(end - start) {
            if grapheme == "\t" {
                res.push_str("    ");
                continue;
            }
            res.push_str(grapheme);
        }
        res
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }
    pub fn len_no_whitespace(&self) -> usize {
        self.string.trim_start().len()
    }
    fn update_len(&mut self) {
        self.len = self.string.graphemes(true).count();
    }
}
