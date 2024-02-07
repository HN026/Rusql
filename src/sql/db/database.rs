use crate::error::{ Result, RUSQLError };
use crate::sql::db::table::Table;
use serde::{ Deserialize, Serialize };
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
        if let Some(table) = self.tables.get(&table_name) {
            Ok(table)
        } else {
            Err(RUSQLError::General(String::from("Table not found.")))
        }
    }

    pub fn get_table_mut(&mut self, table_name: String) -> Result<&mut Table> {
        if let Some(table) = self.tables.get_mut(&table_name) {
            Ok(table)
        } else {
            Err(RUSQLError::General(String::from("Table not found.")))
        }
    }
}
