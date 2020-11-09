{ }:

let
  pkgs = import (
    fetchTarball { url = https://github.com/NixOS/nixpkgs/archive/nixos-20.09.tar.gz;}
  ) {};
  etcd_pkg = pkgs.fetchurl {
    name = "etcd";
    url = "https://github.com/etcd-io/etcd/releases/download/v3.4.13/etcd-v3.4.13-linux-amd64.tar.gz";
    sha256 = "2ac029e47bab752dacdb7b30032f230f49e2f457cbc32e8f555c2210bb5ff107";
  };
in
pkgs.stdenv.mkDerivation({
  name = "rusty-sailor-vendored-packages";
  src = ./Cargo.lock;
  phases = [ "installPhase" ];
  installPhase = ''
    mkdir -p $out
    cp ${etcd_pkg} $out/${etcd_pkg.name}
  '';
})
