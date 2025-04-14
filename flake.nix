{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
  }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        utils = import ./nix/utils.nix;
      in {
        defaultPackage = import ./nix/package.nix {inherit pkgs;};
        devShell = import ./nix/devshell.nix {inherit pkgs;};
		# TODO: Write a home manager module
        nixosModules.default = import ./nix/module.nix;
        overlays.default = import ./nix/overlay.nix {inherit pkgs;};
      }
    );
}
