/// Document state management for the LSP server
use ropey::Rope;

/// Represents an open document in the editor
pub struct Document {
    pub content: String,
    pub rope: Rope,
    pub version: i32,
}

impl Document {
    pub fn new(content: String, version: i32) -> Self {
        let rope = Rope::from_str(&content);
        Document {
            content,
            rope,
            version,
        }
    }

    pub fn update(&mut self, new_content: String, version: i32) {
        self.content = new_content;
        self.rope = Rope::from_str(&self.content);
        self.version = version;
    }

    /// Get the line at a given line number (0-indexed)
    pub fn get_line(&self, line: usize) -> Option<String> {
        if line < self.rope.len_lines() {
            Some(self.rope.line(line).to_string())
        } else {
            None
        }
    }

    /// Get the character at a given position (0-indexed line and column)
    pub fn char_at(&self, line: usize, col: usize) -> Option<char> {
        if line < self.rope.len_lines() {
            let line_start = self.rope.line_to_char(line);
            let line_text = self.rope.line(line);
            if col < line_text.len_chars() {
                return self.rope.get_char(line_start + col);
            }
        }
        None
    }

    /// Get the word at a given position (0-indexed)
    pub fn word_at(&self, line: usize, col: usize) -> Option<(String, usize, usize)> {
        let line_text = self.get_line(line)?;
        let chars: Vec<char> = line_text.chars().collect();

        if col >= chars.len() {
            return None;
        }

        // Find word boundaries
        let mut start = col;
        let mut end = col;

        // Move start backwards to find word start
        while start > 0 && is_identifier_char(chars[start - 1]) {
            start -= 1;
        }

        // Move end forwards to find word end
        while end < chars.len() && is_identifier_char(chars[end]) {
            end += 1;
        }

        if start == end {
            return None;
        }

        let word: String = chars[start..end].iter().collect();
        Some((word, start, end))
    }

    /// Convert byte offset to line and column (0-indexed)
    pub fn offset_to_position(&self, offset: usize) -> (usize, usize) {
        let line = self.rope.byte_to_line(offset.min(self.rope.len_bytes()));
        let line_start = self.rope.line_to_byte(line);
        let col = offset.saturating_sub(line_start);
        (line, col)
    }

    /// Convert line and column (0-indexed) to byte offset
    pub fn position_to_offset(&self, line: usize, col: usize) -> usize {
        if line >= self.rope.len_lines() {
            return self.rope.len_bytes();
        }
        let line_start = self.rope.line_to_byte(line);
        let line_len = self.rope.line(line).len_bytes();
        line_start + col.min(line_len)
    }
}

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_new() {
        let doc = Document::new("hello\nworld".to_string(), 1);
        assert_eq!(doc.version, 1);
        assert_eq!(doc.content, "hello\nworld");
    }

    #[test]
    fn test_get_line() {
        let doc = Document::new("line1\nline2\nline3".to_string(), 1);
        assert_eq!(doc.get_line(0), Some("line1\n".to_string()));
        assert_eq!(doc.get_line(1), Some("line2\n".to_string()));
        assert_eq!(doc.get_line(2), Some("line3".to_string()));
        assert_eq!(doc.get_line(3), None);
    }

    #[test]
    fn test_word_at() {
        let doc = Document::new("def foo_bar(x: int)".to_string(), 1);

        // "foo_bar" starts at column 4
        let (word, start, end) = doc.word_at(0, 5).unwrap();
        assert_eq!(word, "foo_bar");
        assert_eq!(start, 4);
        assert_eq!(end, 11);

        // "def"
        let (word, start, end) = doc.word_at(0, 1).unwrap();
        assert_eq!(word, "def");
        assert_eq!(start, 0);
        assert_eq!(end, 3);
    }

    #[test]
    fn test_offset_to_position() {
        let doc = Document::new("abc\ndef\nghi".to_string(), 1);
        assert_eq!(doc.offset_to_position(0), (0, 0));
        assert_eq!(doc.offset_to_position(3), (0, 3));  // At newline
        assert_eq!(doc.offset_to_position(4), (1, 0));  // Start of line 2
        assert_eq!(doc.offset_to_position(8), (2, 0));  // Start of line 3
    }
}
