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
      default = {};
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
      ".config/manada/${unitSet}.toml".source = configured.toml;
      ".config/manada/${unitSet}".text = configured.conversions;
    };
    files = map genUnitSetFiles unitSets;
  in
    lib.mkIf manada.enable {
      home.packages = [(manada.pkg)];
      home.file = utils.mergeAttrs files;
    };
}
