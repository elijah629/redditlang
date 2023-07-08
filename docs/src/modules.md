# Modules

Importing modules

- The import keyword is `weneed`, or `bringme`.
- `weneed` and `bringme` are functionally equivalent.
- A module name must be specified after the keyword in quotes.
- Standard form:

  ```redditlang
  weneed "<MODULE_NAME>"
  ```

  or,

  ```redditlang
  bringme "<MODULE_NAME>"
  ```

Creating modules

- The module definition keyword is `subreddit`.
- An `r/` must exist in front of the module name. It must not appear within the module name.
- The `subreddit` keyword can only appear once, at the top of each file.
- Standard form:

  ```redditlang
  subreddit r/<MODULE_NAME>
  ```
