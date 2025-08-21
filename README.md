# 100 Days Of Code : Rust For MedTech Focused On The Organ Donor Problem

### Challenge Rules

#### Core
1. **Code ≥ 1 hour every day** for 100 consecutive days.
2. **Push to GitHub daily** (no backfilling). Each commit includes a brief changelog of what changed and why.
3. **No LLM-generated code.** I type every line myself. Reading docs/tutorials is fine.
4. **Comment my code.** Add concise comments explaining intent and any non-obvious logic; use `///` doc comments for public items when applicable.

#### Starting Point (“Book Phase”)
- I’m beginning from zero with **The Rust Programming Language**, **Rust by Example**, and **Rustlings**.
- Early commits can be small, exploratory, or WIP—as long as I ship daily.

#### Daily “Definition of Done”
- ✅ Update `README.md` with a brief note (What I did today, how much I code and What I learned ?).
- ✅ Post a short update on LinkedIn with #100DaysOfCode #Rust.

#### Quality (progressively enforced)
- Aim for `cargo fmt` and **`cargo clippy` clean**; these become required once I move beyond the book phase.
- Prefer safe Rust; if I ever use `unsafe`, I must document the invariant and reason.

#### Accountability
- If I pivot or refactor, I **document the reason** in the commit or PR description.
- Add screenshots/notes when something visual or performance-related changes.

#### Exception (life happens)
- **Allowed only when:** I’m **traveling**, my **laptop is not accessible** (e.g., in transit, repair, loss, restricted environment), or there is an **illness/emergency**.
- Up to **one “micro-day” per week** (≥15 minutes) still counts if I write a TIL note and push something meaningful.
- I must **state the reason** for the exception in `docs/progress.md`, and **make up for it the next day** (e.g., a longer session or an extra completed exercise/commit).

## Day 1 — 20 August 2025

### What I did today
- Started with *The Rust Programming Language* (Rust book).
- Learned how to create, build, and run projects using Cargo.
- Completed the **Guessing Game** from the Rust book (practice with input handling, loops, and comparisons).
- Built a **Git automation** tool following this tutorial: [Git Automation Tutorial](https://www.youtube.com/watch?v=nVSZFjImhRk).
- Built a **release** binary and can now run the automation script from anywhere in my project.

## Day 2 — 21 August 2025

### What I did today
- Reviewed fundamentals (felt comfortable).
- Studied **Ownership** and **Borrow Checking** — still new, so I’ll practice with **Rust by Example** and **Rustlings** tomorrow.
- Built a **CSV Reader** in Rust following this tutorial:[Rust CSV Reader](https://www.youtube.com/watch?v=VQ5cXoAMHQI)
- Picked CSV because it’s essential for **signal processing** workflows (loading time-series data, telemetry, etc.).

### Ownership (3 rules)
1) **Each value has a single owner.**  
   The binding that “owns” a value is responsible for it.

2) **Only one owner at a time.**  
   Assigning or passing a non-`Copy` value **moves** ownership to the new binding; the old one becomes unusable.

3) **When the owner goes out of scope, the value is dropped.**  
   Destructors (`Drop`) run automatically at scope end.

> Notes:  
> • Types like `i32`, `bool`, `char` are `Copy` → assignment/args **duplicate** instead of move.  
> • Heap-backed types (`String`, `Vec<T>`, etc.) move by default; clone only when you truly need two owned copies.  

### References / Borrowing (2 rules)
1) **At any time: either _any number of immutable_ borrows, or _exactly one mutable_ borrow.**  
   (Alias-or-mutate, not both.)

2) **References must always be valid.**  
   A reference can’t outlive the data it points to (no dangling refs). The compiler enforces this via lifetimes.