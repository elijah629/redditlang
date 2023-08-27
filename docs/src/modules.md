# Modules

Importing modules

- The import directive is `weneed`, or `bringme`.
- `weneed` and `bringme` are equivalent.
- They are followed by `r/` then the module path
- The module path is a dot seperated list of `<IDENT>`.
- Standard form:

  ```redditlang
  weneed r/<MODULE_PATH>
  ```

  or,

  ```redditlang
  bringme r/<MODULE_PATH>
  ```

Creating modules

- Modules are created from the filesystem
- For example
```txt
main.rl
a.rl
b/
  b.rl
  c.rl
```
- The module paths for this are `r/main r/a r/b.b r/b.c`
