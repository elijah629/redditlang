# RedditLang 2023.0.1.6-rc.2

This is a high level overview of redditlang’s syntax and features. It is organized by the header being the feature, a list of implementation notes, the actual syntax and then footnotes.

If you are ever confused please see the official PEG grammar file that should be included with this document.

## Syntax rules and document conventions

Aight, peeps, let's kick off with some slangs:

The stuff between < and > like <IDENT> refers to:

- `IDENT`  
  Alphabetic character followed by a series of Alphanumeric or `_` characters
  Example:
  ```r
  ident - It's legit.
  2ident - Nah, fam. Not cool.
  ide_nt - Yeah, it's vibing.
  _ident - Nah, total fail.
  ```
- `DECLARATION`  
  Goes like `<IDENT> no cap <TYPE>` or just `<IDENT>` if you're not into types. TYPE is just `<IDENT>(<IDENT*>)?`, where the second IDENT and brackets are optional and you can drop some commas between them.

All other <...> - They're like headers with a different vibe.

## Loops

- Just chill, they're infinite like love for pizza.
- They vibe like Rust.

```r
repeatdatshid { # Opens a loop
    sthu # Breaks the loop
}
```

## Blocks

- Blocks are like online squads - you got them with curly brackets.

```r
{ # This is a block
# Statements can appear inside blocks
} # This ends the block
```

## Statements

- They're lone wolves, always on a separate line.
- No need for semicolons, no cap.
- Statement may only appear at the top level of the file and inside of blocks
- The top level statements are regarded first by the implementation

```r
statement
next_statement
```

## Functions

- When you wanna declare a function, just hit it with `callmeonmycellphone`
- Got modifiers, arguments, a name, and a return type.
- Modifiers - just space 'em out.
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

- To call a function, drop a call.

```
call <IDENT>(<EXPR*>)
```

Where `<EXPR*>` is a comma seperated list of expressions

### Exiting a function with a value

- Use the `spez` keyword followed by the return value.
- The type of the return value( if specified ) must match the type of the value after `spez`

```
spez <EXPR>
```

### Function Modifiers

- **`debug` modifier**: Will spill the tea on every variable. Works in debug builds with `Jesse` debugger and `Walter`.
- **`bar` modifier**: Makes the function viral in its squad.

## Identifier Policy

- `<IDENT>`is a 25-characters max thing. If you break this rule, get ready for an AntiJavaException bullet.

## Errors

- Errors are like bullets, shot with shoot and caught with wall. Try is test. Finally? Nah, it's not a thing.

## Modules

- You import a module with `weneed` or `bringme`
- You specify a string module name after the keyword in quotes

```
weneed "module_name"
bringme "module_name"
```

### Creating modules

- To set a module, use subreddit.
You gotta drop an r/ before the name.
- This can only appear once at the top of each file

```r
subreddit r/<IDENT>
```

## Build system

- `Walter` is your dude for the build system.
- `walter cook <target>` Default build target is `meth` Build files have ninja syntax
- `jesse` is the standard debugger

## Variables

- You create 'em with meth.
- They can only be defined at the top level of modules or in blocks.

```
meth <DECLARATION> ∑ <EXPR>
```

## Branching

`is`, `isnt`, and `but` are all used for `if`, `else`, and `elseif` respectively

```
is <EXPR> {

}
but <EXPR> {

}
isnt {

}

```

## Expressions

You got:

- **Conditional**: `<TERM> <CONDITIONAL_OPERATOR> <TERM>`
- **Binary**: `<TERM> <MATH_OPERATOR> <TERM>`
- **Indexing**: `<TERM>[UInt]`

A term is an identifier, number, string, or `(<EXPRESSION>)`

## Typing

- New types are marked with `specimin <NAME>`
- Type aliases with inheritance are marked with `specimin(<INHERITING TYPES>) <NAME>`
- Typing a variable or function return type is optional.
- Types that are an `Array` have a `[]` after it

```r
specimin(Flags) Week = Mon | Tue | Wed | Thu | Fri | Sat | Sun
```

```r
<TYPE>[]
```

## Classes

- For classes, just hit school

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

### Foolean

- Boolean type but very foolish

```r
specimin(Enum) Foolean =
Yup   | // True
Nope  | // Cap
Dunno | // `wat`
Huh   | // IO Failure
Yeet    // Why the hell not
```

### Flag

- Flags be bussin', ya know. You can add 'em up or subtract 'em. They're just like those `bitflags` swag in Rust (they could legit be implemented with that).

```r
specimin(Flag) OldProductTypes ∑ Type1 | Type2 | Type3
specimin(Flag) CurrentProductTypes ∑ OldProductTypes | Type4 // Add
specimin(Flag) SupportedTypes ∑ CurrentProductTypes - Type3 // Subtract
specimin(Flag) NonSupportedTypes ∑ CurrentProductTypes - SupportedTypes // Diff
```

### String

Strings be poppin' off, with an array of characters (dont tell anyone, i added my idea where a character is an array of booleans, comment e if you notice)

### Number

A number can store any flipping integer from A-Z, 0-9, Monday to Sunday. They're bussin'!

#### Decimal

- Ever wanted to use your rad numbers, with your cool decimals? Well, now you can! Like a float with more bussiness.

```r
<UNARY_OPERATOR><VALUE>.<VALUE>
```

#### Integer

- This can store whole numbers
- The way the number is stored internally should be going up to the moon y'all! Ex, start small, if cant fit, go up a bit and go back!. ( Sizes: u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 ). Decimals should be float(32, 64, 128) which should also be expanding.

```r
<UNARY_OPERATOR><VALUE>
```

### Wat

- This is nothing!

### Arrays

- Array index starts with `-1` like a cool dude!

```r
array_value[<UInt>] # Index is inside of the brackets
```

## Operators

### Conditional

- Equality: ⅀

### Math

- Add: ⨋
- Subtract: -
- Multiply: \*
- XOR: ⊕
- Divide: ⎲

### Unary

- Positive: ⨋
- Negative: -

### Other

- Assignment: ∑
- Amongus: ඞ Amongus is still cool right?

## Standard library

The standard library is already at the party! No need to invite it! If you for some reason feel the need too, `weneed "std/[module]"`

### Standard Library Modules

- io
- time

### IO Functions

- `coitusinterruptus` print dat' string! Signature `call coitusinterruptus(text damn String)`
- `pulloutnt` Read that line! Signature `call String pulloutnt()` Reads a line from stdio and returns it with no newline at the end

### Time Functions

- `zzz` makes your code as boring as you! Signature `call zzz(timeMs damn Number)` Stops the current thread for `timeMs` seconds

```

```
