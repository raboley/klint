# KQL Linter Language Server (klint-lsp)

The KQL Linter Language Server provides real-time diagnostics for KQL table naming conventions in code editors that support the Language Server Protocol (LSP).

## Quick Start

### 1. Build the LSP Server

```bash
cargo build --bin klint-lsp --release
```

The binary will be available at `target/release/klint-lsp`.

### 2. Configuration

Create a `klint.yaml` configuration file in your workspace root:

```yaml
rules:
  table_naming: "PascalCase"
  excluded_tables:
    - "legacy_table"
    - "system_metrics"

output:
  format: "json"
  colors: true

exclude:
  - "temp_*"
```

### 3. Editor Integration

#### VS Code Example

Create a simple VS Code extension or use a generic LSP client:

```json
{
  "contributes": {
    "languages": [
      {
        "id": "kql",
        "aliases": ["KQL", "Kusto"],
        "extensions": [".kql", ".kusto"]
      }
    ]
  },
  "activationEvents": [
    "onLanguage:kql"
  ],
  "main": "./out/extension.js"
}
```

Extension code:
```javascript
const { LanguageClient } = require('vscode-languageclient/node');

function activate(context) {
    const serverOptions = {
        command: '/path/to/klint-lsp',
        args: []
    };

    const clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'kql' }]
    };

    const client = new LanguageClient(
        'klint-lsp',
        'KQL Linter Language Server',
        serverOptions,
        clientOptions
    );

    client.start();
}
```

#### Neovim Example

Using nvim-lspconfig:

```lua
local lspconfig = require('lspconfig')

-- Add klint-lsp configuration
local configs = require('lspconfig.configs')

if not configs.klint_lsp then
  configs.klint_lsp = {
    default_config = {
      cmd = { '/path/to/klint-lsp' },
      filetypes = { 'kql', 'kusto' },
      root_dir = lspconfig.util.root_pattern('klint.yaml', '.git'),
      settings = {}
    }
  }
end

lspconfig.klint_lsp.setup{}
```

## Features

### Real-time Diagnostics

The LSP server provides warnings for table naming convention violations:

```kql
.create table snake_case_table (id: int, name: string)
                ^^^^^^^^^^^^^
                Warning: Table 'snake_case_table' uses snake_case but PascalCase is expected. Consider renaming to 'SnakeCaseTable'
```

### Supported Language Features

- **Diagnostics**: Real-time warnings for naming violations
- **Document Synchronization**: Incremental updates on file changes
- **Configuration**: Workspace-level `klint.yaml` support

### Supported KQL Syntax

The LSP server uses tree-sitter for parsing and currently supports:

- `.create table` statements (primary focus)
- Basic table references in queries
- Management commands

## Troubleshooting

### Enable Debug Logging

Set the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug klint-lsp
```

### Common Issues

1. **No diagnostics appearing**: Ensure your file has a `.kql` or `.kusto` extension
2. **Configuration not loading**: Check that `klint.yaml` is in the workspace root
3. **Performance issues**: Large files may cause delays; consider excluding them

### Manual Testing

You can test the LSP server manually using stdio:

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"capabilities":{}}}' | klint-lsp
```

## Configuration Options

### Table Naming Conventions

Supported naming cases:
- `PascalCase` (default)
- `camelCase`
- `snake_case`
- `SCREAMING_SNAKE_CASE`
- `kebab-case`

### Exclusion Patterns

```yaml
rules:
  excluded_tables:
    - "legacy_*"      # Glob patterns supported
    - "system_table"  # Exact matches

exclude:
  - "temp_*.kql"      # File-level exclusions
```

## Performance

- **Startup time**: < 100ms typical
- **Diagnostic generation**: < 50ms for most files
- **Memory usage**: ~10MB base + file content

## Limitations

- Complex KQL query parsing is limited by the tree-sitter grammar
- Only table naming conventions are currently supported
- Code actions (quick fixes) are not yet implemented

## Contributing

See the main klint project for contribution guidelines. The LSP server is part of the klint library and follows the same development practices.