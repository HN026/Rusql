use sqlparser::ast::{ ColumnOption, DataType, Statement };
use crate::error::{ Result, RUSQLError };
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
pub struct ParsedColumn {
    pub name: String,
    pub datatype: String,
    pub is_pk: bool,
    pub not_null: bool,
    pub is_unique: bool,
}

#[derive(Debug)]
pub struct CreateQuery {
    pub table_name: String,
    pub columns: Vec<ParsedColumn>,
}

impl CreateQuery {
    pub fn new(statement: &Statement) -> Result<CreateQuery> {
        match statement {
            Statement::CreateTable {
                name,
                columns,
                constraints,
                with_options: _,
                external: _,
                file_format: _,
                location: _,
                ..
            } => {
                let table_name = name.to_string();
                let mut parsed_columns: Vec<ParsedColumn> = Vec::new();
                let mut column_names = HashSet::new();

                for col in columns {
                    let name = col.name.to_string();

                    if !column_names.insert(name.clone()) {
                        return Err(
                            RUSQLError::Internal(format!("Duplicate column name: {}", &name))
                        );
                    }

                    let datatype = data_type_as_str(&col.data_type);

                    let mut is_pk = false;
                    let mut is_unique = false;
                    let mut not_null = false;

                    for column_option in &col.options {
                        match column_option.option {
                            ColumnOption::Unique { is_primary } => {
                                let (new_is_pk, new_is_unique, new_not_null) = handle_unique_option(
                                    is_primary,
                                    &datatype,
                                    &parsed_columns,
                                    &table_name
                                )?;

                                is_pk = new_is_pk;
                                is_unique = new_is_unique;
                                not_null = new_not_null;
                            }
                            ColumnOption::NotNull => {
                                not_null = true;
                            }
                            _ => (),
                        };
                    }

                    parsed_columns.push(ParsedColumn {
                        name,
                        datatype: datatype.to_string(),
                        is_pk,
                        not_null,
                        is_unique,
                    });
                }

                for constraint in constraints {
                    println!("{:?}", constraint);
                }

                Ok(CreateQuery {
                    table_name,
                    columns: parsed_columns,
                })
            }
            _ => Err(RUSQLError::Internal("Error Parsing Query".to_string())),
        }
    }
}

fn data_type_as_str(datatype: &DataType) -> &'static str {
    match datatype {
        DataType::SmallInt(_) => "Integer",
        DataType::Int(_) => "Integer",
        DataType::BigInt(_) => "Integer",
        DataType::Integer(_) => "Integer",
        DataType::Boolean => "Bool",
        DataType::Text => "Text",
        DataType::Varchar(_) => "Text",
        DataType::Real => "Real",
        DataType::Float(_) => "Real",
        DataType::Double => "Real",
        DataType::Decimal(_) => "Real",
        _ => {
            eprintln!("Not matched on datatype: {:?}", datatype);
            "Invalid"
        }
    }
}

fn handle_unique_option(
    is_primary: bool,
    datatype: &str,
    parsed_columns: &[ParsedColumn],
    table_name: &str
) -> Result<(bool, bool, bool)> {
    if datatype != "Real" && datatype != "Bool" {
        let is_pk = is_primary;
        if is_primary {
            if parsed_columns.iter().any(|col| col.is_pk) {
                return Err(
                    RUSQLError::Internal(
                        format!("Table {} already has more than one primary key", table_name)
                    )
                );
            }
            let not_null = true;
            return Ok((is_pk, true, not_null));
        }
        return Ok((is_pk, true, false));
    }
    Ok((false, false, false))
}
