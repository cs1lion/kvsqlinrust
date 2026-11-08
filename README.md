# kvsqlinrust

`kvsqlinrust` is a lightweight SQL engine implemented in Rust, supporting SQL parsing, query execution, and storage engine interactions over Key-Value backends.

## Core Features

- **SQL Parsing & Lexing**: Custom lexer and recursive-descent parser for SQL statements.
- **Relational Query Engine**: DDL & DML processing including table creation, insertion, updates, deletion, and selection.
- **KV Storage Abstraction**: Abstract storage driver mapping relational schema and row records to Key-Value pairs.
- **Type System**: Supports `BOOLEAN`, `FLOAT`/`DOUBLE`, `INTEGER`/`INT`, and `STRING`/`TEXT`/`VARCHAR` data types with constraint enforcement (`NOT NULL`, `DEFAULT`).

## Architecture & Module Overview

- **Parser Layer**: Transforms raw SQL strings into Abstract Syntax Trees (AST).
- **Execution Engine**: Executes AST nodes against the underlying KV storage backend.
- **Storage Layer**: Encodes relational records into byte keys and values for KV storage.

## Quick Start & Testing

Build the project and run test suites with `cargo`:

```bash
# Build the project
cargo build --release

# Run unit and integration tests
cargo test
```

## Supported SQL Syntax

### 1. Create Table
```sql
CREATE TABLE table_name (
  column_name data_type [ column_constraint [...] ]
  [, ... ]
);
```

### 2. Insert Into
```sql
INSERT INTO table_name [ ( column_name [, ...] ) ] VALUES ( expr [, ...] );
```

### 3. Select
```sql
SELECT [* | col_name [ AS output_name [, ...] ]] FROM table_name [ ORDER BY col_name [ASC | DESC] ] [ LIMIT count ];
```

### 4. Update
```sql
UPDATE table_name SET col_name = expr [, ...] [ WHERE condition ];
```

### 5. Delete
```sql
DELETE FROM table_name [ WHERE condition ];
```
