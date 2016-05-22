let
  pkgs = import <nixpkgs> {};
  stdenv = pkgs.stdenv;

in stdenv.mkDerivation rec {
  name = "uswitch-puppet";
  buildInputs = [
    pkgs.cargo
    pkgs.rustc
  ];
}

