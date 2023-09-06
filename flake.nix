{
  description = "rust shell dev";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell rec {
          nativeBuildInputs = [
            makeWrapper
            pkg-config
          ];
          buildInputs = [
            openssl
            just
            cargo-watch
            cargo-release
            sqlx-cli
          ];
          env = {
            ZSTD_SYS_USE_PKG_CONFIG = true;
          };
          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
          shellHook = ''
            alias run="cargo run"
            alias ls=exa
            alias find=fd
            alias build="cargo build"
            alias web="cargo run --release --target wasm32-unknown-unknown"
          '';
        };
      }
    );
}
