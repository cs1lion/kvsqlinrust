## 目前支持的 SQL 语法

### 1. Create Table
```sql
   CREATE TABLE table_name {
        [ column_name data_type [ column_constraint[...] ] ]
        [, ... ]
        );
  
        where data_type is:
         - BOOLEAN(BOOL): true | false
         - FLOAT(DOUBLE)
         - INTEGER(INT)
         - STRING(TEXT, VARCHAR)
  
        where column_constraint is:
        [ NOT NULL | NULL | DEFAULT expr]
  }
```

### 2. Insert Into
```sql
   INSERT INTO table_name
   [ ( column_name [, ...] ) ]
   values ( expr [, ...] );
  
```
### 3.Select
```sql
   SELECT [* | col_name [ [ AS ] output_name [, ...] ]]
   FROM table_name
   [order by col_name [asc | desc] [, ...]]
   [LIMIT count]
   [OFFSET count]
```

### 4. Update
```sql
UPDATE table_name
SET column_name = expr [, ...]
[WHERE condition];

---
where condition is: `column_name = expr`
```
### 5.Delete
```sql
DELETE FROM table_name
[WHERE condition];
```
wHERE condition is: `column_name = expr`