{ pkgs ? import <nixpkgs> { config.allowUnfree = true; } }:
  pkgs.mkShell {
    nativeBuildInputs = [ 
      pkgs.buildPackages.pkg-config
      pkgs.buildPackages.openssl
      pkgs.buildPackages.gcc
      pkgs.buildPackages.bintools
      pkgs.buildPackages.protobuf
    ];
}
