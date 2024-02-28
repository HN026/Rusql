use crate::sql::RUSQLError;
use crate::Database;
use colored::*;
use sqlparser::ast::Statement;

pub fn drop_table(query: &Statement, db: &mut Database) -> Result<String, RUSQLError> {
    if let Statement::Drop {
        object_type, names, ..
    } = query
    {
        if let sqlparser::ast::ObjectType::Table = object_type {
            if let Some(table_name) = names.get(0) {
                db.drop_table(table_name.to_string())?;
                Ok(String::from("DROP TABLE Statement executed.")
                    .green()
                    .to_string())
            } else {
                Err(RUSQLError::Internal(
                    "Table name not found.".red().to_string(),
                ))
            }
        } else {
            Err(RUSQLError::Internal(
                "Only DROP TABLE is supported.".red().to_string(),
            ))
        }
    } else {
        Err(RUSQLError::Internal(
            "Invalid Drop Statement".red().to_string(),
        ))
    }
}
