{
  pkgs,
  lib,
}: {
  unitSet,
  conversions,
  aliases,
  tomlFile ? null,
}: let
  _ = (tomlFile != null && aliases != {}) && (abort "If tomlFile is set, aliases can't be set");
  tomlFormat = pkgs.formats.toml {};
  originKeys = conversions: builtins.attrNames (removeAttrs conversions ["extraConfig"]);
  lines = origin: let
    subset = conversions.${origin};
  in
    map (dest: "${origin} -> ${dest}: ${subset.${dest}}") (builtins.attrNames subset);
in {
  toml =
    if tomlFile == null
    then tomlFormat.generate "${unitSet}.toml" aliases
    else tomlFile;
  conversions =
    builtins.concatStringsSep "\n\n" (
      map
      (origin: lib.concatLines (lines origin))
      (originKeys conversions)
    )
    + (
      if conversions ? extraConfig
      then "\n${conversions.extraConfig}"
      else ""
    );
}
