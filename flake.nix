{
  description = "libsql experiments";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.stable.latest.default;
        nodejs = pkgs.nodejs_latest;

      in
      rec {
        formatter = pkgs.nixpkgs-fmt;

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rust
            rust-analyzer

            nodejs
            nodejs.pkgs.prettier
            nodejs.pkgs.typescript-language-server
            nodejs.pkgs.vscode-langservers-extracted
          ];

          inputsFrom = builtins.attrValues packages;
        };

        packages.sqld = pkgs.rustPlatform.buildRustPackage rec {
          pname = "sqld";
          version = "0.20.0-dev";

          src = pkgs.fetchFromGitHub {
            owner = "libsql";
            repo = "sqld";
            rev = "28a864605b1303ef6ed1685b6c19936f1534ffb5";
            hash = "sha256-K9N0tT/JnrNX9f7nVbmcI69LLFnN1I8oTr2Nw4dwtZA=";
          };

          cargoLock = {
            lockFile = ./sqld-lockfile.toml;
            outputHashes = {
              "console-api-0.5.0" = "sha256-ZaRFGPrvUwFEkwHDZpCyM1PVlgNpYkYQNUDw9867jxQ=";
              "libsqlite3-sys-0.26.0" = "sha256-JzSGpqYtkIq0mVYD0kERIB6rmZUttqkCGne+M4vqTJU=";
              "octopod-0.1.0" = "sha256-V16fOlIp9BCpyzgh1Aei3Mra/y15v8dQFA8tHdOwZm4=";
              "tonic-0.9.2" = "sha256-8OziOZOyZl1PkpLmfx9zGgtWfGLjgB+w4U/VevHDHcE=";
            };
          };

          cargoBuildFlags = [ "-p" "sqld" ];
          # buildType = "debug";
          doCheck = false;

          nativeBuildInputs = with pkgs; [ pkg-config ];

          buildInputs = with pkgs;
            let
              commonInputs = [ protobuf ];
              linuxInputs = [ openssl ];
              darwinInputs = [ darwin.apple_sdk.frameworks.Security ];
            in
            commonInputs ++ (if stdenv.isDarwin then darwinInputs else linuxInputs);
        };

        packages.sqld-primary = pkgs.writeShellApplication {
          name = "sqld-primary";
          text = ''
            ${packages.sqld}/bin/sqld          \
              --http-listen-addr 0.0.0.0:8080  \
              --grpc-listen-addr 0.0.0.0:5001  \
          '';
        };
      }
    );
}
