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

    // TODO: Row related functions
}
