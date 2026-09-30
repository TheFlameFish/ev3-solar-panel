{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      perSystem = { config, self, pkgs, lib, system, ... }:
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
          ];
        };
      };
    };
}