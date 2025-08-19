//! KQL Linter Language Server
//!
//! A Language Server Protocol implementation for the KQL linter that provides
//! real-time table naming convention diagnostics in code editors.

use klint::lsp::server::KlintLanguageServer;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| KlintLanguageServer::new(client))
        .finish();

    tracing::info!("KQL Linter Language Server starting...");
    Server::new(stdin, stdout, socket).serve(service).await;
}