{
  description = "Pimtron development environment and build";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    
    # --- NEW INPUT FOR STYLANCE-RS ---
    # We define the stylance source here. Nix will lock its exact Git hash in flake.lock.
    stylance-rs-src = {
      url = "github:basro/stylance-rs/v0.7.4";
      # CRITICAL FIX: Tell Nix this input is NOT a flake, just source code.
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

          # Build Tools: trunk, rust, and the custom stylance-cli package
          nativeBuildInputs = with pkgs; [
            rustToolchain
            trunk
            pkg-config
            openssl
            # Now using the locally defined package
            stylanceCliPackage 
          ];

          # This is the phase where trunk performs the build.
          # Trunk builds into the 'dist' directory by default.
          buildPhase = ''
            # 1. Run stylance-cli FIRST to generate all CSS/styles
            # The executable is now available on the PATH from nativeBuildInputs
            echo "Running stylance-cli..."
            stylance-cli
            
            # 2. Run trunk build which calculates SRI hashes based on the files 
            # created by the stylance-cli output and bundles the app into the 'dist' folder.
            echo "Running trunk build..."
            trunk build --release
          '';

          # This is the phase where we move the final artifacts to the Nix output ($out)
          installPhase = ''
            echo "Copying built site from dist to $out..."
            # Copy all contents of the 'dist' folder (Trunk's output) to the $out directory.
            # This $out path is the single, clean artifact ready for deployment.
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