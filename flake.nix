{
  description = "not-grocy-server's development flake";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let pkgs = nixpkgs.legacyPackages.${system}; in
        {
          devShell = (pkgs.mkShell.override { stdenv = pkgs.llvmPackages_13.stdenv; }) {
            nativeBuildInputs = [
              pkgs.bashInteractive # fix nested shells
              pkgs.llvmPackages_13.bintools
              pkgs.ghidra-bin
            ];

            buildInputs = [
              
            ];
          };
        }
      );
}