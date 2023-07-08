# Statements

- Statements must all be on separate lines.
- Semicolons shall not be used.
- Statements shall only appear at the top level of the file, and inside blocks.
- Top-level statements have priority in processing.

  **Example**

  ```redditlang
  # This is the top level of a file.
  statement # This is a statement.
  next_statement # This is another statement.
  statement_three statement_four # This is invalid.
  statement_five; statement_six # This is also invalid.
  {
    statement_seven # This is a statement in a block.
  }
  ```
