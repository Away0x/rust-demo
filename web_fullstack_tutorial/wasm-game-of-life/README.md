```bash
# 创建项目
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name wasm-game-of-life
cd wasm-game-of-life
npm init wasm-app www
cd www
```

```bash
# 编译运行
wasm-pack build

cd www
# www 添加依赖
# "dependencies": {
#     "wasm-game-of-life": "file:../pkg"
# }
npm install
npm run start
```
