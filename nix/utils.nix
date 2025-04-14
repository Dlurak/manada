{
  mergeAttrs = attrs: builtins.foldl' (acc: x: acc // x) {} attrs;
}
