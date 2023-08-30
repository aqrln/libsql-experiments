# libsql experiments and playground

## Installing dependencies

```
npm install
```

## Starting sqld

With docker:

```
docker compose up
```

With nix (to use a pinned version or if you are on aarch64 Linux since the docker image is x86\_64 only):

```
nix run .#sqld-primary
```

## Running example apps

JS examples:

```
npm start -w http-js
```

Rust examples:

```
cargo run -p http-rust
```
