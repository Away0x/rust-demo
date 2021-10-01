# Setup
```bash
cargo install diesel_cli --no-default-features --features sqlite
diesel setup
```

# Migration
```bash
# 生成 products table 迁移
diesel migration generate products
# 运行迁移
diesel migration run
diesel migration revert
```