
{ pkgs ? import <nixpkgs> {} }:

let
  moz_overlay = import (builtins.fetchTarball 
    https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);

  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
  with nixpkgs;

  pkgs.mkShell {
    buildInputs = [
      nixpkgs.latest.rustChannels.stable.rust

      wineWowPackages.staging
      winetricks

      mgba
      _0x
    ];

    WINEDEBUG="fixme-all";
  }
