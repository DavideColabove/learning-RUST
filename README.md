# 🦀 Rust 8-Week Learning Journey

Welcome to my **Rust Learning Repository**! This repository tracks my step-by-step progress through an intensive **8-week learning roadmap** based on the official text, ***The Rust Programming Language*** (also known as *The Rust Book*).

The goal of this project is to build a solid foundation in Rust systems programming—from basic syntax and memory safety (Ownership & Borrowing) to advanced topics like concurrency, traits, and building a multithreaded web server.

---

## 📁 Repository Structure

This repository is organized as a **Cargo Workspace**, allowing all weekly exercises and chapters to be built and tested together:

```text
rust-learning-journey/
├── Cargo.toml                    # Workspace manifest
├── README.md                     # Project overview and roadmap
├── ch01_to_03_basics/            # Variables, Types, Functions, Control Flow & I/O
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs               # Exercises and implementations
│   │   └── lib.rs
│   └── QUIZ.md                   # Weekly review quiz with solutions
├── ch04_ownership/               # Ownership, Borrowing, References and Slices
├── ch05_06_structs_enums/        # Structs, Enums, and Pattern Matching
├── ch07_08_modules_collections/  # Packages, Modules, and Common Collections
├── ch09_10_errors_generics/      # Error Handling, Generics, Traits, and Lifetimes
├── ch11_12_tests_minigrep/       # Automated Tests and "Minigrep" CLI Project
├── ch13_14_functional_cargo/     # Closures, Iterators, and Cargo Workspaces
├── ch15_16_pointers_concurrency/ # Smart Pointers and Fearless Concurrency
├── ch17_to_19_advanced/          # OOP Features, Advanced Patterns, and Unsafe Rust
└── ch20_web_server/              # Final Project: Multithreaded Web Server
```

---

## 🎯 Progress Tracker (20 Chapters)

Here I track my progress through all the chapters of the book:

- [ ] **Chapter 1:** Getting Started
- [ ] **Chapter 2:** Programming a Guessing Game
- [ ] **Chapter 3:** Common Programming Concepts
- [ ] **Chapter 4:** Understanding Ownership
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

To execute all tests across the entire workspace, run:
```bash
cargo test --workspace
```

To run a specific chapter's exercise (e.g., the basics chapter):
```bash
cargo run -p ch01_to_03_basics
```
