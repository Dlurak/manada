{pkgs}:
with pkgs;
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
  }
