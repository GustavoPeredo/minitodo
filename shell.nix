{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
    nativeBuildInputs = [ pkgs.rustc pkgs.cargo ];
}