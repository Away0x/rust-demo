# Setup
```bash
cargo install diesel_cli --no-default-features --features sqlite
diesel setup --database-url=database.sqlite
```

# Migration
```bash
# 生成 products table 迁移
diesel migration generate create_products
# 运行迁移
diesel migration run --database-url=database.sqlite
diesel migration revert --database-url=database.sqlite
```