{
  description = "homelab discord bot";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
    };

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };
      inherit (pkgs) lib;

      craneLib = (crane.mkLib pkgs).overrideToolchain (
        p:
          p.rust-bin.stable.latest.default.override {
            targets = [
              "thumbv6m-none-eabi"
              "x86_64-unknown-linux-gnu"
              "wasm32-unknown-unknown"
            ];
          }
      );

      unfilteredRoot = ./.;
      src = lib.fileset.toSource {
        root = unfilteredRoot;
        fileset =
          lib.fileset.difference (lib.fileset.unions [
            (craneLib.fileset.commonCargoSources unfilteredRoot)
            ./host/migrations
          ])
          ./host/.cargo/config.toml;
      };
      commonArgs = {
        inherit src;
        cargoLock = ./host/Cargo.lock;
        cargoToml = ./host/Cargo.toml;
        strictDeps = true;

        nativeBuildInputs = [
          pkgs.pkg-config
        ];

        buildInputs =
          [
            pkgs.openssl
          ]
          ++ lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
            pkgs.darwin.apple_sdk.frameworks.Security
          ];
        postUnpack = ''
          cd $sourceRoot/host
          sourceRoot="."
        '';
      };

      host = craneLib.buildPackage (commonArgs
        // {
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          nativeBuildInputs =
            (commonArgs.nativeBuildInputs or [])
            ++ [
              pkgs.sqlx-cli
            ];

          preBuild = ''
            export DATABASE_URL=sqlite:$(pwd)/db.sqlite3
            sqlx database create
            sqlx migrate run
          '';
        });
    in {
      checks = {
        inherit host;
      };

      packages = {
        inherit host;
      };

      devShells.default = craneLib.devShell {
        checks = self.checks.${system};

        packages = with pkgs; [
          sqlx-cli
          bacon
          probe-rs-tools
          usbutils
          elf2uf2-rs
          picotool
          screen
          dioxus-cli
          tailwindcss
          concurrently
        ];

        shellHook = ''
          touch secrets.env
          export SYSTEMCTL_PATH=${pkgs.systemd}/bin/systemctl
          set -a
          source secrets.env
          set +a
        '';
      };
    });
}
