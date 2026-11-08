# kvsqlinrust

A SQL database engine written from scratch in Rust on top of a key-value storage layer. Crate: `sqldb-rs`.

## Build & run

```bash
cargo build --release
cargo test                # 41 tests

cargo run --bin server    # 127.0.0.1:8080, or pass <addr>
cargo run --bin client    # interactive REPL to a running server
```

## SQL syntax

```sql
CREATE TABLE t (id INT PRIMARY KEY, name STRING, age INT INDEX);
DROP TABLE t;

INSERT INTO t VALUES (1, 'a', 20);
UPDATE t SET age = 21 WHERE id = 1;
DELETE FROM t WHERE id = 1;

SELECT [* | col | count(col) | min(col) | max(col) | sum(col) | avg(col)]
FROM t
[GROUP BY col]
[ORDER BY col [ASC | DESC]]
[LIMIT n] [OFFSET n];

-- joins: t1 [CROSS] JOIN | LEFT JOIN | RIGHT JOIN t2 [ON a.col = b.col]
-- types: BOOLEAN | FLOAT/DOUBLE | INTEGER/INT | STRING/TEXT/VARCHAR
-- constraints: NOT NULL | NULL | DEFAULT expr

SHOW TABLES;
SHOW TABLE `t`;
BEGIN; COMMIT; ROLLBACK;
EXPLAIN <statement>;
```

## Layout

```
src/sql/parser     lexer and recursive-descent parser -> AST
src/sql/plan       query plans (primary-key lookup, index scan, full scan)
src/sql/executor   DDL, DML, aggregation, joins, expressions
src/sql/engine     KV-backed relational engine (tables, rows, indexes)
src/storage        memory and log-structured disk engines, key encoding, MVCC
src/bin            async TCP server and REPL client
```

## Known limitations

- Edition 2021, not 2024; the async server/client relies on tokio 1.x.
- Storage encoding uses `bincode` 2.x and `fs4` 0.13; data written by older bincode 1.x builds is not guaranteed to be readable.
- Single node and teaching-oriented: no replication, no authentication, no crash-recovery guarantee beyond the disk log.
- `Cargo.lock` is not committed (library convention); it is generated on build.
