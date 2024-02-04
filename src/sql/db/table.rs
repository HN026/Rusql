use crate::error::{ Result, RUSQLError };
use crate::sql::parser::create::CreateQuery;
use serde::{ Deserialize, Serialize };
use std::cell::RefCell;
use std::collections::{ BTreeMap, HashMap };
use std::fmt;
use std::rc::Rc;

use prettytable::{ Cell as PrintCell, Row as PrintRow, Table as PrintTable };

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum DataType {
    Integer,
    Text,
    Real,
    Bool,
    None,
    Invalid,
}

impl DataType {
    pub fn new(cmd: String) -> DataType {
        match cmd.to_lowercase().as_ref() {
            "integer" => DataType::Integer,
            "text" => DataType::Text,
            "real" => DataType::Real,
            "bool" => DataType::Bool,
            "none" => DataType::None,
            _ => {
                eprintln!("Invalid data type given {}", cmd);
                return DataType::Invalid;
            }
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DataType::Integer => f.write_str("Integer"),
            DataType::Text => f.write_str("Text"),
            DataType::Real => f.write_str("Real"),
            DataType::Bool => f.write_str("Boolean"),
            DataType::None => f.write_str("None"),
            DataType::Invalid => f.write_str("Invalid"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Table {
    pub tb_name: String,
    pub columns: Vec<Column>,
    pub rows: Rc<RefCell<HashMap<String, Row>>>,
    pub indexes: HashMap<String, String>,
    pub last_rowid: i64,
    pub primary_key: String,
}

fn rusql_insert_datatype_based_row(
    datatype: DataType,
    col_name: String,
    table_rows: &RefCell<HashMap<String, Row>>
) {
    let mut table_rows_mut = table_rows.borrow_mut();
    match datatype {
        DataType::Integer => table_rows_mut.insert(col_name, Row::Integer(BTreeMap::new())),
        DataType::Real => table_rows_mut.insert(col_name, Row::Real(BTreeMap::new())),
        DataType::Text => table_rows_mut.insert(col_name, Row::Text(BTreeMap::new())),
        DataType::Bool => table_rows_mut.insert(col_name, Row::Bool(BTreeMap::new())),
        DataType::Invalid | DataType::None => table_rows_mut.insert(col_name, Row::None),
    }
}

fn create_error(message: &str) -> Result<()> {
    Err(RUSQLError::General(String::from(message)))
}

fn validate_column_unique_constraint(column: &mut Column, name: &str, val: &str) -> Result<()> {
    if !column.is_unique {
        return Ok(());
    }

    let col_idx = &column.index;
    match col_idx {
        Index::Integer(index) => {
            if index.contains_key(&val.parse::<i64>().unwrap()) {
                return create_error(
                    &format!(
                        "Error: Unique constraint violation for column {}. Value {} already exists.",
                        name,
                        val
                    )
                );
            }
        }

        Index::Text(index) => {
            if index.contains_key(val) {
                return create_error(
                    &format!(
                        "Error: Unique constraint violation for column {}. Value {} already exists.",
                        name,
                        val
                    )
                );
            }
        }

        Index::Real(index) => {
            if index.contains_key(&val.parse::<f64>().unwrap()) {
                return create_error(
                    &format!(
                        "Error: Unique constaint violation for column {}. Value {} already exists.",
                        name,
                        val
                    )
                );
            }
        }

        Index::None => {
            return create_error(&format!("Error: Cannot find index for column {}. ", name));
        }
    }
    Ok(())
}

impl Table {
    pub fn new(create_query: CreateQuery) -> Self {
        let table_name = create_query.table_name;
        let mut primary_key: String = String::new("-1");
        let columns = create_query.columns;
        let mut table_cols: Vec<Column> = vec![];
        let table_rows: Rc<RefCell<HashMap<String, Row>>> = Rc::new(RefCell::new(HashMap::new()));

        for col in &columns {
            let col_name = &col.name;
            if col.is_pk {
                primary_key = col_name.to_string();
            }

            table_cols.push(
                Column::new(
                    col_name.to_string(),
                    col.datatype.to_string(),
                    col.is_pk,
                    col.not_null,
                    col.is_unique
                )
            );

            rusql_insert_datatype_based_row(
                DataType::new(col.datatype.to_string()),
                col.name.to_string(),
                &table_rows
            );
        }

        Table {
            tb_name: table_name,
            columns: table_cols,
            rows: table_rows,
            indexes: HashMap::new(),
            last_rowid: 0,
            primary_key: primary_key,
        }
    }

    pub fn contains_column(&self, column: String) -> bool {
        self.columns.iter().any(|col| col.column_name == column)
    }

    pub fn get_column(&self, column_name: String) -> Result<&Column> {
        if
            let Some(column) = self.columns
                .iter()
                .filter(|c| c.column_name == column_name)
                .collect::<Vec<&Column>>()
                .first()
        {
            Ok(column)
        } else {
            Err(RUSQLError::General(String::from("Column not found.")))
        }
    }

    pub fn get_column_mut<'a>(&mut self, column_name: String) -> Result<&mut Column> {
        for elem in self.columns.iter_mut() {
            if elem.column_name == column_name {
                return Ok(elem);
            }
        }
        Err(RUSQLError::General(String::from("Column not found.")))
    }

    pub fn validate__unique_constraint(
        &mut self,
        cols: &Vec<String>,
        values: &Vec<String>
    ) -> Result<()> {
        for (idx, name) in cols.iter().enumerate() {
            let column = self.get_column_mut(name.to_string()).unwrap();
            let val = &values[idx];
            validate_column_unique_constraint(column, name, val)?;
        }
        Ok(())
    }
}
