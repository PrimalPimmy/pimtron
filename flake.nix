{
  description = "Pimtron development environment and build";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    
    # We define the stylance source here. Nix will lock its exact Git hash in flake.lock.
    stylance-rs-src = {
      url = "github:basro/stylance-rs/v0.7.4";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, stylance-rs-src, ... }:
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

        # Helper for reproducible Rust builds
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
        
        # --- Custom Stylance CLI Package ---
        stylanceCliPackage = rustPlatform.buildRustPackage rec {
          pname = "stylance-cli";
          version = "0.7.4";

          src = stylance-rs-src;

          # FIX 1: Point to the workspace root lockfile
          cargoLock.lockFile = "${src}/Cargo.lock";

          buildAndTestSubdir = "stylance-cli";
        };

        # The Leptos site build derivation
        leptosSitePackage = pkgs.stdenv.mkDerivation {
          pname = "pimtron-site";
          version = "0.1.0";

          # Source: Use the entire current directory as the source for the build
          src = self;

          nativeBuildInputs = with pkgs; [
            rustToolchain
            trunk
            pkg-config
            openssl
            stylanceCliPackage 
          ];

          buildPhase = ''
          echo "Setting CARGO_HOME and WASM_BINDGEN_CACHE to temporary, writable directories..."
          
          export CARGO_HOME=$TMPDIR/cargo_home
          mkdir -p $CARGO_HOME

          # cache location for wasm-bindgen to a writable directory
          export WASM_BINDGEN_CACHE=$TMPDIR/wasm_bindgen_cache
          mkdir -p $WASM_BINDGEN_CACHE

          echo "Running trunk build..."
          trunk build --release
          '';

          # move the final artifacts to the Nix output ($out)
          installPhase = ''
            echo "Copying built site from dist to $out..."
            cp -r dist/* $out
          '';
        };

      in
      {
        # --- 1. Define the actual deployable package ---
        packages = {
          pimtron-site = leptosSitePackage;
          default = leptosSitePackage; # Allow running 'nix build' without arguments
        };

        # --- 2. Keep the Dev Shell for local development ---
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            trunk
            pkg-config
            openssl
            sass
            # Also make the custom package available in the dev shell
            stylanceCliPackage
          ];
        };
      }
    );
}