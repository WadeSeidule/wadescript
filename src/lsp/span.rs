/// Span and position utilities for LSP
use crate::lexer::SourceLocation;
use tower_lsp::lsp_types::{Position, Range};

/// A source span with start and end positions
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
    pub start_offset: usize,
    pub end_offset: usize,
}

impl Span {
    pub fn new(
        start_line: usize,
        start_column: usize,
        end_line: usize,
        end_column: usize,
    ) -> Self {
        Span {
            start_line,
            start_column,
            end_line,
            end_column,
            start_offset: 0,
            end_offset: 0,
        }
    }

    pub fn with_offsets(
        start_line: usize,
        start_column: usize,
        end_line: usize,
        end_column: usize,
        start_offset: usize,
        end_offset: usize,
    ) -> Self {
        Span {
            start_line,
            start_column,
            end_line,
            end_column,
            start_offset,
            end_offset,
        }
    }

    /// Create a span from two SourceLocations
    pub fn from_locations(start: &SourceLocation, end: &SourceLocation) -> Self {
        Span {
            start_line: start.line,
            start_column: start.column,
            end_line: end.line,
            end_column: end.column,
            start_offset: start.offset,
            end_offset: end.offset,
        }
    }

    /// Merge two spans into one that covers both
    pub fn merge(a: &Span, b: &Span) -> Self {
        let (start_line, start_column, start_offset) = if a.start_line < b.start_line
            || (a.start_line == b.start_line && a.start_column <= b.start_column)
        {
            (a.start_line, a.start_column, a.start_offset)
        } else {
            (b.start_line, b.start_column, b.start_offset)
        };

        let (end_line, end_column, end_offset) = if a.end_line > b.end_line
            || (a.end_line == b.end_line && a.end_column >= b.end_column)
        {
            (a.end_line, a.end_column, a.end_offset)
        } else {
            (b.end_line, b.end_column, b.end_offset)
        };

        Span {
            start_line,
            start_column,
            end_line,
            end_column,
            start_offset,
            end_offset,
        }
    }

    /// Convert to LSP Range (0-indexed)
    pub fn to_lsp_range(&self) -> Range {
        Range {
            start: Position {
                line: (self.start_line.saturating_sub(1)) as u32,
                character: (self.start_column.saturating_sub(1)) as u32,
            },
            end: Position {
                line: (self.end_line.saturating_sub(1)) as u32,
                character: (self.end_column.saturating_sub(1)) as u32,
            },
        }
    }

    /// Check if a position (1-indexed) is within this span
    pub fn contains(&self, line: usize, column: usize) -> bool {
        if line < self.start_line || line > self.end_line {
            return false;
        }
        if line == self.start_line && column < self.start_column {
            return false;
        }
        if line == self.end_line && column > self.end_column {
            return false;
        }
        true
    }
}

impl Default for Span {
    fn default() -> Self {
        Span {
            start_line: 1,
            start_column: 1,
            end_line: 1,
            end_column: 1,
            start_offset: 0,
            end_offset: 0,
        }
    }
}

/// Convert LSP Position (0-indexed) to WadeScript position (1-indexed)
pub fn lsp_position_to_ws(pos: &Position) -> (usize, usize) {
    ((pos.line + 1) as usize, (pos.character + 1) as usize)
}

/// Convert WadeScript position (1-indexed) to LSP Position (0-indexed)
pub fn ws_position_to_lsp(line: usize, column: usize) -> Position {
    Position {
        line: line.saturating_sub(1) as u32,
        character: column.saturating_sub(1) as u32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_merge() {
        let a = Span::new(1, 5, 1, 10);
        let b = Span::new(1, 1, 1, 8);
        let merged = Span::merge(&a, &b);
        assert_eq!(merged.start_line, 1);
        assert_eq!(merged.start_column, 1);
        assert_eq!(merged.end_line, 1);
        assert_eq!(merged.end_column, 10);
    }

    #[test]
    fn test_span_contains() {
        let span = Span::new(2, 5, 4, 10);
        assert!(span.contains(3, 1)); // Middle line
        assert!(span.contains(2, 5)); // Start
        assert!(span.contains(4, 10)); // End
        assert!(!span.contains(1, 1)); // Before
        assert!(!span.contains(5, 1)); // After
        assert!(!span.contains(2, 4)); // Before start column
    }

    #[test]
    fn test_lsp_position_conversion() {
        // LSP uses 0-indexed, WadeScript uses 1-indexed
        let lsp_pos = Position { line: 0, character: 0 };
        let (line, col) = lsp_position_to_ws(&lsp_pos);
        assert_eq!(line, 1);
        assert_eq!(col, 1);

        let ws_pos = ws_position_to_lsp(1, 1);
        assert_eq!(ws_pos.line, 0);
        assert_eq!(ws_pos.character, 0);
    }

    #[test]
    fn test_span_to_lsp_range() {
        let span = Span::new(1, 1, 2, 5);
        let range = span.to_lsp_range();
        assert_eq!(range.start.line, 0);
        assert_eq!(range.start.character, 0);
        assert_eq!(range.end.line, 1);
        assert_eq!(range.end.character, 4);
    }
}
