# Calculator

A small calculator core in Rust, built step by step.

## What is in it

- `Op`: the four operators (`+ - * /`) with parsing from a character,
  precedence and an `apply` that returns `None` on division by zero
- `Stack`: a stack of numbers with `push`, `pop`, `peek` and `len`, and an
  `apply` that works on the two top values and leaves the stack unchanged
  when it fails
- unit tests for all of it

## Run

    cargo run
    cargo test

## Example

`(2 + 3) * 4 - 1 = 19`, computed with only `push` and `apply`.