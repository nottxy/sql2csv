# Install
```
cargo install sql2csv
```

# Usage
```
sql2csv --sql "select id,name from users" -- header "id,name" --db "postgres://postgres@localhost/test" --out "export.csv"
```
