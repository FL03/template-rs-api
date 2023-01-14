{ pkgs, nixpkgs, system, makeRustPlatform, rust-overlay }:
let
  rustPkgs = import nixpkgs {
    inherit system;
    overlays = [ (import rust-overlay) ];
  };

  rustVersion = "1.68.0";

  rustDefaultTarget = rustPkgs.rust-bin.stable.${rustVersion}.default;

  rustPlatform = makeRustPlatform {
    cargo = rustDefaultTarget;
    rustc = rustDefaultTarget;
  };

  common = {
    version = "0.1.0";
    src = ./.;

    cargoLock = {
      lockFile = ./Cargo.lock;
    };

    nativeBuildInputs = [ pkgs.pkg-config ];
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  };
in {
  workspace = pkgs.rustPlatform.buildRustPackage (common // {
    cargoBuildFlags = "--release --workspace";
  });
}
