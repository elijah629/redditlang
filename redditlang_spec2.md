# RedditLang 2023.0.placeholder

This is a high level overview of redditlang’s syntax and features. It is organized by the header being the feature, a list of implementation notes, the actual syntax and then footnotes.

The official PEG grammar file `grammar.pest` should be included with this document.

## Document Conventions

These are common syntaxes used in this document that refer to parts of the parser.

A name within chevrons (`< >`) refers to one of these:

- `IDENT`
  Alphabetic character followed by alphanumeric characters or underscores. Short for 'identifier'.
  Example:

  ```r
  ident # This is an identifier
  2ident # This is NOT an identifier
  ide_nt # This is an identifier
  _ident # This is NOT an identifier 
  ```

- `DECLARATION`
  `<IDENT> damn <TYPE>`, or
  `<IDENT>` (Without type annotations)
  Where type is `<IDENT>(<IDENT...>)?` where the paranthesised `<IDENT>` is a generic parameter. Everything in parantheses are optional and the list is separated and terminated by commas.
  Example:

  ```r
  identifier damn Number(x damn Number,)
  ```

- A header with the same name, but different casing

A name within chevrons (`< >`), and suffixed by horizontal ellipses (`...`) refers to the below:

- `<IDENT...>`
  An arbitrary number of arguments is allowed, based on requirements.

A token suffixed by a question mark (`?`) refers to the below:

- `<IDENT>?`
  An argument is optional.

- `<IDENT...>?`
  An arbitrary number of arguments is allowed, including 0.

- `(<IDENT...>)?`
  Parantheses and everything within are optional.

## Loops

- Loops do not self-terminate.
- The loops follow [Rust's syntax](https://doc.rust-lang.org/reference/expressions/loop-expr.html#infinite-loops).
- The loop keyword is `repeatdatshid`.
- The break keyword is `sthu`.
  Standard form:
  
  ```r
  repeatdatshid { # Opens a loop
    # Code to execute 
    sthu # Breks the loop 
  }
  ```

## Blocks

- Blocks are created by curly braces.
  Example:

  ```r
  { # This is a block
    # Statements can appear within a black
  } # This terminates the block
  ```

## Statements

- Statements must all be on separate lines.
- Semicolons shall not be used.
- Statements shall only appear at the top level of the file, and inside blocks.
- Top-level statements have priority in processing.
  Example:

  ```r
  # This is the top level of a file.
  statement # This is a statement.
  next_statement # This is another statement.
  statement_three statement_four # This is invalid.
  statement_five; statement_six # This is also invalid.
  {
    statement_seven # This is a statement in a block.
  }
  ```

## Identifier Policy

- All `<IDENT>`'s will have a max length of 25 characters.
- If violated, an [`AntiJavaException`](#errors) bullet will be shot.

## Functions

Function Declaration

- The declare keyword is `callmeonmycellphone`.
- Functions have an identifier, and a return type.
- They can optionally include a modifier and an arbitrary number of arguments.
- Standard form:
  
  ```r
  <FUNCTION_MOD...> callmeonmycellphone <DECLARATION>(<DECLARATION...>?) {
    # Block 
  }
  ```

- Modifiers
  - `Debug` modifier: Will print every variable when it is changed to the console. Only works in debug builds when the `jesse` debugger is ran with `walter`.
  - `bar` modifier: Makes function public to its scope. Only works in classes and top-level of non-main modules.
  - Modifiers are separated by spaces.
- Arguments
  - Arguments are separated by commas.

Function Calls

- The call keyword is `call`.
- Standard form:

  ```r
  call <IDENT>(<EXPR...>?)
  ```

  where `(<EXPR...>?)` is a comma separated list of expressions.

Function returns

- The return keyword is `spez`.
- Standard form:

  ```r
  spez <EXPR>
  ```

- The returned expression's type must match the return type, if specified.

## Errors

- An error is called a `bullet`.
- The throw keyword is `shoot`.
- Only expressions can be shot.
- Standard `shoot` form:
  
  ```r
  shoot <EXPR>
  ```

- The try keyword is `test`.
- The catch keybord is `wall`.
- A `test-wall` is composed of one test and one wall.
- Wall statements can optionally have one expression. Without a expression, wall will catch all expressions shot.
- Standard `test-wall` form:
  
  ```r
  test {
    # Code that possibly shoots a bullet 
  } wall <IDENT>? {
    # Handle exception
  }
  ```

## Modules

Importing modules

- The import keyword is `weneed`, or `bringme`.
- `weneed` and `bringme` are functionally equivalent.
- A module name must be specified after the keyword in quotes.
- Standard form:

  ```r
  weneed "<MODULE_NAME>"
  ```

  or,
  
  ```r
  bringme "<MODULE_NAME>"
  ```

Creating modules

- The module definition keyword is `subreddit`.
- An `r/` must exist in front of the module name. It must not appear within the module name.
- The `subreddit` keyword can only appear once, at the top of each file.
- Standard form:

  ```r
  subreddit r/<MODULE_NAME>
  ```

## Build system
