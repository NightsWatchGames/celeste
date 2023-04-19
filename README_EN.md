# celeste classic
- [x] Aseprite software design pixel art
- [x] Ldtk software edit level
- [x] Load level
- [x] Character jump
- [x] Character dash and animation
- [x] Character climb wall
- [x] Character die and animation
- [x] Character hair effect
- [x] Spring, snow, trap, wooden-stand
- [x] Weather effect
- [x] Camera follows character
- [x] Game ui
- [x] WASM support

Play online: [click here](https://nightswatchgames.github.io/games/celeste/)（Open with PC Chrome/Firefox/Edge）

## Get started
1. native
```
cargo run
```
2. WASM
```
rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner
cargo run --target wasm32-unknown-unknown
```
```
cargo install wasm-bindgen-cli
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/celeste.wasm
```

## Control
- `A` `D` Move
- `K` Jump
- `J` Dash

## Screenshots
Game video: [YouTube](https://www.youtube.com/watch?v=Zcou6M_sQKc)
![](screenshots/start-menu.png)
![](screenshots/play-game.png)

## Reference
- [Celeste Official source project](https://github.com/NoelFB/Celeste)
- [U3D教程实现《蔚蓝 Celeste 》Movement 系统](https://www.bilibili.com/video/BV1D4411d7Xn)
- [casuak/Game_1_Tiny_Celeste_v3](https://github.com/casuak/Game_1_Tiny_Celeste_v3)
- [LDtk一小时完全入门教程](https://www.bilibili.com/video/BV1y64y1z7Uw)