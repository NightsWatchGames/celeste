# celeste 模仿蔚蓝的平台跳跃游戏
- [x] Aseprite软件设计像素美术
- [x] Ldtk软件编辑关卡
- [x] 加载关卡
- [ ] 弹簧、雪堆、陷阱、移动平台
- [ ] 角色跳跃
- [ ] 角色冲刺及动画
- [ ] 角色爬墙
- [ ] 角色死亡重生及动画特效
- [x] 角色头发飘逸效果
- [x] 天气效果
- [ ] 相机跟随角色
- [x] 游戏ui
- [ ] WASM支持

## 运行
1. 本地运行
```
cargo run
```
2. WASM运行
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

## 参考
- [Celeste Official source project](https://github.com/NoelFB/Celeste)
- [U3D教程实现《蔚蓝 Celeste 》Movement 系统](https://www.bilibili.com/video/BV1D4411d7Xn)
- [casuak/Game_1_Tiny_Celeste_v3](https://github.com/casuak/Game_1_Tiny_Celeste_v3)
- [LDtk一小时完全入门教程](https://www.bilibili.com/video/BV1y64y1z7Uw)

## 问题
**1.像素完美是什么意思？**

**2.角色控制有哪些方式？**