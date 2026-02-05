//! Database container for managing multiple tables.
//! Provides O(1) table lookups using HashMap. All operations are currently in-memory.

use crate::error::{RUSQLError, Result};
use crate::sql::db::table::Table;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub db_name: String,
    pub tables: HashMap<String, Table>,
}

impl Database {
    pub fn new(db_name: String) -> Self {
        Database {
            db_name,
            tables: HashMap::new(),
        }
    }

    pub fn contains_table(&self, table_name: String) -> bool {
        self.tables.contains_key(&table_name)
    }

    #[allow(dead_code)]
    pub fn get_table(&self, table_name: String) -> Result<&Table> {
        self.tables
            .get(&table_name)
            .ok_or_else(|| RUSQLError::General(format!("Table '{}' not found", table_name)))
    }

    pub fn get_table_mut(&mut self, table_name: String) -> Result<&mut Table> {
        self.tables
            .get_mut(&table_name)
            .ok_or_else(|| RUSQLError::General(format!("Table '{}' not found", table_name)))
    }

    pub fn drop_table(&mut self, table_name: String) -> Result<()> {
        self.tables.remove(&table_name).map(|_| ()).ok_or_else(|| {
            RUSQLError::General(format!("Cannot drop table '{}': not found", table_name))
        })
    }

    #[allow(dead_code)]
    pub fn table_count(&self) -> usize {
        self.tables.len()
    }

    #[allow(dead_code)]
    pub fn list_table_names(&self) -> Vec<String> {
        self.tables.keys().cloned().collect()
    }
}
