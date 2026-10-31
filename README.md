# SWPS Programming Languages 2026: Rust solutions

My solutions to the Rust assignments from the Programming Languages course at SWPS University. I'm at the beginner stage and add new assignments as the course goes on.

## Assignments

| File | Topics |
|------|--------|
| [assignment01.rs](assignment01.rs) | Ownership and borrowing: fixing compiler errors with references (`&String`, `&str`), borrow ordering, returning owned values. Also string basics: `first_word`, `count_vowels`, `reverse_words`, `capitalize`. |
| [assignment02.rs](assignment02.rs) | Vectors and slices (`&[i32]`, `&mut [i32]`, `Option`), structs and methods (`Point`, `Rectangle`), structs that own their data (`Book`, `Library`). |

Each fix is explained in a short comment above the code.

## Running

Install Rust from [rustup.rs](https://rustup.rs), then:

```bash
rustc assignment01.rs
./assignment01        # on Windows: assignment01.exe
```

## What I'm learning

- How ownership, moves and borrowing work, and how to read the borrow checker's errors
- Using slices (`&[T]`, `&str`) instead of owning types in function parameters
- Modeling data with structs and `impl` blocks
