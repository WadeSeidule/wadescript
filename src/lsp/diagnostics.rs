/// Convert WadeScript errors to LSP diagnostics
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

/// A parse or type error from the WadeScript compiler
#[derive(Debug, Clone)]
pub struct WsError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub severity: WsErrorSeverity,
}

#[derive(Debug, Clone, Copy)]
pub enum WsErrorSeverity {
    Error,
    Warning,
    Info,
}

impl WsError {
    pub fn error(message: String, line: usize, column: usize) -> Self {
        WsError {
            message,
            line,
            column,
            severity: WsErrorSeverity::Error,
        }
    }

    pub fn warning(message: String, line: usize, column: usize) -> Self {
        WsError {
            message,
            line,
            column,
            severity: WsErrorSeverity::Warning,
        }
    }

    /// Convert to LSP Diagnostic
    pub fn to_diagnostic(&self) -> Diagnostic {
        // WadeScript uses 1-indexed, LSP uses 0-indexed
        let line = self.line.saturating_sub(1) as u32;
        let col = self.column.saturating_sub(1) as u32;

        Diagnostic {
            range: Range {
                start: Position {
                    line,
                    character: col,
                },
                end: Position {
                    line,
                    character: col + 1, // Highlight at least one character
                },
            },
            severity: Some(match self.severity {
                WsErrorSeverity::Error => DiagnosticSeverity::ERROR,
                WsErrorSeverity::Warning => DiagnosticSeverity::WARNING,
                WsErrorSeverity::Info => DiagnosticSeverity::INFORMATION,
            }),
            source: Some("wadescript".to_string()),
            message: self.message.clone(),
            ..Default::default()
        }
    }
}

/// Parse error messages from the compiler output and convert to WsErrors
pub fn parse_error_message(error: &str) -> Option<WsError> {
    // Try to parse error messages in various formats
    // Format: "Error at line X, column Y: message"
    // Format: "line X: message"
    // Format: "Type error at line X: message"

    if let Some(rest) = error.strip_prefix("Error at line ") {
        if let Some((pos_part, message)) = rest.split_once(": ") {
            if let Some((line_str, col_str)) = pos_part.split_once(", column ") {
                if let (Ok(line), Ok(col)) = (line_str.parse::<usize>(), col_str.parse::<usize>()) {
                    return Some(WsError::error(message.to_string(), line, col));
                }
            }
        }
    }

    if let Some(rest) = error.strip_prefix("Type error at line ") {
        if let Some((line_str, message)) = rest.split_once(": ") {
            if let Ok(line) = line_str.parse::<usize>() {
                return Some(WsError::error(message.to_string(), line, 1));
            }
        }
    }

    if let Some(rest) = error.strip_prefix("line ") {
        if let Some((line_str, message)) = rest.split_once(": ") {
            if let Ok(line) = line_str.parse::<usize>() {
                return Some(WsError::error(message.to_string(), line, 1));
            }
        }
    }

    // Generic error - put at line 1
    if !error.is_empty() {
        return Some(WsError::error(error.to_string(), 1, 1));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_with_line_and_column() {
        let error = "Error at line 10, column 5: Unexpected token";
        let ws_error = parse_error_message(error).unwrap();
        assert_eq!(ws_error.line, 10);
        assert_eq!(ws_error.column, 5);
        assert_eq!(ws_error.message, "Unexpected token");
    }

    #[test]
    fn test_parse_type_error() {
        let error = "Type error at line 5: Type mismatch";
        let ws_error = parse_error_message(error).unwrap();
        assert_eq!(ws_error.line, 5);
        assert_eq!(ws_error.column, 1);
        assert_eq!(ws_error.message, "Type mismatch");
    }

    #[test]
    fn test_to_diagnostic() {
        let ws_error = WsError::error("Test error".to_string(), 1, 1);
        let diag = ws_error.to_diagnostic();
        assert_eq!(diag.range.start.line, 0); // 0-indexed
        assert_eq!(diag.range.start.character, 0);
        assert_eq!(diag.severity, Some(DiagnosticSeverity::ERROR));
    }
}
