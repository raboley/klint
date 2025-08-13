# Task List: KQL Linter CLI (klint)

## Relevant Files

- `Cargo.toml` - Root cargo manifest with workspace configuration
- `src/main.rs` - CLI entry point with clap argument parsing
- `src/lib.rs` - Library entry point for future LSP integration
- `src/parser/mod.rs` - KQL parser module for extracting table names
- `src/parser/lexer.rs` - Tokenizer for KQL syntax
- `src/parser/ast.rs` - Abstract syntax tree definitions for KQL statements
- `tests/parser_test.rs` - Integration tests for KQL parser
- `src/case_detector.rs` - Module for detecting naming conventions and case conversion
- `tests/case_detection_test.rs` - Integration tests for case detection
- `src/config.rs` - Configuration management with YAML schema
- `tests/config_test.rs` - Integration tests for configuration loading
- `src/linter/mod.rs` - Core linting engine
- `src/linter/rules.rs` - Linting rule definitions
- `src/linter/fixer.rs` - Auto-fix implementation
- `tests/linter_test.rs` - Integration tests for linting functionality
- `src/output/mod.rs` - Output formatting module
- `src/output/terminal.rs` - Colored terminal output formatter
- `src/output/json.rs` - JSON output formatter
- `src/output/report.rs` - File report generation
- `tests/output_test.rs` - Integration tests for output formatting
- `src/file_processor.rs` - File and directory processing with recursive walking
- `tests/file_processor_test.rs` - Integration tests for file processing
- `samples/violations/` - Sample KQL files with various violations
- `samples/valid/` - Sample KQL files with correct naming
- `tests/integration/` - Black-box integration tests
- `.klint.yml` - Default configuration file template
- `README.md` - Project documentation with usage examples
- `docs/configuration.md` - Configuration options documentation
- `docs/naming-conventions.md` - Supported naming convention examples

### Notes

- Use `cargo test` for running all tests
- Use `cargo test --test <test_name>` for specific integration tests
- Each parent task represents a complete vertical slice (code + tests + observability + docs)

## Tasks

- [x] 1.0 Core KQL Parser and Case Detection Library (Complete Vertical Slice)
  - [x] 1.1 Initialize Rust project with cargo workspace structure
  - [x] 1.2 Implement KQL parser for tokenizing .create table statements
  - [x] 1.3 Build parser for extracting table names from KQL statements
  - [x] 1.4 Create case detection module to identify naming conventions (snake_case, SCREAMING_SNAKE_CASE, PascalCase, camelCase, kebab-case)
  - [x] 1.5 Implement case converter utilities for transforming between conventions
  - [x] 1.6 Add logging with tracing for parser operations
  - [x] 1.7 Write integration tests for parser with sample KQL files
  - [x] 1.8 Write integration tests for case detection and conversion
  - [x] 1.9 Document parser API and case detection logic

- [ ] 2.0 Configuration System and CLI Foundation (Complete Vertical Slice)
  - [x] 2.1 Set up clap CLI with subcommands for check, fix, and init
  - [x] 2.2 Define YAML configuration schema with serde
  - [x] 2.3 Implement configuration file discovery (current dir, parent dirs)
  - [x] 2.4 Build configuration precedence system (CLI flags > config file)
  - [ ] 2.5 Create init command to generate default .klint.yml
  - [ ] 2.6 Add configuration validation with helpful error messages
  - [x] 2.7 Implement logging configuration for verbosity levels
  - [ ] 2.8 Write integration tests for CLI argument parsing
  - [ ] 2.9 Write integration tests for configuration loading and precedence
  - [x] 2.10 Document CLI usage and configuration options

- [ ] 3.0 Linting Engine with Check and Fix Modes (Complete Vertical Slice)
  - [x] 3.1 Create linting engine that uses parser to find table names
  - [x] 3.2 Implement violation detection based on configured convention
  - [x] 3.3 Build exclusion system for ignoring specified tables
  - [ ] 3.4 Create fix mode that replaces all instances of table names
  - [x] 3.5 Handle edge cases for mixed/unrecognizable conventions
  - [x] 3.6 Add detailed logging for linting operations
  - [ ] 3.7 Implement dry-run preview of fixes in check mode
  - [x] 3.8 Write integration tests for linting with various violations
  - [ ] 3.9 Write integration tests for fix mode operations
  - [x] 3.10 Document linting rules and fix behavior

- [x] 4.0 Output Formatting and Reporting System (Complete Vertical Slice)
  - [x] 4.1 Implement colored terminal output formatter with colored/termcolor
  - [x] 4.2 Create JSON output formatter for CI/CD integration
  - [x] 4.3 Build report file generator for persistent results
  - [x] 4.4 Add progress indicators for batch file processing
  - [x] 4.5 Implement error formatting with file location and line numbers
  - [x] 4.6 Create violation summary with statistics
  - [x] 4.7 Add logging for output operations
  - [x] 4.8 Write integration tests for different output formats
  - [x] 4.9 Write tests for report generation
  - [x] 4.10 Document output formats and CI/CD integration

- [x] 5.0 File Processing and Directory Traversal (Complete Vertical Slice)
  - [x] 5.1 Implement single file processing with .kql/.kusto extension support
  - [x] 5.2 Create recursive directory walker following golint conventions
  - [x] 5.3 Add glob pattern support for file matching
  - [ ] 5.4 Implement parallel file processing for performance
  - [x] 5.5 Handle file I/O errors gracefully with proper reporting
  - [x] 5.6 Add exit code management (0: success, 1: violations, 2: error)
  - [ ] 5.7 Implement incremental linting support for changed files
  - [x] 5.8 Add comprehensive logging for file operations
  - [x] 5.9 Write integration tests for file and directory processing
  - [x] 5.10 Document file processing behavior and patterns

- [ ] 6.0 Integration Tests and Sample Files (Complete Vertical Slice)
  - [ ] 6.1 Create sample KQL files with various violation patterns
  - [ ] 6.2 Add sample files with correct naming conventions
  - [ ] 6.3 Write black-box integration tests simulating user workflows
  - [ ] 6.4 Create CI/CD integration test scenarios
  - [ ] 6.5 Add performance benchmarks for file processing
  - [ ] 6.6 Create test fixtures for edge cases
  - [ ] 6.7 Write tests for exit codes and error handling
  - [ ] 6.8 Add logging verification in tests
  - [ ] 6.9 Create comprehensive README with usage examples
  - [ ] 6.10 Write configuration and naming convention documentation

## Discovered During Work

_This section will be populated with tasks discovered during implementation_