//! Main LSP server implementation
//!
//! Implements the Language Server Protocol using Tower LSP to provide
//! real-time diagnostics for KQL table naming conventions.

use crate::lsp::diagnostics::DiagnosticGenerator;
use crate::lsp::document::DocumentManager;
use crate::configuration::{Config, ConfigBuilder};
use anyhow::Result;
use lsp_types::{
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    InitializeParams, InitializeResult, InitializedParams,
    ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, Url,
};
use tower_lsp::lsp_types::MessageType;
use tower_lsp::{Client, LanguageServer};

/// KQL Linter Language Server
pub struct KlintLanguageServer {
    client: Client,
    document_manager: tokio::sync::RwLock<DocumentManager>,
    diagnostic_generator: tokio::sync::RwLock<DiagnosticGenerator>,
    config: tokio::sync::RwLock<Config>,
}

impl KlintLanguageServer {
    /// Create a new language server instance
    pub fn new(client: Client) -> Self {
        let config = ConfigBuilder::new().build(); // Default config
        let diagnostic_generator = DiagnosticGenerator::new(config.clone())
            .expect("Failed to create diagnostic generator");

        Self {
            client,
            document_manager: tokio::sync::RwLock::new(DocumentManager::new()),
            diagnostic_generator: tokio::sync::RwLock::new(diagnostic_generator),
            config: tokio::sync::RwLock::new(config),
        }
    }

    /// Send diagnostics for a document
    async fn send_diagnostics(&self, uri: &Url) -> Result<()> {
        let document_manager = self.document_manager.read().await;
        let mut diagnostic_generator = self.diagnostic_generator.write().await;

        if let Some(document) = document_manager.get(uri) {
            match diagnostic_generator.generate_diagnostics(document) {
                Ok(diagnostics) => {
                    self.client.publish_diagnostics(uri.clone(), diagnostics, None).await;
                }
                Err(e) => {
                    tracing::error!("Failed to generate diagnostics for {}: {}", uri, e);
                    self.client
                        .log_message(
                            MessageType::ERROR,
                            format!("Failed to analyze {}: {}", uri.path(), e),
                        )
                        .await;
                }
            }
        }

        Ok(())
    }

    /// Load configuration from workspace
    async fn load_config(&self, workspace_uri: Option<Url>) -> Config {
        // Convert URI to path if provided
        let workspace_path = workspace_uri
            .and_then(|uri| uri.to_file_path().ok());
        
        // Use shared configuration loading logic
        Config::load_for_workspace(workspace_path.as_ref())
    }

    /// Update configuration and regenerate diagnostics for all documents
    async fn update_config(&self, config: Config) -> Result<()> {
        {
            let mut config_lock = self.config.write().await;
            *config_lock = config.clone();
        }

        {
            let mut diagnostic_generator = self.diagnostic_generator.write().await;
            diagnostic_generator.update_config(config);
        }

        // Regenerate diagnostics for all open documents
        let document_manager = self.document_manager.read().await;
        for uri in document_manager.uris() {
            if let Err(e) = self.send_diagnostics(uri).await {
                tracing::error!("Failed to regenerate diagnostics for {}: {}", uri, e);
            }
        }

        Ok(())
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for KlintLanguageServer {
    async fn initialize(&self, params: InitializeParams) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        tracing::info!("Initializing KQL Linter Language Server");

        // Load configuration from workspace
        let config = self.load_config(params.root_uri.clone()).await;
        if let Err(e) = self.update_config(config).await {
            tracing::error!("Failed to load initial configuration: {}", e);
        }

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                diagnostic_provider: None, // We'll use publishDiagnostics instead
                ..Default::default()
            },
            server_info: Some(lsp_types::ServerInfo {
                name: "klint-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        tracing::info!("KQL Linter Language Server initialized");
        self.client
            .log_message(MessageType::INFO, "KQL Linter Language Server ready")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let doc = params.text_document;
        let uri = doc.uri.clone();

        tracing::debug!("Document opened: {}", uri);

        {
            let mut document_manager = self.document_manager.write().await;
            document_manager.open(doc.uri, doc.language_id, doc.version, doc.text);
        }

        if let Err(e) = self.send_diagnostics(&uri).await {
            tracing::error!("Failed to send diagnostics for opened document {}: {}", uri, e);
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let version = params.text_document.version;

        tracing::debug!("Document changed: {} (version {})", uri, version);

        {
            let mut document_manager = self.document_manager.write().await;
            document_manager.update(&uri, version, params.content_changes);
        }

        if let Err(e) = self.send_diagnostics(&uri).await {
            tracing::error!("Failed to send diagnostics for changed document {}: {}", uri, e);
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;

        tracing::debug!("Document closed: {}", uri);

        {
            let mut document_manager = self.document_manager.write().await;
            document_manager.close(&uri);
        }

        // Clear diagnostics for closed document
        self.client.publish_diagnostics(uri, vec![], None).await;
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        tracing::info!("KQL Linter Language Server shutting down");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NamingCase;

    #[test]
    fn test_config_loading() {
        // Test that we can create a server configuration
        let config = ConfigBuilder::new().build();
        assert_eq!(config.naming_case(), NamingCase::PascalCase); // Default case
    }

    #[test]
    fn test_document_manager() {
        // Test document manager functionality
        let mut manager = DocumentManager::new();
        let uri = Url::parse("file:///test.kql").unwrap();
        
        manager.open(
            uri.clone(),
            "kql".to_string(),
            1,
            ".create table TestTable (id: int)".to_string(),
        );

        let doc = manager.get(&uri).unwrap();
        assert_eq!(doc.version, 1);
        assert!(doc.text.contains("TestTable"));

        manager.close(&uri);
        assert!(manager.get(&uri).is_none());
    }
}