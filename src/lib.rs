//! # RUSQL - A SQLite-like Database Engine
//!
//! RUSQL is a lightweight, embedded database engine implemented in Rust.
//! It provides a subset of SQL functionality with a focus on simplicity and educational value.
//!
//! ## Architecture Overview
//!
//! The database engine follows a layered architecture:
//!
//! 1. **REPL Layer** - Interactive command-line interface
//! 2. **Parser Layer** - SQL statement parsing using sqlparser-rs
//! 3. **Execution Layer** - Statement execution and validation
//! 4. **Storage Layer** - In-memory data structures (BTree-based)
//!
//! ## Core Components
//!
//! - **Database**: Container for multiple tables
//! - **Table**: Schema and row storage using columnar format
//! - **Parser**: Converts SQL text to executable queries
//! - **REPL**: Read-Eval-Print-Loop for interactive usage
//!
//! ## Example Usage
//!
//! ```rust
//! use rusql::sql::db::database::Database;
//! use rusql::sql::process_command;
//!
//! let mut db = Database::new("my_database".to_string());
//!
//! // Create a table
//! let create_sql = "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);";
//! process_command(create_sql, &mut db).unwrap();
//!
//! // Insert data
//! let insert_sql = "INSERT INTO users (name) VALUES ('Alice');";
//! process_command(insert_sql, &mut db).unwrap();
//! ```
//!
//! ## Storage Engine Design
//!
//! RUSQL uses a **columnar storage model** where:
//! - Each column is stored separately in a BTreeMap
//! - Row ID serves as the key across all column BTrees
//! - Indexes are maintained separately for fast lookups
//!
//! This design provides:
//! - Efficient column scans
//! - Fast indexed lookups
//! - Memory-efficient storage for sparse data

pub mod error;
pub mod meta_command;
pub mod repl;
pub mod replloop;
pub mod sql;
pub mod util;

// Re-export commonly used types
pub use error::{RUSQLError, Result};
pub use sql::db::database::Database;
pub use sql::db::table::{Column, DataType, Table};
