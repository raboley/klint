# KQL Linter (klint)

A fast and flexible linter for Kusto Query Language (KQL) table naming conventions in Azure Data Explorer.

## Features

- **Table Naming Convention Enforcement**: Supports snake_case, PascalCase, camelCase, SCREAMING_SNAKE_CASE, and kebab-case
- **Automatic Fix Mode**: Automatically convert table names to the desired convention
- **Preview Mode**: See what changes would be made without modifying files
- **Inline Disable Comments**: Disable linting for specific lines with `//nolint:` comments
- **Flexible Configuration**: YAML-based configuration with validation
- **Multiple Output Formats**: Terminal (colored) and JSON output
- **Fast Performance**: Written in Rust for speed and reliability

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/your-org/kql-analyzer.git
cd kql-analyzer

# Build and install
cargo install --path .
```

### Using Cargo

```bash
cargo install klint
```

## Quick Start

1. **Initialize configuration** in your project:
   ```bash
   klint init
   ```

2. **Check your KQL files** for naming violations:
   ```bash
   klint *.kql
   ```

3. **Automatically fix** naming violations:
   ```bash
   klint --fix *.kql
   ```

## Usage

### Basic Commands

```bash
# Check files for violations
klint samples/table_name/violations/mixed_cases.kql

# Fix violations automatically
klint --fix samples/table_name/violations/mixed_cases.kql

# Preview what fixes would be applied
klint --preview samples/table_name/violations/mixed_cases.kql

# Output results in JSON format
klint --output json samples/table_name/violations/mixed_cases.kql

# Initialize default configuration
klint init

# Check all KQL files in current directory
klint

# Check all KQL files recursively
klint --recursive
```

### Configuration

KQL Linter uses a `.klint.yml` configuration file. Generate a default configuration with:

```bash
klint init
```

Example configuration:

```yaml
rules:
  table_naming: "PascalCase"
  excluded_tables:
    - "legacy_*"
    - "temp_data"

output:
  format: "terminal"
  colors: true
  report_path: null

exclude:
  - "*.tmp"
  - "test_*.kql"
```

### Supported Naming Conventions

- `snake_case` - Lowercase with underscores (e.g., `user_data`)
- `PascalCase` - Title case (e.g., `UserData`)
- `camelCase` - Camel case (e.g., `userData`)
- `SCREAMING_SNAKE_CASE` - Uppercase with underscores (e.g., `USER_DATA`)
- `kebab-case` - Lowercase with hyphens (e.g., `user-data`)

### Inline Disable Comments

Disable linting for specific lines or rules:

```kql
// Disable all rules for current line
.create table legacy_data (id: int) //nolint:

// Disable specific rule for current line  
.create table user_data (id: int) //nolint:table-naming

// Disable multiple rules for current line
.create table user_data (id: int) //nolint:table-naming,other-rule

// Works with SQL-style comments too
.create table user_data (id: int) -- nolint:table-naming
```

## Configuration Options

### Rules

- `table_naming`: The naming convention to enforce
- `excluded_tables`: List of table name patterns to exclude from linting

### Output

- `format`: Output format (`terminal` or `json`)
- `colors`: Enable colored terminal output (boolean)
- `report_path`: Path to save detailed reports (optional)

### Global Exclusions

- `exclude`: List of file patterns to exclude from linting

## Examples

### Check a Single File

```bash
klint samples/table_name/violations/mixed_cases.kql
```

Output:
```
samples/table_name/violations/mixed_cases.kql
  4:1 ✗ Table 'user_logs' uses snake_case but should use PascalCase
  12:1 ✗ Table 'error-logs' uses kebab-case but should use PascalCase
  19:1 ✗ Table 'SYSTEM_METRICS' uses SCREAMING_SNAKE_CASE but should use PascalCase
  34:1 ✗ Table 'applicationLogs' uses camelCase but should use PascalCase

✗ Found 4 violations in 1 file of 1 file checked
```

### Fix Files Automatically

```bash
klint --fix samples/table_name/violations/mixed_cases.kql
```

Output:
```
samples/table_name/violations/mixed_cases.kql
  4:1 ✓ Fixed: 'user_logs' → 'UserLogs'
  12:1 ✓ Fixed: 'error-logs' → 'ErrorLogs'
  19:1 ✓ Fixed: 'SYSTEM_METRICS' → 'SystemMetrics'
  34:1 ✓ Fixed: 'applicationLogs' → 'ApplicationLogs'

✓ Fixed 4 violations in 1 file
```

### Preview Mode

```bash
klint --preview samples/table_name/violations/mixed_cases.kql
```

Output:
```
samples/table_name/violations/mixed_cases.kql
  4:1 ✗ Table 'user_logs' uses snake_case but should use PascalCase
  12:1 ✗ Table 'error-logs' uses kebab-case but should use PascalCase
  19:1 ✗ Table 'SYSTEM_METRICS' uses SCREAMING_SNAKE_CASE but should use PascalCase
  34:1 ✗ Table 'applicationLogs' uses camelCase but should use PascalCase

✗ Found 4 violations in 1 file of 1 file checked

Proposed fixes:
  Line 4: user_logs → UserLogs
  Line 12: error-logs → ErrorLogs
  Line 19: SYSTEM_METRICS → SystemMetrics
  Line 34: applicationLogs → ApplicationLogs
```

### JSON Output

```bash
klint --output json samples/table_name/violations/mixed_cases.kql
```

Output:
```json
{
  "violations": [
    {
      "file": "samples/table_name/violations/mixed_cases.kql",
      "line": 4,
      "column": 1,
      "table_name": "user_logs",
      "current_case": "snake_case",
      "expected_case": "PascalCase",
      "message": "Table 'user_logs' uses snake_case but should use PascalCase"
    },
    {
      "file": "samples/table_name/violations/mixed_cases.kql",
      "line": 12,
      "column": 1,
      "table_name": "error-logs",
      "current_case": "kebab-case",
      "expected_case": "PascalCase",
      "message": "Table 'error-logs' uses kebab-case but should use PascalCase"
    }
  ],
  "summary": {
    "total_files": 1,
    "files_with_violations": 1,
    "total_violations": 4
  }
}
```

## Exit Codes

- `0`: No violations found
- `1`: Violations found or errors occurred
- `2`: Configuration or usage errors

## KQL Syntax Support

Currently supports `.create table` commands:

```kql
.create table UserData (
    id: int,
    name: string,
    email: string
)

.create table user_events (
    timestamp: datetime,
    user_id: int,
    event_type: string
) 
```

## Development

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check for security vulnerabilities
cargo audit
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please ensure all tests pass and follow the existing code style.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- **Issues**: Report bugs and request features on [GitHub Issues](https://github.com/your-org/kql-analyzer/issues)
- **Documentation**: Additional documentation available in the [docs/](docs/) directory
- **Examples**: Sample KQL files and configurations in [samples/](samples/) directory