# Rust: Handling Potential Panics with Vec::pop().unwrap()

This repository demonstrates a common error in Rust: using `unwrap()` on the result of `Vec::pop()`.  The `pop()` method returns an `Option`, which can be `None` if the vector is empty.  Calling `unwrap()` on `None` will cause a panic.

The `bug.rs` file shows the problematic code, while `bugSolution.rs` provides a safer alternative.

## Bug
The `bug.rs` file contains code that may panic if the vector is unexpectedly empty.  This highlights the importance of proper error handling in Rust.

## Solution
The `bugSolution.rs` file demonstrates a more robust approach using pattern matching to safely handle the `Option` returned by `pop()`, preventing panics.
