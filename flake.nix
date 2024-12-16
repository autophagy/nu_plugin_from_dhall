{
  description = "nu_plugin_from_dhall - A plugin for Nushell to parse Dhall files";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nix-community/naersk/master";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      naersk,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      rec {
        # `nix build`
        packages = {
          ansine = naersk-lib.buildPackage {
            root = ./.;
            doCheck = true;
            buildInputs = with pkgs; [
              pkg-config
              openssl
            ];
          };
          default = packages.ansine;
        };

        devShell =
          with pkgs;
          mkShell {
            buildInputs = [
              cargo
              rustc
              rustfmt
              rustPackages.clippy
              openssl
              pkg-config
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };

        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
