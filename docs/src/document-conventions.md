# Document Conventions

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
  An arbitrary ( at least one ) number of `IDENT` is allowed, based on requirements.

A token suffixed by a question mark (`?`) refers to the below:

- `<IDENT>?`
  An `IDENT` is optional.

- `<IDENT...>?`
  An arbitrary number of `IDENT` is allowed, including 0.

- `(<IDENT...>)?`
  Parantheses and everything within are optional.

A `<TERM>` is an identifier, number, string or expression.
