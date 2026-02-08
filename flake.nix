{
  description = "Rust is Win";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      flake-parts,
      rust-overlay,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      imports = [ inputs.git-hooks.flakeModule ];
      perSystem =
        {
          system,
          pkgs,
          config,
          ...
        }:
        let
          rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
          formatter = pkgs.nixfmt;
          devShells.default = pkgs.mkShell {
            shellHook = config.pre-commit.installationScript;
            packages = with pkgs; [
              rust
              trunk-ng
              pinact
            ];
          };
          pre-commit.settings = {
            package = pkgs.prek;
            hooks = {
              nixfmt.enable = true;
              nil.enable = true;
              statix.enable = true;
              flake-checker.enable = true;
              deadnix.enable = true;
              cargo-check.enable = true;
              rustfmt = {
                enable = true;
                packageOverrides = {
                  inherit (pkgs) cargo rustfmt;
                };
              };
              check-toml.enable = true;
              taplo.enable = true;
              actionlint.enable = true;
            };
          };
        };
    };
}
