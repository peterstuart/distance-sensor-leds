let
  overlay = import (builtins.fetchTarball
    "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ overlay ]; };
  rust-bin = pkgs.rust-bin.stable.latest.minimal.override {
    extensions = [ "clippy" "rustfmt" "llvm-tools-preview" ];
    targets = [ "thumbv7em-none-eabihf" ];
  };
in pkgs.mkShell {
  buildInputs = [
    rust-bin

    pkgs.cargo-binutils
    pkgs.cargo-edit
    pkgs.cargo-generate
    # pkgs.gcc-arm-embedded
    pkgs.minicom
    pkgs.openocd
    pkgs.pkg-config
    pkgs.rust-analyzer
  ];

  shellHook = "";
}
