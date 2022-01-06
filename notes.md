https://tearne.github.io/adventure/



Deps required to build `http`: `sudo apt install pkg-config libssl-dev`
```
cargo install wasm-pack
cargo install https
```
(https://github.com/thecoshman/http)

```
wasm-pack build --target web --no-typescript
http
```