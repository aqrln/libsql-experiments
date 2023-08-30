# libsql experiments and playground

## Installing dependencies

```sh
npm install
```

## Starting sqld

With docker:

```sh
docker compose up
```

With nix (to use a pinned version or if you are on aarch64 Linux since the docker image is x86\_64 only):

```sh
nix run .#sqld-primary
```

## Running example apps

JS examples:

```sh
npm start -w http-js
```

Rust examples:

```sh
cargo run -p http-rust
```
