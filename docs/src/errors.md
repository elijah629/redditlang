# Errors

- An error is called a `bullet`.
- The throw keyword is `shoot`.
- Only expressions can be shot.
- Standard `shoot` form:

  ```redditlang
  shoot <EXPR>
  ```

- The try keyword is `test`.
- The catch keybord is `wall`.
- A `test-wall` is composed of one test and one wall.
- Wall statements can optionally have one expression. Without a expression, wall will catch all expressions shot.
- Standard `test-wall` form:

  ```redditlang
  test {
    # Code that possibly shoots a bullet
  } wall <IDENT>? {
    # Handle exception
  }
  ```
