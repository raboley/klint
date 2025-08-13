# Configuration Samples

This directory contains sample configurations demonstrating different ways to configure the KQL linter for various project needs.

## How Configuration Discovery Works

The KQL linter automatically discovers configuration files by searching for:
1. `.klint-config.yml` (preferred)
2. `.klint.yml` (legacy)
3. `.klint.yaml` (legacy)

It starts from the current directory and walks up the directory tree until it finds a configuration file.

## Sample Configurations

### 1. No Table Checks (`no-table-checks/`)

**Use Case**: Legacy projects or when you want to disable table naming enforcement entirely.

**Configuration**: Uses `settings.disabled: ["table-naming"]` to completely disable the table-naming rule.

**Example**:
```bash
cd samples/table_name/configured/no-table-checks
klint example_tables.kql  # No violations reported despite mixed naming
```

### 2. SCREAMING_SNAKE_CASE (`screaming-snake-case/`)

**Use Case**: Data warehouses, analytics platforms, and traditional database environments.

**Configuration**: Enforces `SCREAMING_SNAKE_CASE` with specific exclusions for legacy tables.

**Example**:
```bash
cd samples/table_name/configured/screaming-snake-case
klint analytics_tables.kql  # Only SCREAMING_SNAKE_CASE tables pass
```

### 3. snake_case (`snake-case/`)

**Use Case**: Modern web applications, microservices, and developer-friendly environments.

**Configuration**: Enforces `snake_case` with exclusions for system and API tables.

**Example**:
```bash
cd samples/table_name/configured/snake-case
klint microservice_tables.kql  # Only snake_case tables pass
```

### 4. Rule Disabling (`partial-disabled/`)

**Use Case**: When you want to selectively disable specific linting rules while keeping the rest enabled.

**Configuration**: Uses `settings.disabled: ["table-naming"]` to demonstrate fine-grained rule control.

**Example**:
```bash
cd samples/table_name/configured/partial-disabled
klint mixed_tables.kql  # No violations despite mixed naming conventions
```

## Testing the Configurations

You can test each configuration by navigating to its directory and running the linter:

```bash
# Test no table checks
cd no-table-checks
klint example_tables.kql
# Expected: No violations (all checks disabled)

# Test SCREAMING_SNAKE_CASE
cd ../screaming-snake-case  
klint analytics_tables.kql
# Expected: No violations (follows SCREAMING_SNAKE_CASE)

# Test snake_case
cd ../snake-case
klint microservice_tables.kql  
# Expected: No violations (follows snake_case)

# Test rule disabling
cd ../partial-disabled
klint mixed_tables.kql
# Expected: No violations (table-naming rule disabled)
```

## Configuration Features Demonstrated

### Rule Disabling
- **Global rule disabling**: `settings.disabled: ["table-naming"]`
- **Multiple rules**: `["table-naming", "future-rule"]`
- **Clean and intuitive**: No need for wildcard patterns or complex exclusions

### Table Exclusions
- **Wildcard patterns**: `"*"`, `"temp_*"`, `"APP_*"`
- **Specific tables**: `"LegacyUserData"`, `"SystemEvents"`
- **Pattern matching**: Supports glob-style patterns

### File Exclusions
- **Directory patterns**: `"docs/**"`, `"**/temp/**"`
- **File patterns**: `"test_*.kql"`, `"*.example.kql"`
- **Temporary files**: `"**/.tmp/**"`

### Output Configuration
- **Format options**: `"terminal"` (colored), `"json"` (machine-readable)
- **Color control**: `colors: true/false`
- **Report generation**: `report_path: "filename.md"` (optional)

## Creating Your Own Configuration

1. Copy one of these samples as a starting point
2. Modify the `table_naming` convention to your preference
3. Add exclusions for your specific tables/files
4. Test with your KQL files
5. Commit the `.klint-config.yml` to your repository

## Best Practices

- **Use `.klint-config.yml`** for new projects (clearer naming)
- **Include exclusions** for legacy or special-purpose tables
- **Document your choices** in comments within the config file
- **Test thoroughly** before committing to ensure no false positives
- **Use nolint comments** for rare exceptions that can't be configured