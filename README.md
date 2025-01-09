# Learning Rust Projects

This repository contains a collection of projects built while learning Rust through "The Rust Programming Language" book. Each project demonstrates different concepts and progressively builds on knowledge from previous chapters.

## Projects Overview

### 1. Simple Bank Account (Chapters 1-5)
A basic banking system demonstrating fundamental Rust concepts.

#### Concepts Covered:
- Structs and methods
- Basic error handling
- Ownership and borrowing
- Basic data types
- Control flow

#### Key Features:
- Create accounts with initial balance
- Deposit and withdraw funds
- Transfer between accounts
- Track transactions
- Calculate interest for savings accounts

#### Example Usage:
```rust
let mut checking = Account::new(
    String::from("John Doe"),
    1000.0,
    String::from("CHK-12345"),
    AccountType::Checking,
);

checking.deposit(500.0).unwrap();
checking.withdraw(200.0).unwrap();
```

### 2. Task Manager (Chapters 1-8)
A command-line task management system showcasing more advanced Rust features.

#### Concepts Covered:
- Modules and crates
- Collections (HashMap, Vec)
- Advanced error handling with Option
- Module organization
- Public/private interfaces

#### Key Features:
- Create tasks with priority levels
- Add and manage tags
- Track task status
- List and filter tasks
- Delete tasks
- Transaction history

#### Example Usage:
```rust
let mut storage = TaskStorage::new();

let task_id = storage.add_task(
    String::from("Learn Rust"),
    String::from("Complete chapter 9"),
    TaskPriority::High,
);

if let Some(task) = storage.get_task_mut(task_id) {
    task.add_tag(String::from("programming"));
    task.mark_in_progress();
}
```

## Project Structure
```
learn_rust/
├── simple_bank_account/
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
│
└── task_manager/
    ├── src/
    │   ├── main.rs
    │   ├── task.rs
    │   ├── storage.rs
    │   └── display.rs
    └── Cargo.toml
```

## Concepts by Chapter

### Chapters 1-5
- Variables and mutability
- Data types
- Functions
- Control flow
- Ownership concepts
- Structs and methods
- Enums and pattern matching

### Chapters 6-8
- Modules
- Packages and crates
- Collections (Vec, HashMap)
- Error handling
- Generic types
- Public/private interfaces

## Running the Projects

1. Clone the repository
2. Navigate to the desired project directory
```bash
cd learn_rust/task_manager
# or
cd learn_rust/simple_bank_account
```
3. Run the project
```bash
cargo run
```

## Next Steps
Future projects will cover:
- Advanced error handling (Chapter 9)
- Generic types and traits (Chapter 10)
- Testing (Chapter 11)
- Command line program (Chapter 12)
- Functional language features (Chapter 13)
- Concurrency (Chapter 16)

## Learning Resources
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)