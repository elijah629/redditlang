# Modules

Importing modules

- The import directive is `weneed`, or `bringme`.
- `weneed` and `bringme` are equivalent.
- The module path is a dot seperated list of `<IDENT>`.
- Standard form:

  ```redditlang
  weneed "<MODULE_PATH>"
  ```

  or,

  ```redditlang
  bringme "<MODULE_PATH>"
  ```

Creating modules

- Modules are created from the filesystem
- For example
```
main.rl
a.rl
b/
  b.rl
  c.rl
```
- The module paths for this are `r/main r/a r/b.b r/b.c`
