use rusql::sql::db::database::Database;
use rusql::sql::process_command;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_end_to_end_table_creation() {
        let mut db = Database::new("test_db".to_string());
        let query = "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);";

        let result = process_command(query, &mut db);
        assert!(result.is_ok());
        assert!(db.contains_table("users".to_string()));
    }

    #[test]
    fn test_end_to_end_insert() {
        let mut db = Database::new("test_db".to_string());

        // Create table first
        let create_query = "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER);";
        let result = process_command(create_query, &mut db);
        assert!(result.is_ok());

        // Insert data
        let insert_query = "INSERT INTO users (name, age) VALUES ('Alice', 30);";
        let result = process_command(insert_query, &mut db);
        assert!(result.is_ok());

        let table = db.get_table("users".to_string()).unwrap();
        assert_eq!(table.last_rowid, 1);
    }

    #[test]
    fn test_end_to_end_multiple_inserts() {
        let mut db = Database::new("test_db".to_string());

        // Create table
        let create_query = "CREATE TABLE products (id INTEGER PRIMARY KEY, name TEXT, price REAL);";
        let result = process_command(create_query, &mut db);
        assert!(result.is_ok());

        // Insert multiple rows
        let insert1 = "INSERT INTO products (name, price) VALUES ('Laptop', 999.99);";
        let insert2 = "INSERT INTO products (name, price) VALUES ('Mouse', 29.99);";
        let insert3 = "INSERT INTO products (name, price) VALUES ('Keyboard', 79.99);";

        assert!(process_command(insert1, &mut db).is_ok());
        assert!(process_command(insert2, &mut db).is_ok());
        assert!(process_command(insert3, &mut db).is_ok());

        let table = db.get_table("products".to_string()).unwrap();
        assert_eq!(table.last_rowid, 3);
    }

    #[test]
    fn test_end_to_end_drop_table() {
        let mut db = Database::new("test_db".to_string());

        // Create table
        let create_query = "CREATE TABLE temp (id INTEGER);";
        assert!(process_command(create_query, &mut db).is_ok());
        assert!(db.contains_table("temp".to_string()));

        // Drop table
        let drop_query = "DROP TABLE temp;";
        assert!(process_command(drop_query, &mut db).is_ok());
        assert!(!db.contains_table("temp".to_string()));
    }

    #[test]
    fn test_list_tables() {
        let mut db = Database::new("test_db".to_string());

        // Create multiple tables
        process_command("CREATE TABLE users (id INTEGER);", &mut db).unwrap();
        process_command("CREATE TABLE posts (id INTEGER);", &mut db).unwrap();
        process_command("CREATE TABLE comments (id INTEGER);", &mut db).unwrap();

        let result = process_command("LIST TABLES;", &mut db);
        assert!(result.is_ok());
    }

    #[test]
    fn test_duplicate_table_error() {
        let mut db = Database::new("test_db".to_string());

        let create_query = "CREATE TABLE users (id INTEGER);";
        assert!(process_command(create_query, &mut db).is_ok());

        // Try to create same table again
        let result = process_command(create_query, &mut db);
        assert!(result.is_err());
    }

    #[test]
    fn test_insert_into_nonexistent_table() {
        let mut db = Database::new("test_db".to_string());

        let insert_query = "INSERT INTO nonexistent (id) VALUES (1);";
        let result = process_command(insert_query, &mut db);
        assert!(result.is_err());
    }

    #[test]
    fn test_unique_constraint_violation() {
        let mut db = Database::new("test_db".to_string());

        // Create table with unique constraint
        let create_query = "CREATE TABLE users (id INTEGER PRIMARY KEY, email TEXT UNIQUE);";
        assert!(process_command(create_query, &mut db).is_ok());

        // Insert first row
        let insert1 = "INSERT INTO users (email) VALUES ('test@example.com');";
        assert!(process_command(insert1, &mut db).is_ok());

        // Try to insert duplicate email
        let insert2 = "INSERT INTO users (email) VALUES ('test@example.com');";
        let result = process_command(insert2, &mut db);
        assert!(result.is_err());
    }

    #[test]
    fn test_column_value_count_mismatch() {
        let mut db = Database::new("test_db".to_string());

        let create_query = "CREATE TABLE users (id INTEGER, name TEXT, age INTEGER);";
        assert!(process_command(create_query, &mut db).is_ok());

        // More values than columns
        let insert_query = "INSERT INTO users (name) VALUES ('Alice', 30);";
        let result = process_command(insert_query, &mut db);
        assert!(result.is_err());
    }

    #[test]
    fn test_insert_into_nonexistent_column() {
        let mut db = Database::new("test_db".to_string());

        let create_query = "CREATE TABLE users (id INTEGER, name TEXT);";
        assert!(process_command(create_query, &mut db).is_ok());

        // Try to insert into non-existent column
        let insert_query = "INSERT INTO users (id, nonexistent) VALUES (1, 'value');";
        let result = process_command(insert_query, &mut db);
        assert!(result.is_err());
    }

    #[test]
    fn test_complex_table_with_all_constraints() {
        let mut db = Database::new("test_db".to_string());

        let create_query = "CREATE TABLE employees (
            id INTEGER PRIMARY KEY,
            email TEXT UNIQUE NOT NULL,
            name TEXT NOT NULL,
            salary REAL,
            is_active BOOLEAN
        );";

        let result = process_command(create_query, &mut db);
        assert!(result.is_ok());

        let table = db.get_table("employees".to_string()).unwrap();
        assert_eq!(table.columns.len(), 5);
        assert_eq!(table.primary_key, "id");
    }
}
