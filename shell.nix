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

    rustup component add clippy

    export LD_LIBRARY_PATH=''$(rustc --print sysroot)/lib
    export RUST_BACKTRACE=full
    export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib";
    export OPENSSL_INCLUDE_DIR="${pkgs.openssl.dev}/include";
  '';
}