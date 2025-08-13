# KQL Linter Quickstart Guide

Get up and running with the KQL linter in under 5 minutes!

## 🚀 Installation

### Option 1: From Source (Recommended)
```bash
# Clone and install
git clone https://github.com/your-org/kql-analyzer.git
cd kql-analyzer
cargo install --path .
```

### Option 2: Via Cargo (when published)
```bash
cargo install klint
```

## ⚡ Quick Setup

### 1. Initialize Configuration
```bash
# Navigate to your KQL project directory
cd your-kql-project

# Create default configuration
klint init
```

This creates a `.klint.yml` file with sensible defaults (PascalCase table naming).

### 2. Check Your Files
```bash
# Check a single file
klint my-tables.kql

# Check all KQL files in current directory
klint *.kql

# Check all files recursively
klint --recursive
```

### 3. Fix Issues Automatically
```bash
# Preview what would be fixed
klint --preview my-tables.kql

# Apply fixes automatically
klint --fix my-tables.kql
```

## 📝 Common Configuration

Edit `.klint.yml` to customize:

```yaml
rules:
  # Choose your naming convention
  table_naming: "PascalCase"  # or snake_case, camelCase, etc.
  
  # Exclude specific tables
  excluded_tables:
    - "legacy_*"
    - "temp_data"

output:
  format: "terminal"  # or "json"
  colors: true

# Exclude file patterns
exclude:
  - "test_*.kql"
  - "*.tmp"
```

## 🔧 Disable Specific Lines

Use `//nolint:` comments to disable linting:

```kql
// Disable all rules for this line
.create table legacy_data (id: int) //nolint:

// Disable specific rule
.create table user_data (id: int) //nolint:table-naming

// Works with SQL comments too
.create table old_table (id: int) -- nolint:table-naming
```

## 📊 Output Formats

### Terminal (Default)
```bash
klint my-file.kql
```
Shows colored output with violations and summaries.

### JSON
```bash
klint --output json my-file.kql
```
Machine-readable format for CI/CD integration.

## 🔄 Common Workflows

### Development Workflow
```bash
# Check files during development
klint src/

# Fix issues before commit
klint --fix src/
```

### CI/CD Integration
```bash
# In your CI pipeline
klint --output json --recursive . > lint-results.json

# Exit with error code if violations found
klint . && echo "All good!" || echo "Linting failed"
```

### Pre-commit Hook
```bash
# .git/hooks/pre-commit
#!/bin/bash
klint --quiet . || {
    echo "KQL linting failed. Run 'klint --fix .' to fix issues."
    exit 1
}
```

## 📋 Naming Conventions

| Convention | Example | Description |
|------------|---------|-------------|
| `PascalCase` | `UserData` | Title case (default) |
| `snake_case` | `user_data` | Lowercase with underscores |
| `camelCase` | `userData` | Camel case |
| `SCREAMING_SNAKE_CASE` | `USER_DATA` | Uppercase with underscores |
| `kebab-case` | `user-data` | Lowercase with hyphens |

## 🆘 Troubleshooting

### No violations found but files have issues?
- Check your `.klint.yml` configuration
- Ensure files have `.create table` statements
- Verify file paths are correct

### False positives?
- Use `//nolint:table-naming` to disable specific lines
- Add tables to `excluded_tables` in config
- Check if naming convention matches your intent

### CI/CD integration not working?
- Use `--output json` for machine parsing
- Check exit codes (0 = success, 1 = violations found)
- Use `--quiet` to suppress debug output

## 🎯 Next Steps

1. **Customize Configuration**: Adjust `.klint.yml` for your team's standards
2. **Integrate with CI/CD**: Add linting to your pipeline
3. **Team Adoption**: Share configuration and guidelines with your team
4. **Advanced Usage**: Explore JSON output for custom reporting

## 💡 Pro Tips

- **Preview Mode**: Always use `--preview` before `--fix` on important files
- **Recursive Checking**: Use `--recursive` to check entire project trees
- **JSON Output**: Perfect for integration with other tools and IDEs
- **Selective Fixing**: Use nolint comments for legacy code that can't be changed
- **Configuration Validation**: Run `klint init --force` to reset to defaults if config is corrupted

## 🔗 Further Reading

- [Full README](README.md) - Complete documentation
- [Configuration Guide](README.md#configuration-options) - All available options
- [GitHub Issues](https://github.com/your-org/kql-analyzer/issues) - Bug reports and feature requests

---

**Need help?** Open an issue on GitHub or check the full documentation in README.md.