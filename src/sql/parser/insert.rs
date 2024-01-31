use sqlparser::ast::{ Expr, SetExpr, Statement, Value, Values };
use crate::error::{ Result, RUSQLError };

#[derive(Debug)]
pub struct InsertQuery {
    pub table_name: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl InsertQuery {
    pub fn new(statement: &Statement) -> Result<InsertQuery> {
        match statement {
            Statement::Insert { table_name, columns, source: Some(query), .. } => {
                let table_name = table_name.to_string();
                let columns = columns
                    .iter()
                    .map(|col| col.to_string())
                    .collect();
                let rowvec = extract_values(&query.body)?;

                Ok(InsertQuery {
                    table_name,
                    columns,
                    rows: rowvec,
                })
            }
            _ => Err(RUSQLError::Internal("Error Parsing Insert Query.".to_string())),
        }
    }
}

fn extract_values(body: &SetExpr) -> Result<Vec<Vec<String>>> {
    if let SetExpr::Values(Values { explicit_row: _, rows }) = body {
        Ok(
            rows
                .iter()
                .map(|row| extract_row_values(row))
                .collect()
        )
    } else {
        Err(RUSQLError::Internal("Error extracting values".to_string()))
    }
}

fn extract_row_values(row: &[Expr]) -> Vec<String> {
    row.iter()
        .filter_map(|expr| {
            match expr {
                Expr::Value(v) => Some(match_value(v)),
                Expr::Identifier(i) => Some(i.to_string()),
                _ => None,
            }
        })
        .collect()
}

fn match_value(value: &Value) -> String {
    match value {
        Value::Number(n, _) => n.to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::SingleQuotedString(s) => s.to_string(),
        Value::Null => "Null".to_string(),
        _ => {
            eprintln!("Unhandled Value variant: {:?}", value);
            String::new()
        }
    }
}
