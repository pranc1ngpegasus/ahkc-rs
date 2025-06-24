{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    fenix.url = "github:nix-community/fenix/monthly";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    pre-commit-hooks.url = "github:cachix/git-hooks.nix";
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    flake-utils,
    pre-commit-hooks,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        toolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
        };
        checks = pre-commit-hooks.lib.${system}.run {
          src = ./.;
          hooks = {
            alejandra = {
              enable = true;
              entry = "alejandra .";
              files = "^.*\\.nix$";
              language = "system";
              pass_filenames = false;
            };
            clippy = {
              enable = true;
              files = "^.*\\.rs";
              pass_filenames = false;
              packageOverrides.cargo = toolchain;
              packageOverrides.clippy = toolchain;
              settings.allFeatures = true;
            };
            rustfmt = {
              enable = true;
              files = "^.*\\.rs";
              pass_filenames = false;
              packageOverrides.cargo = toolchain;
            };
            taplo = {
              enable = true;
              entry = "taplo lint .";
              files = "^.*\\.toml";
              language = "system";
              pass_filenames = false;
            };
            typos = {
              enable = true;
              entry = "typos .";
              language = "system";
              pass_filenames = false;
            };
          };
        };
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo-nextest
            rustup
            sccache
            taplo
            typos
          ];

          env = {};

          nativeBuildInputs = [toolchain];

          shellHook = ''
            ${checks.shellHook}
            export RUSTC_WRAPPER=${pkgs.sccache}/bin/sccache
          '';
        };
      }
    );
}
