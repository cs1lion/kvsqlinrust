# kvsqlinrust

`kvsqlinrust` is a lightweight SQL engine implemented in Rust, supporting SQL parsing, query execution, and storage engine interactions over Key-Value backends. It is also a teaching-oriented implementation: every layer (lexer, parser, planner, executor, MVCC transaction, disk log storage) is written from scratch without a third-party SQL or storage library.

The crate is named `sqldb-rs` (imported as `sqldb_rs`).

## Core Features

- **SQL Parsing & Lexing**: Custom lexer and recursive-descent parser for SQL statements.
- **Relational Query Engine**: DDL & DML processing including table creation/drop, insertion, updates, deletion, and selection.
- **Query Planning**: `EXPLAIN` support and a planner that chooses primary-key lookup, index scan, or full table scan.
- **Aggregation & Joins**: `GROUP BY` with `count`/`min`/`max`/`sum`/`avg`, plus `JOIN`, `LEFT JOIN`, `RIGHT JOIN` and `CROSS JOIN` (including a hash join optimization).
- **Indexes**: Column-level secondary indexes maintained on insert/update/delete.
- **Transactions**: `BEGIN`/`COMMIT`/`ROLLBACK` with MVCC isolation (dirty read, unrepeatable read, phantom read and write-conflict protection).
- **KV Storage Abstraction**: Abstract storage driver mapping relational schema and row records to Key-Value pairs, with memory and disk (log-structured) engines.
- **Type System**: Supports `BOOLEAN`, `FLOAT`/`DOUBLE`, `INTEGER`/`INT`, and `STRING`/`TEXT`/`VARCHAR` data types with constraint enforcement (`NOT NULL`, `DEFAULT`).
- **Server & Client**: An async TCP server plus an interactive REPL client (`rustyline`).

## Architecture & Module Overview

- **Parser Layer**: Transforms raw SQL strings into Abstract Syntax Trees (AST).
- **Planning Layer**: Builds and optimizes query plans from the AST.
- **Execution Engine**: Executes AST nodes against the underlying KV storage backend.
- **Transaction Layer**: MVCC transactions on top of the storage engine.
- **Storage Layer**: Encodes relational records into byte keys and values for KV storage.

## Quick Start & Testing

Build the project and run test suites with `cargo`:

```bash
# Build the project
cargo build --release

# Run unit and integration tests
cargo test
```

The repository contains 41 unit tests covering the parser, executor, MVCC transactions and the disk engine.

### Server & Client

Start the TCP server (defaults to `127.0.0.1:8080`, override with the first argument), then connect the REPL client:

```bash
cargo run --bin server            # or: cargo run --bin server -- 127.0.0.1:9000
cargo run --bin client            # or: cargo run --bin client -- 127.0.0.1:9000
```

## Supported SQL Syntax

### 1. Create/Drop Table
```sql
CREATE TABLE table_name (
  column_name data_type [index] [ column_constraint [...] ]
  [, ... ]
);

-- data_type: BOOLEAN(BOOL) | FLOAT(DOUBLE) | INTEGER(INT) | STRING(TEXT, VARCHAR)
-- column_constraint: [ NOT NULL | NULL | DEFAULT expr ]
```

```sql
DROP TABLE table_name;
```

### 2. Insert Into
```sql
INSERT INTO table_name [ ( column_name [, ...] ) ] VALUES ( expr [, ...] );
```

### 3. Select
```sql
SELECT [* | col_name | function [ [ AS ] output_name [, ...] ]]
FROM from_item
[GROUP BY col_name]
[ORDER BY col_name [ASC | DESC] [, ...]]
[LIMIT count]
[OFFSET count];
```

where `function` is `count(col_name)`, `min(col_name)`, `max(col_name)`, `sum(col_name)` or `avg(col_name)`.

where `from_item` is a `table_name`, or `table_name join_type table_name [ON predicate]`; `join_type` is `CROSS JOIN`, `JOIN`, `LEFT JOIN` or `RIGHT JOIN`, and `predicate` is `column_name = column_name`.

### 4. Update
```sql
UPDATE table_name SET col_name = expr [, ...] [ WHERE condition ];
```

### 5. Delete
```sql
DELETE FROM table_name [ WHERE condition ];
```

### 6. Show Tables
```sql
SHOW TABLES;
SHOW TABLE `table_name`;
```

### 7. Transactions
```sql
BEGIN;
COMMIT;
ROLLBACK;
```

### 8. Explain
```sql
EXPLAIN sql;
```
