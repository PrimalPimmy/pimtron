{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;

        rustToolchainFor =
          p:
          p.rust-bin.stable.latest.default.override {
            # Set the build targets supported by the toolchain,
            # wasm32-unknown-unknown is required for trunk
            targets = [ "wasm32-unknown-unknown" ];
          };
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchainFor;

        # When filtering sources, we want to allow assets other than .rs files
        unfilteredRoot = ./.; # The original, unfiltered source
        src = lib.fileset.toSource {
          root = unfilteredRoot;
          fileset = lib.fileset.unions [
            # Default files from crane (Rust and cargo files)
            (craneLib.fileset.commonCargoSources unfilteredRoot)
            (lib.fileset.fileFilter (
              file:
              lib.any file.hasExt [
                "html"
                "scss"
                "css"
                "md"
                "toml"
                "tmTheme"
              ]
            ) unfilteredRoot)
            (lib.fileset.maybeMissing ./assets)
            (lib.fileset.maybeMissing ./posts)
            (lib.fileset.maybeMissing ./public)
            (lib.fileset.maybeMissing ./_redirects)
            (lib.fileset.maybeMissing ./_headers)
          ];
        };

        # Separate source for dependency building to avoid rebuilding deps when assets change
        cargoSrc = lib.fileset.toSource {
            root = unfilteredRoot;
            fileset = craneLib.fileset.commonCargoSources unfilteredRoot;
        };

        stylance-cli = pkgs.rustPlatform.buildRustPackage rec {
          pname = "stylance-cli";
          version = "0.7.4";

          src = pkgs.fetchCrate {
            inherit pname version;
            hash = "sha256-lGgKmNqZ0nflVAM3GMDwGgxnXyLCqVz1bTUsvabXmj8=";
          };

          cargoHash = "sha256-HWZQNEKTyNnmA1twN5cfo5RY2tOeCnL6zEp+M4F+Tqg=";
        };

        # Common arguments can be set here to avoid repeating them later
        commonArgs = {
          inherit src;
          strictDeps = true;
          # We must force the target, otherwise cargo will attempt to use your native target
          # CARGO_BUILD_TARGET = "wasm32-unknown-unknown";


          nativeBuildInputs = [
            stylance-cli
          ];

          buildInputs = [
            # Add additional build inputs here
          ]
          ++ lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
          ];
        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly (
          commonArgs
          // {
            src = cargoSrc;
            # You cannot run cargo test on a wasm build
            doCheck = false;
          }
        );

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        # This derivation is a directory you can put on a webserver.
        my-app = craneLib.buildTrunkPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            # The version of wasm-bindgen-cli here must match the one from Cargo.lock.
            wasm-bindgen-cli = pkgs.buildWasmBindgenCli rec {
              src = pkgs.fetchCrate {
                pname = "wasm-bindgen-cli";
                version = "0.2.106";
                hash = "sha256-M6WuGl7EruNopHZbqBpucu4RWz44/MSdv6f0zkYw+44";
              };

              cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
                inherit src;
                inherit (src) pname version;
                hash = "sha256-ElDatyOwdKwHg3bNH/1pcxKI7LXkhsotlDPQjiLHBwA=";
              };
            };
          }
        );


      in
      {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit my-app;

          # Run clippy (and deny all warnings) on the crate source,
          # again, reusing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          my-app-clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            }
          );

          # # Check formatting
          # my-app-fmt = craneLib.cargoFmt {
          #   inherit src;
          # };
        };

        packages.default = my-app;

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};

          # Additional dev-shell environment variables can be set directly
          # MY_CUSTOM_DEVELOPMENT_VAR = "something else";

          # Extra inputs can be added here; cargo and rustc are provided by default.
          packages = [
            pkgs.trunk
          ];
        };
      }
    );
}
