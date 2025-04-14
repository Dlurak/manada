{pkgs}:
pkgs.rustPlatform.buildRustPackage {
  pname = "manada";
  version = "0.1.0";
  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;
}
