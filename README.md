# rust-wasm
A template for wasm projects in Rust.

# Dependencies
```Shell
rustup wasm-pack nodejs
```

# Recipe
```Shell
Install dependencies make them available
wasm-pack build
npm install serve
npm run serve
Open browser with http://localhost:8080
```

# Quick summary for NixOS
## Use once
```Shell
nix-shell --run "npm install webpack"
```
## Then use
```Shell
nix-shell --run "npm run serve"
```
