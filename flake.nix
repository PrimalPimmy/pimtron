{
  description = "Pimtron development environment and build";

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

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            trunk
            # Dependencies often needed for Rust crates
            pkg-config
            openssl
            sass
          ];

          shellHook = ''
            export PATH=$PATH:$HOME/.cargo/bin
            if ! command -v stylance &> /dev/null; then
              echo "Installing stylance-cli..."
              cargo install stylance-cli
            fi
          '';
        };
      }
    );
}
