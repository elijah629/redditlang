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

You can also run `walter help <COMMAND>` to see info about a specific command, or to see info about the entire program. You can add `--help` or `-h` to get help aswell.
To print the version run `walter -V` or `walter --version`

walter `<COMMAND>`

- `cook [OPTIONS]` **Builds a program**

  **Options**

  - `-r`, `--release` Enables release mode, longer build but more optimizations
  - `-a`, `--assembly` Compiles LLVM to an assembly file instead of an object file before linking
  - `-n`, `--no-std` Does not link the standard library

- `serve [OPTIONS] [ARGS]...` **Builds and runs a program**

  **Arguments**

  - `[ARGS]...` Optional arguments to pass to the program

  **Options**

  - `-r`, `--release` Enables release mode, longer build but more optimizations
  - `-a`, `--assembly` Compiles LLVM to an assembly file instead of an object file before linking
  - `-n`, `--no-std` Does not link the standard library
  - `-s`, `--show-ir` Shows the LLVM IR when compiling

- `clean` **Removes build dir**
- `rise [NAME]` **Creates a new walter project**

  **Arguments**

  1. `[NAME]` If you don't specify a name it is created in the current directory with the current directories name if it is empty
