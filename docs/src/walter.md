# Walter

## How to create a new project

1. Navigate to your project folder in your command line.
2. Run `walter rise <PACKAGE_NAME>` to initialize a new project, if you don't specify a name it is created in the current directory with the current directories name if it is empty.

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

## CLI Documentation

Run `walter help <COMMAND?>` to see info about a specific command, or to see info about the entire program. You can add `--help` or `-h` to get help aswell.

> There used to be a documentation here, but it was not up to date and became a hassle to update. Please refer to the cli instead.
