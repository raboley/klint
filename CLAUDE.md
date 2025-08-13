# Claude Development Guidelines for KQL Linter

## Project Context

This is a KQL (Kusto Query Language) linter for Azure Data Explorer that enforces table naming conventions. Before working on any feature:

1. **Read the PRD**: Always reference `tasks/prd-kql-linter.md` for project requirements, goals, and constraints
2. **Check task list**: Review `tasks/tasks-prd-kql-linter.md` for current implementation status
3. **Understand KQL syntax**: This tool works with `.create table` commands, not SQL syntax

## Development Workflow

### Test-Driven Development (TDD)

Follow this strict TDD cycle for all new features:

1. **RED** - Write a failing test first
   - Create integration tests that demonstrate the desired behavior
   - Tests should use actual KQL files and real data
   - Ensure the test fails for the right reason
   
2. **GREEN** - Write minimal code to make the test pass
   - Implement only what's needed to pass the test
   - Don't over-engineer or add unnecessary features
   - Focus on getting to working state quickly
   
3. **REFACTOR** - Clean up and improve the code
   - Extract common functionality into reusable modules
   - Eliminate duplication (DRY principle)
   - Improve naming and structure
   - Ensure all tests still pass

### Code Quality Standards

Every commit must meet these standards:

- ✅ **All tests passing** (`cargo test`)
- ✅ **Zero compiler warnings** (`cargo build` with no warnings)
- ✅ **All public functions documented** with rustdoc comments
- ✅ **No unused imports or dead code**
- ✅ **Clean, readable code structure**

### Reuse Before Create

Always look for existing functionality before implementing new code:

1. **Check existing modules** - Can case_detector, config, parser, etc. be extended?
2. **Look for similar patterns** - Is there existing code that does something similar?
3. **Use established crates** - Prefer well-maintained external dependencies over custom implementations
4. **Extract common utilities** - If you find yourself duplicating logic, create shared functions

## File Organization

- **Single responsibility**: Each module should have one clear purpose
- **Files over directories**: Use single files for simple modules, directories only for complex ones
- **Consistent naming**: Follow Rust conventions (snake_case for files, PascalCase for types)
- **Logical grouping**: Related functionality should be co-located

## Testing Guidelines

### Integration Tests
- Test actual user workflows end-to-end
- Use real KQL files in `samples/table_name/` directory
- Test both success and failure cases
- Verify actual output, not just that code doesn't crash

### Test Structure
```rust
#[test]
fn test_descriptive_name() {
    // Arrange - set up test data
    let input = "test data";
    
    // Act - perform the operation
    let result = function_under_test(input);
    
    // Assert - verify expected behavior
    assert_eq!(result.expected, actual, "Descriptive failure message with context");
}
```

### Test Data Management
- Use `samples/table_name/table_name/violations/` for files that should trigger violations
- Use `samples/table_name/table_name/valid/` for files that should pass validation
- Create unique test data for each test to avoid interference
- Clean up any temporary files created during tests

## Documentation Standards

### Public Functions
Every public function must have rustdoc comments:

```rust
/// Brief description of what the function does
/// 
/// # Arguments
/// 
/// * `param1` - Description of first parameter
/// * `param2` - Description of second parameter
/// 
/// # Returns
/// 
/// Description of return value and any error conditions
/// 
/// # Examples
/// 
/// ```
/// use klint::function_name;
/// let result = function_name("input");
/// assert_eq!(result, expected);
/// ```
pub fn function_name(param1: &str, param2: bool) -> Result<String> {
    // Implementation
}
```

### Module Documentation
Each module should have clear documentation:

```rust
//! Module name and purpose
//! 
//! Longer description of what this module provides
//! and how it fits into the overall architecture.
```

## Error Handling

- Use `anyhow::Result<T>` for functions that can fail
- Provide meaningful error messages with context
- Chain errors appropriately with `.with_context()`
- Handle errors gracefully at application boundaries

## Performance Considerations

- Profile before optimizing
- Prefer simplicity over premature optimization
- Use appropriate data structures for the use case
- Consider memory usage for large file processing

## Git Workflow

### Commit Messages
Follow conventional commit format:
```
feat: add new linting rule for column naming
fix: correct case detection for mixed conventions  
refactor: extract common parsing logic
docs: update configuration examples
test: add integration tests for fix mode
```

### Before Each Commit
Run this checklist:
- [ ] `cargo test` - All tests pass
- [ ] `cargo build` - No warnings
- [ ] `cargo clippy` - No clippy warnings
- [ ] `cargo fmt` - Code is formatted
- [ ] All public functions documented
- [ ] Integration tests cover new functionality

This ensures we maintain high code quality and can confidently iterate on the codebase.

## Refactoring Guidelines

Before returning to the user after implementing any working feature, always:

1. **Leverage Rust Ecosystem**: Use context7 MCP server to research idiomatic Rust patterns and existing crates that could simplify our code
2. **Apply Rust Paradigms**: Look for opportunities to use:
   - Standard traits (Display, From, Into, TryFrom, etc.)
   - Iterator patterns instead of manual loops
   - Type-driven design and the type system for correctness
   - Proper error handling with Result and ? operator
   - Builder patterns for complex configuration
3. **Simplify and Clarify**: Refactor complex logic into smaller, well-named functions that express intent clearly
4. **Remove Duplication**: Extract common patterns into reusable utilities
5. **Document Design Decisions**: Add comments explaining why certain approaches were chosen

Always commit working code first, then refactor incrementally while maintaining all tests.