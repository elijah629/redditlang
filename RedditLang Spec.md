# RedditLang 2023.0.1.8

This is a high level overview of redditlang’s syntax and features. It is organized by the header being the feature, a list of implementation notes, the actual syntax and then footnotes.

The official PEST grammar file, [grammar.pest](./grammar.pest) should be included with this document.

## Contents

- [RedditLang 2023.0.1.7](#redditlang-2023017)
  - [Contents](#contents)
  - [Document Conventions](#document-conventions)
  - [Blocks](#blocks)
  - [Statements](#statements)
  - [Identifier Policy](#identifier-policy)
  - [Variables](#variables)
  - [Typing](#typing)
  - [Loops](#loops)
  - [Branching](#branching)
  - [Expressions](#expressions)
  - [Functions](#functions)
  - [Errors](#errors)
  - [Comments](#comments)
  - [Primitive types](#primitive-types)
  - [Operators](#operators)
  - [Classes](#classes)
  - [Modules](#modules)
  - [Build system](#build-system)
    - [Walter](#walter)
  - [Standard library](#standard-library)

## Document Conventions

These are common syntaxes used in this document that refer to parts of the parser.

A name within chevrons (`< >`) refers to one of these:

- `IDENT`

  Alphabetic character or underscore followed by alphanumeric characters or underscores. Short for 'identifier'.

  **Example**

  ```redditlang
  text   # This is valid   ✅
  te_xt  # This is valid   ✅
  _text  # This is valid   ✅

  42     # This is invalid ❌
  42text # This is invalid ❌
  te xt  # This is invalid ❌
  ```

- `DECLARATION`

  `<IDENT> damn <TYPE>`, or

  `<IDENT>` (Without type annotations)

  Where type is `<IDENT>(<IDENT...>)?` where the paranthesised `<IDENT>` is a generic parameter. Everything in parantheses are optional and the list is separated and terminated by commas.

  **Example**

  ```redditlang
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

A `<TERM>` is an identifier, number, string or expression.

## Blocks

- Blocks are created by curly braces.

  **Example**

  ```redditlang
  { # This is a block
    # Statements can appear within a black
  } # This terminates the block
  ```

## Statements

- Statements must all be on separate lines.
- Semicolons shall not be used.
- Statements shall only appear at the top level of the file, and inside blocks.
- Top-level statements have priority in processing.

  **Example**

  ```redditlang
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

## Variables

The variable creation keyword is `meth`.

They can ony be defined at the top level of modules, or in blocks.

An initializer is required.

Standard form:

```redditlang
meth <IDENT> ∑ <EXPR>
```

## Typing

Typing a variable or a function type is optional.

Array access

- Standard form:

  ```redditlang
  <TYPE>[]
  ```

## Loops

- Loops do not self-terminate.
- The loops follow [Rust's syntax](https://doc.rust-lang.org/reference/expressions/loop-expr.html#infinite-loops).
- The loop keyword is `repeatdatshid`.
- The break keyword is `sthu`.
  Standard form:

  ```redditlang
  repeatdatshid { # Opens a loop
    # Code to execute
    sthu # Breks the loop
  }
  ```

## Branching

The if keyword is `is`.

Standard form:

```redditlang
is <EXPR> {
  # Code
}
```

The else keyword is `isnt`.

Standard form:

```redditlang
is <EXPR> {
  # Code
} isnt {
  # Code
}
```

The else-if keyword is `but`.

Standard form:

```redditlang
is <EXPR> {
  # Code
} but <EXPR> {
  # Code
} isnt {
  # Code
}
```

## Expressions

Expression types

- Conditional
- Binary
- Indexing

Conditional

- Returns a boolean.
- Standard form:

  ```redditlang
  <TERM> <CONDITIONAL_OPERATOR> <TERM>
  ```

Binary

- Standard form:

  ```redditlang
  <TERM> <MATHEMATICAL_OPERATOR> <TERM>
  ```

Indexing

- Returns type at index of array.
- Standard form:

  ```redditlang
  <TERM>[<UNSIGNED_INT>]
  ```

## Functions

Function Declaration

- The declare keyword is `callmeonmycellphone`.
- Functions have an identifier, and a return type.
- They can optionally include a modifier and an arbitrary number of arguments.
- Standard form:

  ```redditlang
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

  ```redditlang
  call <IDENT>(<EXPR...>?)
  ```

  where `(<EXPR...>?)` is a comma separated list of expressions.

Function returns

- The return keyword is `spez`.
- Standard form:

  ```redditlang
  spez <EXPR>
  ```

- The returned expression's type must match the return type, if specified.

## Errors

- An error is called a `bullet`.
- The throw keyword is `shoot`.
- Only expressions can be shot.
- Standard `shoot` form:

  ```redditlang
  shoot <EXPR>
  ```

- The try keyword is `test`.
- The catch keybord is `wall`.
- A `test-wall` is composed of one test and one wall.
- Wall statements can optionally have one expression. Without a expression, wall will catch all expressions shot.
- Standard `test-wall` form:

  ```redditlang
  test {
    # Code that possibly shoots a bullet
  } wall <IDENT>? {
    # Handle exception
  }
  ```

## Comments

Single line comments are prefixed by `#`.

Multi-line comments are prefixed by `#*` and suffixed by `*#`.

## Primitive types

### Boolean

| Varient | Meaning |
| ------- | ------- |
| `Yup`   | True    |
| `Nope`  | False   |

### Foolean

- Contains all varients of a `Boolean`
- Has these additional varients

| Varient | Meaning          |
| ------- | ---------------- |
| `Dunno` | Null             |
| `Huh`   | I/O Failure      |
| `Yeet`  | Random `Foolean` |

### String

- An array of characters.

### Number

- f64s' or doubles
- Positive by default
- Doesn't require a `<SIGN>`
- Standard form:

```redditlang
<SIGN?><UNSIGNED_INTEGER>
```

#### Sign

- `Add` or `Subtrat` operator symbol

### Null

- The null keyword is `wat`.

### Arrays

- Array types are suffixed by the `[]` token.
- Array indexes start with `1`.

## Operators

### Conditional

- The equality operator is `⅀`
- The inequality operator is `≠`
- The greater than operator is `>`
- The greater than or equal to operator is`⋝`
- The less than operator is `<`
- The less than or equal to operator is `⋜`

### Math

- The addition binary operator is `⨋`
- The subtraction binary operator is `–` **Note: This is not U+2012 ( dash ), this is U+2013 ( en dash )**
- The multiplication binary operator is `⋇`
- The XOR binary operator is `⊕`
- The division binary operator is `⎲`

### Unary

- The positive unary operator is `⨋`
- The negative unary operator is `–`
- The negation unary operator is `¡`

### Other

- The assignment operator is `∑`

## Classes

The class definition keyword is `school`.

Standard form:

```redditlang
school <IDENT> {
  # Parts of the class
}
```

The constructor member function name is `cooK`.  
The destructor member function name is `snoRt`.

Constructors and destructors have the `bar` function modifier by default.

**Example**

```redditlang
school myClass {
  callmeonmycellphone cooK() {
    # Constructor
  }

  callmeonmycellphone snoRt() {
    # Destructor
  }
}
```

Fields are placed at the top of the class.

They are private by default.

## Modules

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

## Build system

The build system is `walter`.

The standard debugger is `jesse`.

### Walter

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

## Standard library

The standard library is imported by default. If required for any reason, use the `weneed "std/<MODULE>"` or `bringme "std/<MODULE>` statements to import the requisite module(s).

Modules:

- io
- time

std/io functions

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

std/time functions

- `zzz`

  - Standard sleep function.
  - Stops the current thread for `timeMs` seconds.
  - Function signature:

    ```redditlang
    call zzz(timeMs damn Number)
    ```
