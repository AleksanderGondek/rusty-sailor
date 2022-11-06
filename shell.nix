{ }:

let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs {};
  vendored_packages = import ./vendored.nix {};
in
pkgs.mkShell {
  name = "rusty-sailor-shell";

  buildInputs = with pkgs; [
    bash
    busybox
    cacert
    rustup
    # crate openssl-endored deps
    gcc
    gnumake
    perl
    # VM-based testing
    curlFull
    libvirt
    openssh
    rsync
    vagrant
  ];

  shellHook = ''
    export RUSTUP_HOME=$(pwd)/.rustup

    rustup install stable
    rustup default stable

    rustup component add clippy

    # Ensuring OPENSSL behaves
    export LD_LIBRARY_PATH=''$(rustc --print sysroot)/lib
    export RUST_BACKTRACE=full

    # Ensuring Vendored Packages exist
    if [ ! -d "vendored" ]; then
      cp -R "${vendored_packages}" ./vendored
    fi
  '';
}
