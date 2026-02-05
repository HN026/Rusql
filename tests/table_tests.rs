use rusql::sql::db::table::{Column, DataType, Table};
use rusql::sql::parser::create::{CreateQuery, ParsedColumn};

#[cfg(test)]
mod table_tests {
    use super::*;

    #[test]
    fn test_datatype_creation() {
        assert_eq!(DataType::new("integer".to_string()), DataType::Integer);
        assert_eq!(DataType::new("INTEGER".to_string()), DataType::Integer);
        assert_eq!(DataType::new("text".to_string()), DataType::Text);
        assert_eq!(DataType::new("real".to_string()), DataType::Real);
        assert_eq!(DataType::new("bool".to_string()), DataType::Bool);
        assert_eq!(DataType::new("none".to_string()), DataType::None);
        assert_eq!(DataType::new("invalid_type".to_string()), DataType::Invalid);
    }

    #[test]
    fn test_table_creation() {
        let create_query = CreateQuery {
            table_name: "users".to_string(),
            columns: vec![
                ParsedColumn {
                    name: "id".to_string(),
                    datatype: "Integer".to_string(),
                    is_pk: true,
                    not_null: true,
                    is_unique: true,
                },
                ParsedColumn {
                    name: "name".to_string(),
                    datatype: "Text".to_string(),
                    is_pk: false,
                    not_null: false,
                    is_unique: false,
                },
            ],
        };

        let table = Table::new(create_query);
        assert_eq!(table.tb_name, "users");
        assert_eq!(table.columns.len(), 2);
        assert_eq!(table.last_rowid, 0);
        assert_eq!(table.primary_key, "id");
    }

    #[test]
    fn test_table_contains_column() {
        let create_query = CreateQuery {
            table_name: "users".to_string(),
            columns: vec![
                ParsedColumn {
                    name: "id".to_string(),
                    datatype: "Integer".to_string(),
                    is_pk: true,
                    not_null: true,
                    is_unique: true,
                },
                ParsedColumn {
                    name: "email".to_string(),
                    datatype: "Text".to_string(),
                    is_pk: false,
                    not_null: false,
                    is_unique: true,
                },
            ],
        };

        let table = Table::new(create_query);
        assert!(table.contains_column("id".to_string()));
        assert!(table.contains_column("email".to_string()));
        assert!(!table.contains_column("nonexistent".to_string()));
    }

    #[test]
    fn test_column_creation() {
        let column = Column::new("id".to_string(), "Integer".to_string(), true, true, true);

        assert_eq!(column.column_name, "id");
        assert_eq!(column.datatype, DataType::Integer);
        assert_eq!(column.is_pk, true);
        assert_eq!(column.not_null, true);
        assert_eq!(column.is_unique, true);
        assert_eq!(column.is_indexed, true);
    }

    #[test]
    fn test_insert_row() {
        let create_query = CreateQuery {
            table_name: "users".to_string(),
            columns: vec![
                ParsedColumn {
                    name: "id".to_string(),
                    datatype: "Integer".to_string(),
                    is_pk: true,
                    not_null: true,
                    is_unique: true,
                },
                ParsedColumn {
                    name: "name".to_string(),
                    datatype: "Text".to_string(),
                    is_pk: false,
                    not_null: false,
                    is_unique: false,
                },
                ParsedColumn {
                    name: "age".to_string(),
                    datatype: "Integer".to_string(),
                    is_pk: false,
                    not_null: false,
                    is_unique: false,
                },
            ],
        };

        let mut table = Table::new(create_query);
        let cols = vec!["name".to_string(), "age".to_string()];
        let values = vec!["John".to_string(), "25".to_string()];

        table.insert_row(&cols, &values);
        assert_eq!(table.last_rowid, 1);
    }

    #[test]
    fn test_insert_row_with_primary_key() {
        let create_query = CreateQuery {
            table_name: "users".to_string(),
            columns: vec![
                ParsedColumn {
                    name: "id".to_string(),
                    datatype: "Integer".to_string(),
                    is_pk: true,
                    not_null: true,
                    is_unique: true,
                },
                ParsedColumn {
                    name: "name".to_string(),
                    datatype: "Text".to_string(),
                    is_pk: false,
                    not_null: false,
                    is_unique: false,
                },
            ],
        };

        let mut table = Table::new(create_query);
        let cols = vec!["id".to_string(), "name".to_string()];
        let values = vec!["100".to_string(), "Alice".to_string()];

        table.insert_row(&cols, &values);
        assert_eq!(table.last_rowid, 100);
    }

    #[test]
    fn test_unique_constraint_validation() {
        let create_query = CreateQuery {
            table_name: "users".to_string(),
            columns: vec![
                ParsedColumn {
                    name: "id".to_string(),
                    datatype: "Integer".to_string(),
                    is_pk: true,
                    not_null: true,
                    is_unique: true,
                },
                ParsedColumn {
                    name: "email".to_string(),
                    datatype: "Text".to_string(),
                    is_pk: false,
                    not_null: false,
                    is_unique: true,
                },
            ],
        };

        let mut table = Table::new(create_query);

        // Insert first row
        let cols = vec!["email".to_string()];
        let values = vec!["test@example.com".to_string()];
        table.insert_row(&cols, &values);

        // Try to insert duplicate email - should fail
        let result = table.validate_unique_constraint(&cols, &values);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_column() {
        let create_query = CreateQuery {
            table_name: "users".to_string(),
            columns: vec![ParsedColumn {
                name: "id".to_string(),
                datatype: "Integer".to_string(),
                is_pk: true,
                not_null: true,
                is_unique: true,
            }],
        };

        let table = Table::new(create_query);
        let result = table.get_column("id".to_string());
        assert!(result.is_ok());

        let result = table.get_column("nonexistent".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_print_table_schema() {
        let create_query = CreateQuery {
            table_name: "users".to_string(),
            columns: vec![
                ParsedColumn {
                    name: "id".to_string(),
                    datatype: "Integer".to_string(),
                    is_pk: true,
                    not_null: true,
                    is_unique: true,
                },
                ParsedColumn {
                    name: "name".to_string(),
                    datatype: "Text".to_string(),
                    is_pk: false,
                    not_null: false,
                    is_unique: false,
                },
            ],
        };

        let table = Table::new(create_query);
        let result = table.print_table_schema();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }
}
