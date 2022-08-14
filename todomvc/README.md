# Run
```bash
# Terminal 1 - start postgresql
docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14

# Terminal 2 - build frontend
cd frontend
npm run build

# Terminal 3 - build backend
cd backend
cargo run -- ../frontend/web-folder
```

# Test & Dev
```bash
# Terminal 1 - build & watch the backend code
# cargo install cargo-watch
cd backend

cargo test -- --test-threads=1 --nocapture
# watch src 下文件的变化, 执行命令
cargo watch -q -c -w src/ -x 'run -- ../frontend/web-folder' # dev
cargo watch -q -c -w src/ -x 'test model_db_ -- --test-threads=1 --nocapture' # test

# Terminal 2 - build & watch the frontend
cd frontend
npm build -- -w
```

# DB
```bash
# Start the database
docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14

# optional psql (other terminal) 
docker exec -it -u postgres pg psql
```