# sylex

`sylex` is a small hobby language implemented in Rust. It currently consists of only expressions, separated by
semicolons. The rather small subset of expressions included makes this more of a fancy calculator than a real language,
but we'll get there.

Note: This README has been AI-generated, but was reviewed and corrected by a human (me).

## Features

### Values

`sylex` currently supports these runtime values:

- integers
- floating-point numbers
- booleans
- strings

### Expressions

Supported syntax includes:

- grouping with `(...)`
- arithmetic: `+`, `-`, `*`, `/`
- comparisons: `>`, `>=`, `<`, `<=`
- equality: `==`, `!=`
- logical operators: `&&`, `||`
- function calls such as `print(1 + 2)`
- the ternary operator `condition ? if-true : if-false`

### Builtins

Available builtins:

- `print(...)` - prints values to stdout
- `sin(x)` - self-explanatory
- `cos(x)` - self-explanatory
- `tan(x)` - self-explanatory
- `exit()` - exits the program immediately

## Quick start

### Build

```bash
cargo build
```

### Run the REPL

Start the interpreter with no arguments:

```bash
cargo run
```

In REPL mode, semicolons are optional.

### Run a file

Pass a source file as the single argument:

```bash
cargo run -- code/code.sy
```

When running from a file, semicolons are required at the end of expressions.

## Language overview

### Literals

```text
123
1.5
"hello"
true
```

### Example expressions

```text
1 + 2 * 3
(1 + 2) * 3
3 > 2 && 4 < 5
"Hello" == "Hello"
sin(1.3)
1 == 1 ? "True" : "False"
```

### Truthiness

The interpreter treats values as truthy/falsy like this:

- `false` is falsey
- empty strings are falsey
- 0 value floats and ints are falsey
- everything else is truthy

## Example program

The file [`code/code.sy`](code/code.sy) contains a larger example, which uses most of the available syntax.

## Notes and current limitations

- The language is currently expression-only.
- The lexer recognizes a lot more tokens, but these are not implemented currently.
- Function arguments are separated by only spaces currently, but only `print` takes more than one anyways. 
