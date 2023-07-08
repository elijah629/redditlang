# Standard library

The standard library is imported by default.
It is available at <https://github.com/elijah629/redditlang-stl>

## I/O

- `coitusinterruptus`

  - Standard print function.
  - Function signature:

    ```redditlang
    call coitusinterruptus(text damn String)
    ```

- `pulloutnt`

  - Standard readline.
  - Reads a line from `stdio` and returns it without a newline at the end.
  - Function signature:

    ```redditlang
    call String pulloutnt()
    ```

## Time

- `zzz`

  - Standard sleep function.
  - Stops the current thread for `timeMs` seconds.
  - Function signature:

    ```redditlang
    call zzz(timeMs damn Number)
    ```
