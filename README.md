# SWPS Programming Languages 2026: Rust solutions

My solutions to the Rust assignments from the Programming Languages course at SWPS University. I'm at the beginner stage and add new assignments as the course goes on.

## Assignments

| File | Topics |
|------|--------|
| [assignment01.rs](assignment01.rs) | Ownership and borrowing: fixing compiler errors with references (`&String`, `&str`), borrow ordering, returning owned values. Also string basics: `first_word`, `count_vowels`, `reverse_words`, `capitalize`. |
| [assignment02.rs](assignment02.rs) | Vectors and slices (`&[i32]`, `&mut [i32]`, `Option`), structs and methods (`Point`, `Rectangle`), structs that own their data (`Book`, `Library`). |
| [assignment03.rs](https://github.com/Ilginceylan/Programming-languages-SWPS2026/blob/main/assignment03.rs) | Counting words with a `HashMap<String, usize>`: `count_words`, `count_of`, `total_words`, `sorted_words`. Lowercasing words, looking up counts with `get`, and sorting a list of `(String, usize)` pairs. |

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
- Counting things with a `HashMap` (`get`, `insert`) and sorting the results
