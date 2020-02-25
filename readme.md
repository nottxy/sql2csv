# Install
```
cargo install sql2csv
```

# Usage
```
cargo run -- "select * from users" --db "postgres://postgres@localhost/test" --out "export.csv"
```