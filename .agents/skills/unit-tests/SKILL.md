---
name: "UnitTest Creator"
description: "Generates unit tests and ensures happy path as well as negative testing"
version: "1.0"
compatibility: ["Claude", "Gemini"]
---

# Skill: Unit Test Creator

### Prerequisites
Ensure the application builds and that current existing tests pass

### Steps to Execute
1. **Test Placement** Generate unit tests direclty in the existing src files at the bottom
2. **Configuration** Annotate all `tests` module tests with `#[cfg(test)]` to ensure it only compiles when running tests
3. **Scope** Use `use super::*;` at the top of the `tests` module to allow tests of private and public functions
4. **Function Names** Name test functions descriptively using snake_case as is the practice in rust. (example: `fn test_adding_two_valid_numbers_should_succeed()`)
5. **Assertions** Use rust's build-in macros to assert tests:
  - `assert!(condition) for booleans
  - `assert_eq!(left, right) for equality
  - `assert_ne!(left, right) for inequality
  6. **Panics** Use `#[should_panic] attibute for functions that are expected to panic with invalid inputs.
