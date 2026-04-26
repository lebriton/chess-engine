# ♟️ Rust Chess Engine

A lightweight, Stockfish-style chess engine written in Rust, built primarily for learning and experimentation.

This project is part of my personal chess journey — **road to 2000 ELO** — and focuses on understanding how chess engines actually work under the hood: move generation, evaluation, and search.

---

## 🎯 Purpose

This engine explores the core mechanics of chess programming with a focus on performance and data-oriented design:

- Efficient move generation using optimized board representations (bitboards)
- Compact, cache-friendly position encoding for fast access and lightweight state updates
- Numerical evaluation of positions using material and positional heuristics

The goal is not to beat Stockfish — just to learn and have fun in the process.

---

## 🧠 What it does

WIP

---

## 🛠️ Tech Stack

- 🦀 Rust

Rust is used here specifically for its memory safety, speed, and control over low-level performance.

---

## 📦 Installation & Development

```bash
# build the project
cargo build --release

# run the engine
cargo run
```

---

## 📚 Resources

1. [https://deepwiki.com/official-stockfish/Stockfish/2-core-data-structures-and-types](https://deepwiki.com/official-stockfish/Stockfish/2-core-data-structures-and-types)
