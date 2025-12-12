/// Main LSP server implementation using tower-lsp
use dashmap::DashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use super::analysis::Analyzer;
use super::document::Document;

/// The WadeScript Language Server
pub struct WadeScriptServer {
    client: Client,
    documents: DashMap<Url, Document>,
    analyzer: Analyzer,
}

impl WadeScriptServer {
    pub fn new(client: Client) -> Self {
        WadeScriptServer {
            client,
            documents: DashMap::new(),
            analyzer: Analyzer::new(),
        }
    }

    async fn publish_diagnostics(&self, uri: &Url) {
        if let Some(doc) = self.documents.get(uri) {
            let diagnostics = self.analyzer.analyze(&doc.content);
            self.client
                .publish_diagnostics(uri.clone(), diagnostics, Some(doc.version))
                .await;
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for WadeScriptServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(true),
                        })),
                        ..Default::default()
                    },
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![".".to_string()]),
                    resolve_provider: Some(false),
                    ..Default::default()
                }),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "wadescript-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "WadeScript language server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let doc = Document::new(
            params.text_document.text,
            params.text_document.version,
        );
        self.documents.insert(uri.clone(), doc);
        self.publish_diagnostics(&uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        if let Some(change) = params.content_changes.into_iter().last() {
            if let Some(mut doc) = self.documents.get_mut(&uri) {
                doc.update(change.text, params.text_document.version);
            }
        }
        self.publish_diagnostics(&uri).await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        if let Some(text) = params.text {
            let uri = params.text_document.uri.clone();
            if let Some(mut doc) = self.documents.get_mut(&uri) {
                doc.content = text;
            }
            self.publish_diagnostics(&uri).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.remove(&params.text_document.uri);
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(doc) = self.documents.get(uri) {
            if let Some(hover_info) = self.analyzer.hover(&doc.content, position) {
                return Ok(Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: hover_info,
                    }),
                    range: None,
                }));
            }
        }
        Ok(None)
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = &params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        if let Some(doc) = self.documents.get(uri) {
            let items = self.analyzer.complete(&doc.content, position);
            if !items.is_empty() {
                return Ok(Some(CompletionResponse::Array(items)));
            }
        }
        Ok(None)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(doc) = self.documents.get(uri) {
            if let Some(location) = self.analyzer.goto_definition(&doc.content, position, uri) {
                return Ok(Some(GotoDefinitionResponse::Scalar(location)));
            }
        }
        Ok(None)
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let uri = &params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        if let Some(doc) = self.documents.get(uri) {
            let refs = self.analyzer.find_references(&doc.content, position, uri);
            if !refs.is_empty() {
                return Ok(Some(refs));
            }
        }
        Ok(None)
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = &params.text_document.uri;

        if let Some(doc) = self.documents.get(uri) {
            let symbols = self.analyzer.document_symbols(&doc.content);
            if !symbols.is_empty() {
                return Ok(Some(DocumentSymbolResponse::Nested(symbols)));
            }
        }
        Ok(None)
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let uri = &params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let new_name = &params.new_name;

        if let Some(doc) = self.documents.get(uri) {
            if let Some(edit) = self.analyzer.rename(&doc.content, position, new_name, uri) {
                return Ok(Some(edit));
            }
        }
        Ok(None)
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = &params.text_document.uri;

        if let Some(doc) = self.documents.get(uri) {
            if let Some(edits) = self.analyzer.format(&doc.content) {
                return Ok(Some(edits));
            }
        }
        Ok(None)
    }
}

/// Run the LSP server on stdin/stdout
pub async fn run_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(WadeScriptServer::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
