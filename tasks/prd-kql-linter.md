# Product Requirements Document: KQL Linter CLI

## 1. Introduction

The KQL Linter is a command-line tool written in Rust that analyzes and automatically fixes naming convention violations in KQL (Kusto Query Language) files used with Azure Data Explorer (ADX). The tool addresses the problem of inconsistent table naming across KQL codebases by providing configurable linting rules and automated fixes. It is designed to work both as a local development tool and as part of CI/CD pipelines.

the tool should be called `klint` 

we should support two things 
    `klint <filepath>`
    `klint <filepath> --fix`

for example `klint ./... --fix`

if given a directory it should follow the conventions of over tools and probably go recursive or add a flag for recursive, whatever is most commoon. lets look at golint as our example. 

## 2. Goals

- Enforce consistent table naming conventions across KQL files
- Provide automated fixing capabilities to reduce manual refactoring effort
- Support multiple naming convention standards (snake_case, SCREAMING_SNAKE_CASE, PascalCase, camelCase, kebab-case)
- Integrate seamlessly into both local development workflows and CI/CD pipelines
- Lay the foundation for a future Language Server Protocol (LSP) implementation
- Reduce code review cycles by catching naming violations early

## 3. User Stories

**Primary User Stories:**
- As a data engineer, I want to check my KQL files for table naming violations so that I can maintain consistent code standards
- As a data engineer, I want to automatically fix table naming violations so that I don't have to manually refactor table names
- As a DevOps engineer, I want to integrate the linter into CI/CD pipelines so that naming violations are caught before merging
- As a team lead, I want to configure naming conventions via YAML so that different projects can have different standards
- As a developer, I want to exclude certain tables from linting so that legacy or external table references aren't flagged

**Secondary User Stories:**
- As a developer, I want colored terminal output so that I can quickly identify violations
- As a CI/CD system, I want JSON output format so that I can parse results programmatically
- As a developer, I want different verbosity levels so that I can control the amount of output based on my needs

## 4. Functional Requirements

1. The system must parse KQL files and identify table names in CREATE TABLE and ALTER TABLE statements
2. The system must support the following naming conventions:
   - snake_case
   - SCREAMING_SNAKE_CASE
   - PascalCase
   - camelCase
   - kebab-case
3. The system must accept a YAML configuration file that specifies:
   - Desired naming convention for tables
   - Tables to exclude from linting
   - Output format preferences
4. The system must provide two operational modes:
   - Check mode: Reports violations without making changes
   - Fix mode: Automatically corrects violations in-place
5. The system must fix all instances of a table name throughout the file when in fix mode
6. The system must support processing of:
   - Individual KQL files
   - Multiple files via glob patterns
   - Partial file content (for future LSP integration)
7. The system must provide output in:
   - Colored terminal format (default)
   - JSON format (via configuration or CLI flag)
8. The system must return appropriate exit codes:
   - 0: No violations found or all violations fixed
   - 1: Violations found (in check mode)
   - 2: Error during execution
9. The system must support multiple verbosity levels:
   - Quiet: Only show errors
   - Normal: Show violations and fixes
   - Verbose: Show all processing details
10. The system must handle edge cases:
    - Tables with mixed or unrecognizable naming conventions must be reported to the user for manual intervention
    - Invalid KQL syntax should be reported but not cause complete failure
11. The system must process common file extensions (.kql, .kusto) and be agnostic to file type when content is provided directly

## 5. Non-Goals

- This version will NOT lint column names, function names, or other KQL elements
- This version will NOT provide automatic backup creation (users should use version control)
- This version will NOT support custom regex patterns for naming conventions
- This version will NOT provide interactive fixing (all fixes are automatic or skipped)
- This version will NOT format or prettify KQL queries beyond table name corrections
- This version will NOT validate KQL syntax or semantics beyond identifying table names

## 6. Design Considerations

- **CLI Interface**: Clear, intuitive command structure with helpful error messages
- **Configuration**: YAML format for easy human readability and editing
- **Output Formatting**: 
  - Color-coded terminal output for easy scanning
  - Structured JSON for programmatic consumption
- **Error Messages**: Clear indication of:
  - File location and line number of violations
  - Current naming vs. expected naming
  - Reason why automatic fix couldn't be applied (when applicable)
- **Progress Indicators**: For batch processing of multiple files

## 7. Technical Considerations

- **Language**: Rust for performance and memory safety
- **Architecture**: Library-first design to enable future LSP implementation
- **Parser**: Robust KQL parser capable of handling partial/malformed queries
- **Configuration**: YAML parsing with schema validation
- **Testing Strategy**:
  - Black-box integration tests simulating user workflows
  - Minimal unit tests for case detection logic (if not using existing crate)
  - Sample KQL files with various violation patterns
- **Dependencies**: 
  - Evaluate existing crates for case conversion
  - YAML parsing library (e.g., serde_yaml)
  - Colored output library (e.g., colored, termcolor)
  - CLI argument parser (e.g., clap)
- **Performance**: Should process files efficiently, target < 100ms for typical KQL files

## 8. Success Metrics

- **Adoption Rate**: Tool is used in at least 80% of KQL projects within 6 months
- **CI/CD Integration**: Successfully integrated into automated pipelines
- **Fix Success Rate**: > 95% of naming violations can be automatically fixed
- **Performance**: Average processing time < 100ms per file
- **User Satisfaction**: Positive feedback on ease of use and configuration
- **Bug Reports**: < 5 critical bugs in first 3 months after release

## 9. Open Questions

1. Should the tool support incremental linting (only checking changed lines)?
   2. yes
2. What should be the default naming convention if none is specified?
   3. PascalCase
3. Should there be a "suggest" mode that provides multiple fixing options?
   4. No
4. Should the tool support project-wide configuration inheritance?
   5. Yes
5. How should the tool handle table names in comments or strings?
   6. Yes
6. Should there be a dry-run mode for the fix operation?
   7. Running it without fix should be the same thing, we should give violations and potential fixes.
7. What specific KQL statement types beyond CREATE/ALTER TABLE need support?
   8. we only want to lint on the times we create tables ince using tables may be outside our control. read https://learn.microsoft.com/en-us/azure/data-explorer/create-table-wizard for what actual create statements would be.
8. Should the tool generate a report file in addition to console output?
   9. We should have that option
9. How should configuration precedence work (CLI flags vs. config file)?
   10. cli flags should overrule config file
10. Should there be an init command to generate a default configuration file?
    11. yes