# KQL Linter Samples

This directory contains sample KQL files and configurations organized by the type of linting rules they demonstrate.

## Directory Structure

### `table_name/`
Contains all samples related to table naming convention rules:

- **`configured/`** - Sample configurations for different naming conventions and use cases
- **`violations/`** - KQL files that violate table naming conventions (for testing)
- **`valid/`** - KQL files that follow proper table naming conventions

## Future Structure

As the linter expands to support additional rule types, new directories will be added:

- `column_name/` - Column naming convention rules and samples
- `function_name/` - Function naming convention rules and samples  
- `query_style/` - Query formatting and style rules and samples
- `security/` - Security-related linting rules and samples

## Getting Started

1. **View violations**: Check `table_name/violations/` to see examples of common naming issues
2. **See valid examples**: Look at `table_name/valid/` for properly formatted tables
3. **Try configurations**: Navigate to `table_name/configured/` subdirectories and run `klint` to test different rules

## Usage Examples

```bash
# Test a violation file
klint samples/table_name/violations/mixed_cases.kql

# Test with a specific configuration
cd samples/table_name/configured/snake-case
klint microservice_tables.kql

# Test rule disabling
cd samples/table_name/configured/no-table-checks  
klint example_tables.kql  # Should pass despite mixed naming
```

## Contributing Samples

When adding new samples:

1. Place them in the appropriate rule-type directory
2. Include both positive (valid) and negative (violation) examples
3. Add configuration samples that demonstrate real-world use cases
4. Update relevant README files with usage examples