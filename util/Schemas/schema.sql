-- CREATE_TABLE.sql EXAMPLE 1

CREATE TABLE employees (
    id INTEGER PRIMARY KEY,
    age SMALLINT,
    salary DECIMAL(10, 2),
    name VARCHAR(255) UNIQUE,
    email TEXT,
    is_full_time BOOLEAN,
    performance_rating REAL
);

-- INSERT.sql

INSERT INTO employees (id, age, salary, name, email, is_full_time, performance_rating)
VALUES (1, 30, 50000.00, "John Doe", "john.doe@example.com", TRUE, 3.5),
       (2, 45, 70000.00, "Jane Smith", "jane.smith@example.com", FALSE, 4.0),
       (3, 27, 55000.00, "Bob Johnson", "bob.johnson@example.com", TRUE, 4.5);

-- DROP_TABLE.sql

DROP TABLE employees;

-- CREATE_TABLE.sql EXAMPLE 2
CREATE TABLE products (
    product_id INTEGER PRIMARY KEY,
    product_name VARCHAR(255) UNIQUE,
    price DECIMAL(10, 2),
    description TEXT,
    is_in_stock BOOLEAN,
    rating REAL
);

-- INSERT.sql
INSERT INTO products (product_id, product_name, price, description, is_in_stock, rating)
VALUES (1, "Product A", 19.99, "Description for Product A", TRUE, 4.5),
       (2, "Product B", 29.99, "Description for Product B", FALSE, 4.0),
       (3, "Product C", 39.99, "Description for Product C", TRUE, 3.5);

-- DROP_TABLE.sql
DROP TABLE products;

-- CREATE_TABLE.sql EXAMPLE 3
CREATE TABLE orders (
    order_id INTEGER PRIMARY KEY,
    customer_name VARCHAR(255),
    product_id INTEGER,
    quantity SMALLINT,
    order_date TEXT
);

-- INSERT.sql
INSERT INTO orders (order_id, customer_name, product_id, quantity, order_date)
VALUES (1, "John Doe", 1, 2, "2022-01-01"),
       (2, "Jane Smith", 2, 1, "2022-01-02"),
       (3, "Bob Johnson", 1, 3, "2022-01-03");

-- DROP_TABLE.sql
DROP TABLE orders;

