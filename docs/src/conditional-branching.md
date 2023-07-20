# Conditional Branching

The if keyword is `is`.

Standard form:

```redditlang
is <EXPR> {
  # Code
}
```

The else keyword is `isn't`.

Standard form:

```redditlang
is <EXPR> {
  # Code
} isn't {
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
} isn't {
  # Code
}
```
