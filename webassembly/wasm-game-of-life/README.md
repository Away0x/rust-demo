# Conway's Game of Life
## Init
```bash
# init template
cargo generate --git https://github.com/rustwasm/wasm-pack-template
cd wasm-game-of-life
wasm-pack build

# init frontend
npm init wasm-app www
cd www
npm install
```

## Start
```bash
cd wasm-game-of-life
wasm-pack build

cd www
npm run start
```

## Test
```bash
wasm-pack test --chrome --headless
```