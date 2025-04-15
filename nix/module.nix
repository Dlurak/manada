{
  pkgs,
  lib,
  config,
  ...
}: let
  utils = import ./utils.nix;
in {
  options.programs.manada = {
    enable = lib.mkEnableOption "Install and configure manada";
    pkg = lib.mkOption {
      default = import ./package.nix {inherit pkgs;};
      description = "The manada package to use";
    };
    config = lib.mkOption {
      default = let
        mkDefault = unitSet: {
          conversions.extraConfig = builtins.readFile ../conversions/${unitSet};
          tomlFile = ../conversions/${unitSet}.toml;
        };
      in {
        distance = mkDefault "distance";
        temperature = mkDefault "temperature";
        data = mkDefault "data";
      };
      description = "Manada config to be applied for all users";
    };
  };

  config = let
    manada = config.programs.manada;
    unitSets = builtins.attrNames manada.config;
    configBuilder = import ./config.nix {inherit pkgs lib;};
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
      environment.systemPackages = [(manada.pkg)];
      environment.etc = utils.mergeAttrs files;
      nixpkgs.overlays = [(import ./overlay.nix {pkg = manada.pkg;})];
    };
}
