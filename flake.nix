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
            ];
          }
      );

      src = craneLib.cleanCargoSource ./.;

      commonArgs = {
        inherit src;
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
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      individualCrateArgs =
        commonArgs
        // {
          inherit cargoArtifacts;
          inherit (craneLib.crateNameFromCargoToml {inherit src;}) version;
          # NB: we disable tests since we'll run them all via cargo-nextest
          doCheck = false;
        };

      fileSetForCrate = crate:
        lib.fileset.toSource {
          root = ./.;
          fileset = lib.fileset.unions [
            ./Cargo.toml
            ./Cargo.lock
            (craneLib.fileset.commonCargoSources ./common)
            (craneLib.fileset.commonCargoSources crate)
            ./host/migrations
            ./hardware_observer/memory.x
          ];
        };

      host = craneLib.buildPackage (individualCrateArgs
        // {
          pname = "host";
          cargoExtraArgs = "-p host";
          src = fileSetForCrate ./host;

          nativeBuildInputs =
            (commonArgs.nativeBuildInputs or [])
            ++ [
              pkgs.sqlx-cli
            ];

          preBuild = ''
            export DATABASE_URL=sqlite:./db.sqlite3
            sqlx database create
            sqlx migrate run --source host/migrations
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
        ];

        shellHook = ''
          touch secrets.env
          export SYSTEMCTL_PATH=${pkgs.systemd}/bin/systemctl
          export $(grep -v '^#' secrets.env | xargs -d '\n')
        '';
      };
    });
}
