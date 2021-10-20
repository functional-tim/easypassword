let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
  with pkgs;
  stdenv.mkDerivation {
    name = "moz_overlay_shell";
    buildInputs = [
      pkgs.latest.rustChannels.stable.cargo
      pkgs.latest.rustChannels.stable.rust
      pkgs.latest.rustChannels.stable.rust-std
    ];
  }
