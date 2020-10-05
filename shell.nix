{ }:

let
  pkgs = import (
    fetchTarball { url = https://github.com/NixOS/nixpkgs/archive/nixos-20.09.tar.gz;}
  ) {};
in
pkgs.mkShell {
  name = "rusty-sailor-shell";

  buildInputs = with pkgs; [
    bash
    busybox
    cacert
    rustup
  ];

  shellHook = ''
    export RUSTUP_HOME=$(pwd)/.rustup

    rustup install stable
    rustup default stable
  '';
}