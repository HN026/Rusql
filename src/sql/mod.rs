pub mod parser;
pub mod db;

use parser::create::CreateQuery;
use parser::insert::InsertQuery;

use sqlparser::ast::Statement;
use sqlparser::dialect::SQLiteDialect;
use sqlparser::parser::{ Parser, ParserError };

use crate::error::{ Result, RUSQLError };
use crate::sql::db::database::Database;
use crate::sql::db::table::Table;

#[derive(Debug, PartialEq)]
pub enum SQLCommand {
    Insert(String),
    Delete(String),
    Update(String),
    CreateTable(String),
    DropTable(String),
    Select(String),
    Unknown(String),
}

impl SQLCommand {
    pub fn new(command: String) -> SQLCommand {
        let v = command.split(" ").collect::<Vec<&str>>();
        match v[0] {
            "insert" => SQLCommand::Insert(command),
            "update" => SQLCommand::Update(command),
            "delete" => SQLCommand::Delete(command),
            "create" => SQLCommand::CreateTable(command),
            "drop" => SQLCommand::DropTable(command),
            "select" => SQLCommand::Select(command),
            _ => SQLCommand::Unknown(command),
        }
    }
}

pub fn process_command(query: &str, db: &mut Database) -> Result<String> {
    let dialect = SQLiteDialect {};
    let mut ast = Parser::parse_sql(&dialect, &query).map_err(RUSQLError::from)?;

    if ast.len() > 1 {
        return Err(
            RUSQLError::SqlError(
                ParserError::ParserError(format!("Expected one statement, found {}", ast.len()))
            )
        );
    }

    let query = ast.pop().unwrap();

    match query {
        Statement::CreateTable { .. } => create_table(&query, db),
        Statement::Insert { .. } => insert_into_table(&query, db),
        Statement::Drop { object_type, .. } => {
            if let sqlparser::ast::ObjectType::Table = object_type {
                drop_table(&query, db)
            } else {
                Err(RUSQLError::NotImplemented("Only DROP TABLE is supported".to_string()))
            }
        }
        Statement::Query(_query) => Ok(String::from("Not Implemented yet.".to_string())),
        Statement::Delete { .. } => Ok(String::from("Not Implemented yet.".to_string())),
        _ => Err(RUSQLError::NotImplemented("SQL command not supported yet.".to_string())),
    }
}

fn create_table(query: &Statement, db: &mut Database) -> Result<String> {
    let create_query = CreateQuery::new(query)?;
    let table_name = create_query.table_name.clone();

    if db.contains_table(table_name.clone()) {
        return Err(RUSQLError::Internal(format!("Table {} already exists.", table_name)));
    }

    let table = Table::new(create_query);
    table.print_table_schema()?;
    db.tables.insert(table_name.to_string(), table);

    Ok(String::from("CREATE TABLE Statement executed."))
}

fn insert_into_table(query: &Statement, db: &mut Database) -> Result<String> {
    let insert_query = InsertQuery::new(query)?;
    let table_name = insert_query.table_name;
    let columns = insert_query.columns;
    let values = insert_query.rows;
    let db_table = db
        .get_table_mut(table_name.to_string())
        .or_else(|_| { Err(RUSQLError::Internal("Table doesn't exist.".to_string())) })?;

    if !columns.iter().all(|column| db_table.contains_column(column.to_string())) {
        return Err(RUSQLError::Internal("Cannot Insert, column doesn't exist.".to_string()));
    }

    for value in &values {
        if columns.len() != value.len() {
            return Err(
                RUSQLError::Internal(
                    format!(
                        "Column count and value count mismatch. Columns: {}, Values: {}",
                        columns.len(),
                        value.len()
                    )
                )
            );
        }

        db_table
            .validate_unique_constraint(&columns, value)
            .map_err(|err| {
                RUSQLError::Internal(format!("Unique key constraint violation: {}", err))
            })?;

        db_table.insert_row(&columns, &value);
    }

    db_table.print_table_data();
    Ok(String::from("INSERT Statement executed."))
}

fn drop_table(query: &Statement, db: &mut Database) -> Result<String> {
    if let Statement::Drop { object_type, names, .. } = query {
        if let sqlparser::ast::ObjectType::Table = object_type {
            if let Some(table_name) = names.get(0) {
                db.drop_table(table_name.to_string())?;
                Ok(String::from("DROP TABLE Statement executed."))
            } else {
                Err(RUSQLError::Internal("Table name not found.".to_string()))
            }
        } else {
            Err(RUSQLError::Internal("Only DROP TABLE is supported.".to_string()))
        }
    } else {
        Err(RUSQLError::Internal("Invalid Drop Statement".to_string()))
    }
}
