{ }:

let
  pkgs = import (
    fetchTarball { url = https://github.com/NixOS/nixpkgs/archive/nixos-20.09.tar.gz;}
  ) {};
  containerd_pkg = pkgs.fetchurl {
    name = "containerd";
    url = "https://github.com/containerd/containerd/releases/download/v1.4.3/cri-containerd-cni-1.4.3-linux-amd64.tar.gz";
    sha256 = "0ggz5fl517smd87346gdblgssckyzmcy44w3nhd22z27wd1a75r6";
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
    # Etcd binaries
    cp ${etcd_pkg} "$out/${etcd_pkg.name}.tar.gz"

    # Containerd binaries with
    # unfortunate repacking
    cp ${containerd_pkg} ./cni-containerd.tar.gz
    mkdir -p cni-containerd-unpacked
    mkdir -p containerd

    tar -xvf ./cni-containerd.tar.gz -C ./cni-containerd-unpacked
    cp ./cni-containerd-unpacked/usr/local/bin/* ./containerd
    cp ./cni-containerd-unpacked/usr/local/sbin/* ./containerd
    cp ./cni-containerd-unpacked/opt/cni/bin/* ./containerd

    cd ./containerd
    tar -czvf $out/${containerd_pkg.name}.tar.gz .
    cd ..
  '';
})
