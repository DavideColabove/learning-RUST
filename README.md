# 🦀 Rust 8-Week Learning Journey

Welcome to my **Rust Learning Repository** (`learning-RUST`)! This repository tracks my step-by-step progress through an intensive **8-week learning roadmap** to master systems programming, primarily following ***Programming Rust 2nd Edition***.

The goal of this project is to build a solid foundation in Rust—from basic syntax and memory safety (Ownership & Borrowing) to advanced topics like concurrency, traits, and building complex applications.

---

## 📁 Repository Structure

This repository is organized by weeks and study topics. Instead of a single Cargo Workspace, it contains independent Cargo projects, standalone scripts, and markdown notes:

```text
learning-RUST/
├── misc/                          # Study plans, quizzes, and general exercises
│   ├── rust_exercises.md
│   ├── rust_quizzes.md
│   └── rust_study_plan.md
└── week01_fundations/             # Week 1: Foundations and basic concepts
    ├── guessing_game/             # Independent Cargo project
    ├── hello_cargo/               # Independent Cargo project
    ├── hello_world.rs             # Standalone rustc executable file
    ├── basics.md                  # Personal study notes
    ├── Chapter1.md
    └── Chapter2.md
```

---

## 🎯 Progress Tracker (20 Chapters)

Here I track my progress through all the chapters of the book:

- [x] **Chapter 1:** Getting Started
- [x] **Chapter 2:** Programming a Guessing Game
- [x] **Chapter 3:** Common Programming Concepts
- [x] **Chapter 4:** Understanding Ownership
- [ ] **Chapter 5:** Using Structs to Structure Related Data
- [ ] **Chapter 6:** Enums and Pattern Matching
- [ ] **Chapter 7:** Managing Growing Projects with Packages, Crates, and Modules
- [ ] **Chapter 8:** Common Collections
- [ ] **Chapter 9:** Error Handling
- [ ] **Chapter 10:** Generic Types, Traits, and Lifetimes
- [ ] **Chapter 11:** Writing Automated Tests
- [ ] **Chapter 12:** An I/O Project: Building a Command Line Program
- [ ] **Chapter 13:** Functional Language Features: Iterators and Closures
- [ ] **Chapter 14:** More about Cargo and Crates.io
- [ ] **Chapter 15:** Smart Pointers
- [ ] **Chapter 16:** Fearless Concurrency
- [ ] **Chapter 17:** Object-Oriented Programming Features of Rust
- [ ] **Chapter 18:** Patterns and Matching
- [ ] **Chapter 19:** Advanced Features
- [ ] **Chapter 20:** Final Project: Building a Multithreaded Web Server

---

## 🛠️ How to Run Exercises & Tests

Since these are standalone projects rather than a unified Cargo Workspace, you need to navigate into the specific project directory before running Cargo commands.

To run a specific Cargo project (e.g., guessing_game):
```bash
cd week01_fundations/guessing_game
cargo run
```

To run tests for a specific project:
```bash
cd week01_fundations/guessing_game
cargo test
```

To compile and execute a standalone .rs file (e.g., hello_world.rs):
```bash
cd week01_fundations
rustc hello_world.rs
./hello_world   # On Windows: .\hello_world.exe
```