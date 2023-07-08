# Expressions

Expression types

- Conditional
- Binary
- Indexing

Conditional

- Returns a boolean.
- Standard form:

  ```redditlang
  <TERM> <CONDITIONAL_OPERATOR> <TERM>
  ```

Binary

- Standard form:

  ```redditlang
  <TERM> <MATHEMATICAL_OPERATOR> <TERM>
  ```

Indexing

- Returns type at index of array.
- Standard form:

  ```redditlang
  <TERM>[<INDEX>]
  ```

### Index

- One of the following
  - BinaryExpr
  - UNumber
  - String
  - Ident
  - (Index)
