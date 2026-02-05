use rusql::sql::parser::create::{CreateQuery, ParsedColumn};
use sqlparser::dialect::SQLiteDialect;
use sqlparser::parser::Parser;

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn test_parse_simple_create_table() {
        let sql = "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);";
        let dialect = SQLiteDialect {};
        let mut ast = Parser::parse_sql(&dialect, sql).unwrap();
        let statement = ast.pop().unwrap();

        let result = CreateQuery::new(&statement);
        assert!(result.is_ok());

        let create_query = result.unwrap();
        assert_eq!(create_query.table_name, "users");
        assert_eq!(create_query.columns.len(), 2);
    }

    #[test]
    fn test_parse_create_table_with_constraints() {
        let sql = "CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            email TEXT UNIQUE NOT NULL,
            age INTEGER
        );";
        let dialect = SQLiteDialect {};
        let mut ast = Parser::parse_sql(&dialect, sql).unwrap();
        let statement = ast.pop().unwrap();

        let result = CreateQuery::new(&statement);
        assert!(result.is_ok());

        let create_query = result.unwrap();
        assert_eq!(create_query.table_name, "users");
        assert_eq!(create_query.columns.len(), 3);

        let id_col = &create_query.columns[0];
        assert_eq!(id_col.name, "id");
        assert_eq!(id_col.datatype, "Integer");
        assert!(id_col.is_pk);

        let email_col = &create_query.columns[1];
        assert_eq!(email_col.name, "email");
        assert_eq!(email_col.datatype, "Text");
        assert!(email_col.is_unique);
        assert!(email_col.not_null);
    }

    #[test]
    fn test_parse_duplicate_column_names() {
        let sql = "CREATE TABLE users (id INTEGER, id TEXT);";
        let dialect = SQLiteDialect {};
        let mut ast = Parser::parse_sql(&dialect, sql).unwrap();
        let statement = ast.pop().unwrap();

        let result = CreateQuery::new(&statement);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_multiple_primary_keys() {
        let sql = "CREATE TABLE users (id INTEGER PRIMARY KEY, email TEXT PRIMARY KEY);";
        let dialect = SQLiteDialect {};
        let mut ast = Parser::parse_sql(&dialect, sql).unwrap();
        let statement = ast.pop().unwrap();

        let result = CreateQuery::new(&statement);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_various_data_types() {
        let sql = "CREATE TABLE test (
            int_col INTEGER,
            text_col TEXT,
            real_col REAL,
            bool_col BOOLEAN,
            varchar_col VARCHAR(255)
        );";
        let dialect = SQLiteDialect {};
        let mut ast = Parser::parse_sql(&dialect, sql).unwrap();
        let statement = ast.pop().unwrap();

        let result = CreateQuery::new(&statement);
        assert!(result.is_ok());

        let create_query = result.unwrap();
        assert_eq!(create_query.columns.len(), 5);
        assert_eq!(create_query.columns[0].datatype, "Integer");
        assert_eq!(create_query.columns[1].datatype, "Text");
        assert_eq!(create_query.columns[2].datatype, "Real");
        assert_eq!(create_query.columns[3].datatype, "Bool");
        assert_eq!(create_query.columns[4].datatype, "Text");
    }

    #[test]
    fn test_parsed_column_equality() {
        let col1 = ParsedColumn {
            name: "id".to_string(),
            datatype: "Integer".to_string(),
            is_pk: true,
            not_null: true,
            is_unique: true,
        };

        let col2 = ParsedColumn {
            name: "id".to_string(),
            datatype: "Integer".to_string(),
            is_pk: true,
            not_null: true,
            is_unique: true,
        };

        assert_eq!(col1, col2);
    }

    #[test]
    fn test_parse_table_without_primary_key() {
        let sql = "CREATE TABLE logs (message TEXT, timestamp INTEGER);";
        let dialect = SQLiteDialect {};
        let mut ast = Parser::parse_sql(&dialect, sql).unwrap();
        let statement = ast.pop().unwrap();

        let result = CreateQuery::new(&statement);
        assert!(result.is_ok());

        let create_query = result.unwrap();
        assert_eq!(create_query.columns.len(), 2);
        assert!(!create_query.columns.iter().any(|col| col.is_pk));
    }
}
