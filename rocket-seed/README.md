```bash
# 操作 sqlite

# 查看数据库
echo .dump | sqlite3 database.sqlite
# 查看数据
echo 'select * from task;' | sqlite3 database.sqlite
```
```bash
# build
cargo build --all
cargo build -p frontend
cargo build -p backend
```
```bash
# bin

# 插入数据
cargo run -p backend --bin todo new "do the thing1"
cargo run -p backend --bin todo new "do the thing2"
# 查找数据
cargo run -p backend --bin todo show
cargo run -p backend --bin todo show "do the thing1"
# 修改数据
cargo run -p backend --bin todo done "do the thing1"
# 删除
cargo run -p backend --bin todo delete "do the thing1"
# 启动 server
cargo run -p backend --bin backend
# localhost:8000/tasks1
# localhost:8000/tasks2
```