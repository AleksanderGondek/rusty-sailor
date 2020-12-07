{ }:

let
  pkgs = import (
    fetchTarball { url = https://github.com/NixOS/nixpkgs/archive/nixos-20.09.tar.gz;}
  ) {};
  containerd_pkg = pkgs.fetchurl {
    name = "containerd";
    url = "https://github.com/containerd/containerd/releases/download/v1.4.3/containerd-1.4.3-linux-amd64.tar.gz";
    sha256 = "1wa2apxcknnj28hffbwj274camhsxfg366hl0dxk7zarykin389l";
  };
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
    cp ${containerd_pkg} "$out/${containerd_pkg.name}.tar.gz"
  '';
})
