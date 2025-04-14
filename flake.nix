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

        devShell = with pkgs;
          mkShell {
            buildInputs = [
              cargo
              rustc
              rustfmt
              pre-commit
              rustPackages.clippy
              rust-analyzer
              bacon
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
            MANADA_CONFIG = "./conversions";
          };
        nixosModules.default = {
          pkgs,
          lib,
          config,
          ...
        }: {
          options.programs.manada = {
            enable = lib.mkEnableOption "Install and configure manada";
            config = lib.mkOption {
              default = {
                distance = {
                  conversions.extraConfig = builtins.readFile ./conversions/distance;
				  tomlFile = ./conversions/distance.toml;
                };
                temperature = {
                  conversions.extraConfig = builtins.readFile ./conversions/temperature;
				  tomlFile = ./conversions/temperature.toml;
                };
              };
              description = "Manada config to be applied for all users";
            };
          };

          config = let
            manada = config.programs.manada;
            unitSets = builtins.attrNames manada.config;
            configBuilder = import ./nix/config.nix {inherit pkgs lib;};
            genUnitSetFiles = unitSet: let
              unitConfig = manada.config.${unitSet};
              configured = configBuilder {
                inherit unitSet;
                conversions = unitConfig.conversions;
                aliases =
                  if unitConfig ? aliases
                  then unitConfig.aliases
                  else {};
                tomlFile =
                  if unitConfig ? tomlFile
                  then unitConfig.tomlFile
                  else null;
              };
            in {
              "manada/${unitSet}.toml".source = configured.toml;
              "manada/${unitSet}".text = configured.conversions;
            };
            files = builtins.map genUnitSetFiles unitSets;
          in
            lib.mkIf manada.enable {
              environment.systemPackages = [(import ./nix/package.nix {inherit pkgs;})];
              environment.etc = utils.mergeAttrs files;
            };
        };
      }
    );
}
