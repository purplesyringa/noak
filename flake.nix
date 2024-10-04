{
  description = "A java bytecode decoding and encoding library";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        craneLib = crane.mkLib pkgs;
        src = craneLib.cleanCargoSource ./.;

        # Common arguments can be set here to avoid repeating them later
        commonArgs = {
          inherit src;
          strictDeps = true;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        noak = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            doCheck = true;
          }
        );
      in
      {
        checks = {
          inherit noak;

          noakFmt = craneLib.cargoFmt {
            inherit src;
          };

          noakClippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
            }
          );

          noakTest = craneLib.cargoTest (
            commonArgs
            // {
              inherit cargoArtifacts;
            }
          );
        };

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};
        };
      }
    );
}
