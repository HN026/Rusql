use sqlparser::ast::{ Expr, Query, SetExpr, Statement, Value, Values };

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
            Statement::Insert { table_name, columns, source, .. } => {
                let tname = table_name.to_string();
                let mut col_names = columns
                    .iter()
                    .map(|col| col.to_string())
                    .collect::<Vec<String>>();
                let mut rowvec: Vec<Vec<String>> = vec![];

                if let Some(query) = source {
                    if let Query { body: SetExpr::Values(Values { rows, .. }), .. } = &**query {
                        for row in rows {
                            let row_set: Vec<String> = row
                                .iter()
                                .map(|e| Self::parse_value_to_string(e))
                                .collect();
                            rowvec.push(row_set);
                        }
                    }
                } else {
                    return Err(RUSQLError::Internal("Error parsing Insert Query".to_string()));
                }

                Ok(InsertQuery {
                    table_name: tname,
                    columns: col_names,
                    rows: rowvec,
                })
            }
            _ => Err(RUSQLError::Internal("Error parsing Insert Query".to_string())),
        }
    }

    fn parse_value_to_string(expr: &Expr) -> String {
        match expr {
            Expr::Value(value) =>
                match value {
                    Value::Number(n, _) => n.to_string(),
                    Value::Boolean(b) => b.to_string(),
                    Value::SingleQuotedString(s) => s.to_string(),
                    Value::Null => "NULL".to_string(),
                    _ => "".to_string(),
                }
            Expr::Identifier(i) => i.to_string(),
            _ => "".to_string(),
        }
    }
}
