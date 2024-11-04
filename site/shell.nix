let
  # Pinned nixpkgs, deterministic. Last updated: 2/12/21.
  # pkgs = import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/a58a0b5098f0c2a389ee70eb69422a052982d990.tar.gz")) {};

  # Rolling updates, not deterministic.
  pkgs = import (fetchTarball("channel:nixpkgs-unstable")) {};
in pkgs.mkShell {
  buildInputs = [ pkgs.lld pkgs.cargo pkgs.rustc pkgs.openssl pkgs.pkg-config pkgs.trunk ];
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}