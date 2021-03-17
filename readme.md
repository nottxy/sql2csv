# Install
```
cargo install sql2csv
```

# Usage

## Query with inline SQL

```sql
sql2csv \
    --db "postgres://postgres@localhost/test" \
    --out "export.csv" \
    inline \
    --sql "SELECT id, name FROM users" \
    --header "id, name" 
```


## Query with SQL from TOML file
```
sql2csv \
    --db "postgres://postgres@localhost/test" \
    --out "export.csv" \
    file \
    --file test.toml
```
