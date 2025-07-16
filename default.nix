let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
in
  { pkgs ? import <nixpkgs> { overlays = [ moz_overlay ]; } }:

  let cargoNix = import ./Cargo.nix { inherit pkgs; };
  in cargoNix.rootCrate.build
