# 🦀 Rust 8-Week Learning Journey

Welcome to my **Rust Learning Repository**! This repository tracks my step-by-step progress through an intensive **8-week learning roadmap** based on the book ***Programming Rust (2nd Edition)*** by Jim Blandy, Jason Orendorff, and Leonora F. S. Tindall.

The goal of this project is to build a solid foundation in Rust systems programming—from basic syntax and memory safety to advanced topics like concurrency, unsafe Rust, and FFI.

---

## 📁 Repository Structure

This repository is organized as a **Cargo Workspace**, allowing all weekly exercises to be built and tested together:

```text
rust-learning-journey/
├── Cargo.toml                 # Workspace manifest
├── README.md                  # Project overview and roadmap
├── week01_foundations/        # Week 1: Basic syntax, Cargo, and data types
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs            # Weekly exercise and unit tests
│   │   └── lib.rs
│   └── QUIZ.md                # Weekly review quiz with solutions
├── week02_ownership/          # Week 2: Ownership, borrowing, and lifetimes
├── week03_expressions_errors/ # Week 3: Expressions, error handling, modules
├── week04_structs_enums/      # Week 4: Structs, Enums, and pattern matching
├── week05_traits_iterators/   # Week 5: Traits, Generics, closures, iterators
├── week06_stdlib/             # Week 6: Standard library collections, UTF-8, I/O
├── week07_concurrency/        # Week 7: Threads, MPSC, Arc/Mutex, async/await
└── week08_advanced/           # Week 8: Macros, Unsafe Rust, and FFI
```

---

## 🛠️ How to Run Exercises & Tests

To execute all tests across the entire workspace, run:
```text
cargo test --workspace
```

To run a specific week's exercise:
```text
cargo test -p week01_foundations
```
