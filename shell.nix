{ }:

let
  pkgs = import (
    fetchTarball { url = https://github.com/NixOS/nixpkgs/archive/nixos-20.09.tar.gz;}
  ) {};
  vendored_packages = import ./vendored.nix {};
in
pkgs.mkShell {
  name = "rusty-sailor-shell";

  buildInputs = with pkgs; [
    bash
    busybox
    cacert
    openssl
    rustup
  ];

  shellHook = ''
    export RUSTUP_HOME=$(pwd)/.rustup

    rustup install stable
    rustup default stable

    rustup component add clippy

    # Ensuring OPENSSL behaves
    export LD_LIBRARY_PATH=''$(rustc --print sysroot)/lib
    export RUST_BACKTRACE=full
    export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib";
    export OPENSSL_INCLUDE_DIR="${pkgs.openssl.dev}/include";

    # Ensuring Vendored Packages exist
    if [ ! -d "vendored" ]; then
      cp -R "${vendored_packages}" ./vendored
    fi
  '';
}
