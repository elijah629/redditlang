# Conditional Branching

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
