{ }:

let
  pkgs = import (
    fetchTarball { url = https://github.com/NixOS/nixpkgs/archive/nixos-20.09.tar.gz;}
  ) {};
  etcd_pkg = pkgs.fetchurl {
    name = "etcd";
    url = "https://github.com/etcd-io/etcd/releases/download/v3.4.14/etcd-v3.4.14-linux-amd64.tar.gz";
    sha256 = "0l8nc1hhhw81v68bw2chnp92msili6wx1g7hwa27lwpbbka42wgm";
  };
in
pkgs.stdenv.mkDerivation({
  name = "rusty-sailor-vendored-packages";
  src = ./Cargo.lock;
  phases = [ "installPhase" ];
  installPhase = ''
    mkdir -p $out
    cp ${etcd_pkg} "$out/${etcd_pkg.name}.tar.gz"
  '';
})
