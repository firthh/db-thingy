let
  pkgs = import <nixpkgs> {};
  stdenv = pkgs.stdenv;

in stdenv.mkDerivation rec {
  name = "uswitch-puppet";
  buildInputs = [
    pkgs.opam
    pkgs.ocaml
  ];


  shellHook = ''

    if [ ! -d ".opam" ]; then
      opam init --root "$(pwd)/.opam" --dot-profile=/dev/null
    fi

    . /home/hugo.firth/code/db-thingy/ocaml/.opam/opam-init/init.sh > /dev/null 2> /dev/null || true
  '';
}

