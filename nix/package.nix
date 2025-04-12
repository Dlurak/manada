{pkgs}:
pkgs.rustPlatform.buildRustPackage {
  pname = "manada";
  version = "0.1.0";
  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;

  postInstall = ''
    mkdir -p $out/etc/
	cp -r $src/conversions $out/etc/manada
  '';
}
