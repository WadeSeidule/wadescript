/// Analysis coordinator for the LSP
/// Wraps the lexer, parser, and type checker to provide LSP functionality
use std::collections::HashMap;

use tower_lsp::lsp_types::*;

use crate::ast::{Statement, Type};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::typechecker::TypeChecker;

use super::diagnostics::{parse_error_message, WsError};
use super::span::lsp_position_to_ws;

/// Symbol information for LSP features
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub kind: SymbolKind,
    pub symbol_type: Option<String>,
    pub line: usize,
    pub column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

/// The main analyzer that provides all LSP functionality
pub struct Analyzer {
    // Cache of analyzed files (uri -> symbols)
    _symbols_cache: HashMap<String, Vec<SymbolInfo>>,
}

impl Analyzer {
    pub fn new() -> Self {
        Analyzer {
            _symbols_cache: HashMap::new(),
        }
    }

    /// Analyze source code and return diagnostics
    pub fn analyze(&self, source: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // Try to lex
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize();

        // Check for lexer errors (panics in current impl, so we catch them)
        // For now, assume lexer succeeds if we get here

        // Try to parse
        let mut parser = Parser::new_from_tokens(tokens);
        let parse_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            parser.parse()
        }));

        let program = match parse_result {
            Ok(prog) => prog,
            Err(e) => {
                // Extract panic message
                let msg = if let Some(s) = e.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "Parse error".to_string()
                };

                if let Some(ws_error) = parse_error_message(&msg) {
                    diagnostics.push(ws_error.to_diagnostic());
                } else {
                    diagnostics.push(WsError::error(msg, 1, 1).to_diagnostic());
                }
                return diagnostics;
            }
        };

        // Try to type check
        let mut type_checker = TypeChecker::new();
        if let Err(type_error) = type_checker.check_program(&program) {
            if let Some(ws_error) = parse_error_message(&type_error) {
                diagnostics.push(ws_error.to_diagnostic());
            } else {
                diagnostics.push(WsError::error(type_error, 1, 1).to_diagnostic());
            }
        }

        diagnostics
    }

    /// Get hover information at a position
    pub fn hover(&self, source: &str, position: Position) -> Option<String> {
        let (line, col) = lsp_position_to_ws(&position);
        let symbols = self.collect_symbols(source);

        // Find symbol at position
        for sym in &symbols {
            if sym.line == line && col >= sym.column && col <= sym.end_column {
                let type_info = sym.symbol_type.as_deref().unwrap_or("unknown");
                return Some(format!(
                    "**{}** ({})\n\nType: `{}`",
                    sym.name,
                    format!("{:?}", sym.kind).to_lowercase(),
                    type_info
                ));
            }
        }

        None
    }

    /// Get completion items at a position
    pub fn complete(&self, source: &str, position: Position) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        let symbols = self.collect_symbols(source);

        // Add symbols as completion items
        for sym in symbols {
            let kind = match sym.kind {
                SymbolKind::FUNCTION | SymbolKind::METHOD => CompletionItemKind::FUNCTION,
                SymbolKind::VARIABLE => CompletionItemKind::VARIABLE,
                SymbolKind::CLASS => CompletionItemKind::CLASS,
                SymbolKind::FIELD => CompletionItemKind::FIELD,
                _ => CompletionItemKind::TEXT,
            };

            items.push(CompletionItem {
                label: sym.name.clone(),
                kind: Some(kind),
                detail: sym.symbol_type,
                ..Default::default()
            });
        }

        // Add keywords
        let keywords = [
            "def", "class", "if", "elif", "else", "while", "for", "in",
            "return", "break", "continue", "pass", "try", "except", "finally",
            "raise", "import", "assert", "True", "False", "None", "and", "or", "not",
        ];

        for kw in keywords {
            items.push(CompletionItem {
                label: kw.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            });
        }

        // Add types
        let types = ["int", "float", "str", "bool", "list", "dict", "void"];
        for ty in types {
            items.push(CompletionItem {
                label: ty.to_string(),
                kind: Some(CompletionItemKind::TYPE_PARAMETER),
                ..Default::default()
            });
        }

        // Filter based on what's being typed
        let (ws_line, ws_col) = lsp_position_to_ws(&position);
        if let Some(prefix) = self.get_word_prefix(source, ws_line, ws_col) {
            items.retain(|item| {
                item.label.to_lowercase().starts_with(&prefix.to_lowercase())
            });
        }

        items
    }

    /// Go to definition
    pub fn goto_definition(
        &self,
        source: &str,
        position: Position,
        uri: &Url,
    ) -> Option<Location> {
        let (line, col) = lsp_position_to_ws(&position);
        let word = self.get_word_at(source, line, col)?;
        let symbols = self.collect_symbols(source);

        // Find the definition of the symbol
        for sym in symbols {
            if sym.name == word {
                // Found definition
                let range = Range {
                    start: Position {
                        line: (sym.line.saturating_sub(1)) as u32,
                        character: (sym.column.saturating_sub(1)) as u32,
                    },
                    end: Position {
                        line: (sym.end_line.saturating_sub(1)) as u32,
                        character: (sym.end_column.saturating_sub(1)) as u32,
                    },
                };
                return Some(Location {
                    uri: uri.clone(),
                    range,
                });
            }
        }

        None
    }

    /// Find all references to a symbol
    pub fn find_references(
        &self,
        source: &str,
        position: Position,
        uri: &Url,
    ) -> Vec<Location> {
        let (line, col) = lsp_position_to_ws(&position);
        let word = match self.get_word_at(source, line, col) {
            Some(w) => w,
            None => return Vec::new(),
        };

        let mut refs = Vec::new();

        // Simple approach: find all occurrences of the word in the source
        for (line_num, line_text) in source.lines().enumerate() {
            let mut search_start = 0;
            while let Some(col_idx) = line_text[search_start..].find(&word) {
                let actual_col = search_start + col_idx;
                // Check if it's a word boundary
                let before_ok = actual_col == 0
                    || !line_text.chars().nth(actual_col - 1).map_or(false, is_identifier_char);
                let after_ok = actual_col + word.len() >= line_text.len()
                    || !line_text.chars().nth(actual_col + word.len()).map_or(false, is_identifier_char);

                if before_ok && after_ok {
                    refs.push(Location {
                        uri: uri.clone(),
                        range: Range {
                            start: Position {
                                line: line_num as u32,
                                character: actual_col as u32,
                            },
                            end: Position {
                                line: line_num as u32,
                                character: (actual_col + word.len()) as u32,
                            },
                        },
                    });
                }
                search_start = actual_col + word.len();
            }
        }

        refs
    }

    /// Get document symbols (outline)
    pub fn document_symbols(&self, source: &str) -> Vec<DocumentSymbol> {
        let symbols = self.collect_symbols(source);
        let mut doc_symbols = Vec::new();

        for sym in symbols {
            let range = Range {
                start: Position {
                    line: (sym.line.saturating_sub(1)) as u32,
                    character: (sym.column.saturating_sub(1)) as u32,
                },
                end: Position {
                    line: (sym.end_line.saturating_sub(1)) as u32,
                    character: (sym.end_column.saturating_sub(1)) as u32,
                },
            };

            #[allow(deprecated)]
            doc_symbols.push(DocumentSymbol {
                name: sym.name,
                detail: sym.symbol_type,
                kind: sym.kind,
                range,
                selection_range: range,
                children: None,
                tags: None,
                deprecated: None,
            });
        }

        doc_symbols
    }

    /// Rename a symbol
    pub fn rename(
        &self,
        source: &str,
        position: Position,
        new_name: &str,
        uri: &Url,
    ) -> Option<WorkspaceEdit> {
        let refs = self.find_references(source, position, uri);
        if refs.is_empty() {
            return None;
        }

        let edits: Vec<TextEdit> = refs
            .into_iter()
            .map(|loc| TextEdit {
                range: loc.range,
                new_text: new_name.to_string(),
            })
            .collect();

        let mut changes = HashMap::new();
        changes.insert(uri.clone(), edits);

        Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        })
    }

    /// Format document
    pub fn format(&self, source: &str) -> Option<Vec<TextEdit>> {
        // Simple formatting: normalize indentation
        let mut formatted = String::new();
        let mut indent_level = 0;

        for line in source.lines() {
            let trimmed = line.trim();

            // Decrease indent for closing braces
            if trimmed.starts_with('}') && indent_level > 0 {
                indent_level -= 1;
            }

            // Add proper indentation
            if !trimmed.is_empty() {
                formatted.push_str(&"    ".repeat(indent_level));
                formatted.push_str(trimmed);
            }
            formatted.push('\n');

            // Increase indent after opening braces
            if trimmed.ends_with('{') {
                indent_level += 1;
            }
        }

        // Remove trailing newline if original didn't have one
        if !source.ends_with('\n') && formatted.ends_with('\n') {
            formatted.pop();
        }

        if formatted == source {
            return None;
        }

        let line_count = source.lines().count();
        Some(vec![TextEdit {
            range: Range {
                start: Position { line: 0, character: 0 },
                end: Position {
                    line: line_count as u32,
                    character: 0,
                },
            },
            new_text: formatted,
        }])
    }

    /// Collect all symbols from the source
    fn collect_symbols(&self, source: &str) -> Vec<SymbolInfo> {
        let mut symbols = Vec::new();

        // Try to parse the source
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.tokenize();
        let mut parser = Parser::new_from_tokens(tokens);

        let program = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            parser.parse()
        })) {
            Ok(prog) => prog,
            Err(_) => return symbols, // Return empty on parse error
        };

        // Collect symbols from AST
        for stmt in &program.statements {
            self.collect_symbols_from_statement(stmt, &mut symbols);
        }

        symbols
    }

    fn collect_symbols_from_statement(&self, stmt: &Statement, symbols: &mut Vec<SymbolInfo>) {
        match stmt {
            Statement::FunctionDef {
                name,
                params,
                return_type,
                body: _,
            } => {
                let param_types: Vec<String> = params
                    .iter()
                    .map(|p| format_type(&p.param_type))
                    .collect();
                let sig = format!(
                    "({}) -> {}",
                    param_types.join(", "),
                    format_type(return_type)
                );

                symbols.push(SymbolInfo {
                    name: name.clone(),
                    kind: SymbolKind::FUNCTION,
                    symbol_type: Some(sig),
                    line: 1, // TODO: track actual line in AST
                    column: 1,
                    end_line: 1,
                    end_column: name.len(),
                });

                // Add parameters
                for param in params {
                    symbols.push(SymbolInfo {
                        name: param.name.clone(),
                        kind: SymbolKind::VARIABLE,
                        symbol_type: Some(format_type(&param.param_type)),
                        line: 1,
                        column: 1,
                        end_line: 1,
                        end_column: param.name.len(),
                    });
                }
            }
            Statement::ClassDef {
                name,
                _base_class: _,
                fields,
                methods,
            } => {
                symbols.push(SymbolInfo {
                    name: name.clone(),
                    kind: SymbolKind::CLASS,
                    symbol_type: Some("class".to_string()),
                    line: 1,
                    column: 1,
                    end_line: 1,
                    end_column: name.len(),
                });

                // Add fields
                for field in fields {
                    symbols.push(SymbolInfo {
                        name: field.name.clone(),
                        kind: SymbolKind::FIELD,
                        symbol_type: Some(format_type(&field.field_type)),
                        line: 1,
                        column: 1,
                        end_line: 1,
                        end_column: field.name.len(),
                    });
                }

                // Add methods
                for method in methods {
                    self.collect_symbols_from_statement(method, symbols);
                }
            }
            Statement::VarDecl { name, type_annotation, initializer: _ } => {
                symbols.push(SymbolInfo {
                    name: name.clone(),
                    kind: SymbolKind::VARIABLE,
                    symbol_type: Some(format_type(type_annotation)),
                    line: 1,
                    column: 1,
                    end_line: 1,
                    end_column: name.len(),
                });
            }
            _ => {}
        }
    }

    /// Get the word at a position (1-indexed)
    fn get_word_at(&self, source: &str, line: usize, col: usize) -> Option<String> {
        let line_text = source.lines().nth(line.saturating_sub(1))?;
        let chars: Vec<char> = line_text.chars().collect();
        let col_idx = col.saturating_sub(1);

        if col_idx >= chars.len() {
            return None;
        }

        // Find word boundaries
        let mut start = col_idx;
        let mut end = col_idx;

        while start > 0 && is_identifier_char(chars[start - 1]) {
            start -= 1;
        }

        while end < chars.len() && is_identifier_char(chars[end]) {
            end += 1;
        }

        if start == end {
            return None;
        }

        Some(chars[start..end].iter().collect())
    }

    /// Get the prefix of the word being typed (for completion filtering)
    fn get_word_prefix(&self, source: &str, line: usize, col: usize) -> Option<String> {
        let line_text = source.lines().nth(line.saturating_sub(1))?;
        let chars: Vec<char> = line_text.chars().collect();
        let col_idx = col.saturating_sub(1);

        if col_idx == 0 {
            return None;
        }

        let mut start = col_idx;
        while start > 0 && is_identifier_char(chars[start - 1]) {
            start -= 1;
        }

        if start == col_idx {
            return None;
        }

        Some(chars[start..col_idx].iter().collect())
    }
}

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn format_type(ty: &Type) -> String {
    match ty {
        Type::Int => "int".to_string(),
        Type::Float => "float".to_string(),
        Type::Bool => "bool".to_string(),
        Type::Str => "str".to_string(),
        Type::Void => "void".to_string(),
        Type::List(inner) => format!("list[{}]", format_type(inner)),
        Type::Dict(k, v) => format!("dict[{}, {}]", format_type(k), format_type(v)),
        Type::Array(inner, size) => format!("array[{}, {}]", format_type(inner), size),
        Type::Custom(name) => name.clone(),
        Type::Optional(inner) => format!("{}?", format_type(inner)),
        Type::Exception => "Exception".to_string(),
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}
