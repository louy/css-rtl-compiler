# CSS RTL Compiler WASM
A WASM library to allow running a playground of CSS RTL Compiler in the browser.

## Getting started
You need `wasm-pack` to build this:

```console
$ cargo install wasm-pack
```

Then, just build and run:
```console
$ wasm-pack build --target web
$ python3 -m http.server
$ open http://localhost:8000
```
