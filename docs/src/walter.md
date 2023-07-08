# Walter

initializing a project

1. Navigate to your project folder in your command line.
2. Run `walter new` to initialize a new project.

Default project structure

- `src/main.rl`
- `.gitignore`
- `walter.yml`

All source files are placed in `src/`.

`src/main.rl` is the main build target.

`walter.yml` should contain a `name` and `version` attribute, within double inverted commas.

```redditlang
name: "<NAME>"
version: "<VERSION_NUMBER"
```

Building

- The command to build a project is `cook`.
- Standard form:

  ```redditlang
  walter cook
  ```

Cleaning

- The command to remove build directory is `clean`.
- Standard form:

  ```redditlang
  walter clean
  ```
