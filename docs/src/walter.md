# Walter

How to create a new project

1. Navigate to your project folder in your command line.
2. Run `walter new <PACKAGE_NAME>` to initialize a new project, if you don't specify a name it is created in the current directory with the current directories name if it is empty.

Default project structure

- `src/main.rl`
- `.gitignore`
- `walter.yml`

All source files are placed in `src/`.

`src/main.rl` is the main build target.

`walter.yml` should contain a `name` and `version` key, which are both strings.

```redditlang
name: <NAME>
version: <SEMVER_VERSION_NUMBER>
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
