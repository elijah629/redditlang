# Functions

## Function Declaration

- The declare keyword is `callmeonmycellphone`.
- Functions have an identifier, and a return type.
- They can optionally include a modifier and an arbitrary number of arguments.
- **All** arguments, last one or not, will end with a comma.
- Standard form:

  ```redditlang
  <FUNCTION_MOD...> callmeonmycellphone <DECLARATION>(<DECLARATION,...>) {
    # Block
  }
  ```

- Modifiers
  - `Debug` modifier: Will print every variable when it is changed to the console. Only works in debug builds when the `jesse` debugger is ran with `walter`.
  - `bar` modifier: Makes function public to its scope. Only works in classes and top-level of non-main modules.
  - Modifiers are separated by spaces.
- Arguments
  - Arguments are separated by commas.

## Function Calls

- **All** arguments, last one or not, will end with a comma.
- The call keyword is `call`.
- Standard form:

  ```redditlang
  call <IDENT>(<EXPR,...>?)
  ```

## Function returns

- The return keyword is `spez`.
- Standard form:

  ```redditlang
  spez <EXPR>
  ```

- The returned expression's type must match the return type, if specified.
