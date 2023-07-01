# RedditLang 2023.0.1.6-rc.2

This is a high level overview of redditlang’s syntax and features. It is organized by the header being the feature, a list of implementation notes, the actual syntax and then footnotes.

If you are ever confused please see the official PEG grammar file that should be included with this document.

## Syntax rules and document conventions

These are some common phrases or terms used in this document that refer to parts of the parser

A name inside of < and > refers to one of these:

- `IDENT`  
  Alphabetic character followed by a series of Alphanumeric or `_` characters
  Example:

  ```r
  ident # This is an IDENT
  2ident # This is NOT an IDENT
  ide_nt # This is an IDENT
  _ident # This is NOT an IDENT
  ```

- `DECLARATION`  
  `<IDENT> damn <TYPE>` or  
  `<IDENT>` ( No types used )
  Where type is `<IDENT>(<IDENT*>)?` where the second Ident (Generic parameters) and its parenthases are optional and the list is comma seperated.

All other <...>'s refer to a header with the same name but different casing

## Loops

- There are only infinite loops
- The loops are rust-like

```r
repeatdatshid { # Opens a loop
    sthu # Breaks the loop
}
```

## Blocks

- Blocks are created with curly braces

```r
{ # This is a block
# Statements can appear inside blocks
} # This ends the block
```

## Statements

- Statements are all on separate lines
- There is no semicolon or something of that like
- Statement may only appear at the top level of the file and inside of blocks
- The top level statements are regarded first by the implementation

```r
statement
next_statement
```

## Functions

- Functions are declared with the `callmeonmycellphone` keyword
- Functions have modifiers, arguments, a name, and a return type
- Modifiers are space seperated
- The declaration's type is the return type and the Ident is the name of the function
- The arguments are comma seprated

```r
<FUNCTION_MOD*> callmeonmycellphone <DECLARATION>(<DECLARATION*>) <BLOCK>
```

Fullest example

```r
debug bar callmeonmycellphone name damn Type(arg1 damn Type) {

}
```

### Calling a function

- Functions are called with the `call` keyword

```r
call <IDENT>(<EXPR*>)
```

Where `<EXPR*>` is a comma seperated list of expressions

### Exiting a function with a value

- Use the `spez` keyword followed by the return value.
- The type of the return value( if specified ) must match the type of the value after `spez`

```r
spez <EXPR>
```

### Function Modifiers

- **`debug` modifier**: Will print every variable and when it changed to the console. Only works in debug builds when the `jesse` debugger is used is ran with `walter`.
- **`bar` modifier**: Makes the function public to its scope

## Identifier Policy

- All `<IDENT>`'s will have a max length of 25 characters. A bullet ( `AntiJavaException` ) will be shot if this rule is broken.

## Errors

- Errors are `bullet`s, you throw an error with `shoot` and catch with `wall`. Try is `test`. Finally does not exist.

## Modules

- You import a module with `weneed` or `bringme`
- You specify a string module name after the keyword in quotes

```r
weneed "module_name"
bringme "module_name"
```

### Creating modules

- Define a module with the `subreddit` keyword
- There must be an `r/` infront of the module name, it does not appear inside of the module name.
- This can only appear once at the top of each file

```r
subreddit r/<IDENT>
```

## Build system

- Possible build system is named `walter`.
- `walter cook <target>` Default build target is `meth` Build files have ninja syntax
- `jesse` is the standard debugger

## Variables

- Variables are created with the `meth` keyword.
- They can only be defined at the top level of modules or in blocks.

```r
meth <DECLARATION> ∑ <EXPR>
```

## Branching

`is`, `isnt`, and `but` are all used for `if`, `else`, and `elseif` respectively

```r
is <EXPR> {

}
but <EXPR> {

}
isnt {

}

```

## Expressions

Types of expressions:

- **Conditional**: `<TERM> <CONDITIONAL_OPERATOR> <TERM>`
- **Binary**: `<TERM> <MATH_OPERATOR> <TERM>`
- **Indexing**: `<TERM>[UInt]`

A term is an identifier, number, string, or `(<EXPRESSION>)`

## Typing

- Typing a variable or function return type is optional.
- Types that are an `Array` have a `[]` after it

```r
<TYPE>[]
```

## Classes

- They are made with the `school` keyword

```r
school <IDENT> {
    # These have `bar` by default, no need to specify
    callmeonmycellphone snoRt() {
        # Destructor
    }

    callmeonmycellphone cooK() {
        # Constructor
    }
}
```

The constructors and destructors must have these names.

Fields are placed at the top of the class, they are by default private.

## Comments

- Comments are `#` and `#*` + `*#` for multi line.

## Primitive types

### "Boolean"

- This is a `Yes or No` question

```r
Yup   | # True
Nope    # False
```

### Foolean

- Boolean type but very foolish

```r
Yup   | # True
Nope  | # False
IDK   | # Null
Huh   | # IO Failure
Yeet    # Random Foolean
```

<!-- removing typedefs ### Flag

- Flags can be subtracted, added or taken the difference of. These are like the `bitflags` crate in rust ( technically could be implemented with this ).

```r
specimin(Flag) OldProductTypes ∑ Type1 | Type2 | Type3
specimin(Flag) CurrentProductTypes ∑ OldProductTypes | Type4 # Add
specimin(Flag) SupportedTypes ∑ CurrentProductTypes - Type3 # Subtract
specimin(Flag) NonSupportedTypes ∑ CurrentProductTypes - SupportedTypes # Diff
``` -->

### String

An array of chars, a string, can represent data

### Number

A number can store any value.

#### Decimal

- This can store fractional numbers

```r
<UNARY_OPERATOR><VALUE>.<VALUE>
```

#### Integer

- This can store whole numbers
- The way the number is stored internally should be expanding. Ex, start at the smallest size, if cant fit, go up a size and repeat. ( Sizes: u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 ). Decimals should be float(32, 64, 128) which should also be expanding.

```r
<UNARY_OPERATOR><VALUE>
```

### Null

- This is `wat`

### Arrays

- Array index starts with `1`

```r
array_value[<UInt>] # Index is inside of the brackets
```

## Operators

### Conditional

- Equality: ⅀
- Anti-Equality: ≠

### Math

- Add: ⨋
- Subtract: -
- Multiply: \*
- XOR: ⊕
- Divide: ⎲

### Unary

- Positive: ⨋
- Negative: -
- Negation: ¡

### Other

- Assignment: ∑

## Standard library

The standard library is imported by default. No need to add it manually. If you for some reason want too, `weneed "std/[module]"`

### Standard Library Modules

- io
- time

### IO Functions

- `coitusinterruptus` standard print function Signature `call coitusinterruptus(text damn String)`
- `pulloutnt` standard readline function Signature `call String pulloutnt()` Reads a line from stdio and returns it with no newline at the end

### Time Functions

- `zzz` standard sleep function Signature `call zzz(timeMs damn Number)` Stops the current thread for `timeMs` seconds
