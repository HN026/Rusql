use rusql::sql::db::database::Database;
use rusql::sql::db::table::Table;
use rusql::sql::parser::create::{CreateQuery, ParsedColumn};

#[cfg(test)]
mod database_tests {
    use super::*;

    #[test]
    fn test_database_creation() {
        let db = Database::new("test_db".to_string());
        assert_eq!(db.db_name, "test_db");
        assert_eq!(db.tables.len(), 0);
    }

    #[test]
    fn test_database_contains_table() {
        let mut db = Database::new("test_db".to_string());
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
        db.tables.insert("users".to_string(), table);

        assert!(db.contains_table("users".to_string()));
        assert!(!db.contains_table("posts".to_string()));
    }

    #[test]
    fn test_get_table() {
        let mut db = Database::new("test_db".to_string());
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
        db.tables.insert("users".to_string(), table);

        let result = db.get_table("users".to_string());
        assert!(result.is_ok());

        let result = db.get_table("nonexistent".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_get_table_mut() {
        let mut db = Database::new("test_db".to_string());
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
        db.tables.insert("users".to_string(), table);

        let result = db.get_table_mut("users".to_string());
        assert!(result.is_ok());

        let result = db.get_table_mut("nonexistent".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_drop_table() {
        let mut db = Database::new("test_db".to_string());
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
        db.tables.insert("users".to_string(), table);

        assert!(db.contains_table("users".to_string()));

        let result = db.drop_table("users".to_string());
        assert!(result.is_ok());
        assert!(!db.contains_table("users".to_string()));

        let result = db.drop_table("nonexistent".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_tables() {
        let mut db = Database::new("test_db".to_string());

        let create_query1 = CreateQuery {
            table_name: "users".to_string(),
            columns: vec![ParsedColumn {
                name: "id".to_string(),
                datatype: "Integer".to_string(),
                is_pk: true,
                not_null: true,
                is_unique: true,
            }],
        };
        let table1 = Table::new(create_query1);
        db.tables.insert("users".to_string(), table1);

        let create_query2 = CreateQuery {
            table_name: "posts".to_string(),
            columns: vec![ParsedColumn {
                name: "id".to_string(),
                datatype: "Integer".to_string(),
                is_pk: true,
                not_null: true,
                is_unique: true,
            }],
        };
        let table2 = Table::new(create_query2);
        db.tables.insert("posts".to_string(), table2);

        assert_eq!(db.tables.len(), 2);
        assert!(db.contains_table("users".to_string()));
        assert!(db.contains_table("posts".to_string()));
    }
}
