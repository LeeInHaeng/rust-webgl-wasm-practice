vscode rust-analyzer 미동작 관련
================================
##### failed to find any projects in , rust-analyzer failed to discover workspace

- 하위 프로젝트의 Cargo.toml 인식 불가능 (깊은 depth 는 자동 인식이 안됨)

- .vscode 의 settings.json 수정 :  파일위치 (Window 기준) %AppData%/Roaming/Code/User/settings.json

```
"rust-analyzer.linkedProjects": [
  "relateive/path/to/the/project/directory/Cargo.toml",
]
```




WebGL Introduction
================================
`https://www.tutorialspoint.com/webgl/webgl_introduction.htm`




web-sys site
================================
`https://rustwasm.github.io/wasm-bindgen/api/web_sys/`




WASM Build
================================
- Cargo.toml 디렉토리 경로 : wasm 디렉토리 있어야됨

`cargo build --target wasm32-unknown-unknown --release`

`wasm-bindgen target/wasm32-unknown-unknown/release/{wasm 이름}.wasm --out-dir wasm --target web`