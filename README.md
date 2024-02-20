# assign1-rust

Implement a basic program that uses ownership concepts

# Simple String Concatenation in Rust

This project demonstrates string concatenation in Rust while emphasizing ownership, borrowing, and references. It takes two strings as input, concatenates them, and prints the result without violating any ownership rules.

## Prerequisites

- Rust compiler (`rustc`) installed ([https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install))
- Basic understanding of Rust concepts like ownership, borrowing, and references

## Code Overview

The program consists of two main parts:

- main function: Initializes two strings and calls concatenate_strings with references to them.
- concatenate_strings function: Takes two string slices as arguments, appends them to a new String using push_str, and returns the new String.

## Understanding the Code

The code focuses on demonstrating key concepts:

- Ownership: Strings own their allocated memory. Copying a string creates a new instance with separate memory.
- Borrowing: Borrowing allows temporary access to data without transferring ownership.
- References: References are pointers to existing data, providing another way to borrow without copying.

By using references in concatenate_strings, we avoid unnecessary copying and respect ownership rules.
