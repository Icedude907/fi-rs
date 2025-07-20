# `fi!`

`fi!` is a rust macro designed to reduce pain points of conditional expressions.

I end up writing an equivalent in virtually every rust project I start, so may as well break it into a separate crate. Rust's pattern matching syntax is a little odd.

### Functionality
1. Ternary operator syntax:  
   Rewrite `{if x == y { 10 } else { 20 }}` as `fi!(x == y, 10, 20)`.
2. Inline `if let`:  
   Rewrite `let x = if let Foo::A(a) = bar && a == 20 { true } else { false };` as `let x = fi!(let Foo::A(a) && a == 20);`  
   Can also replace the `matches!` macro if you prefer the if-let syntax: `matches!(x, Some(_))` -> `fi!(let Some(_) = x)`

### Known shortcomings:
- `let PATTERN` cannot be used in the ternary syntax.  
  `fi!(let Some(x) = y, x, 99)` does **not** expand to `{if let Some(x) = y { x } else { 99 }}`. This could be fixable but I've yet to come across a scenario I really wanted this.
