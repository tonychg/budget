{
  description = "Budget Manager";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        inherit (pkgs) lib;
        craneLib = crane.lib.${system};
        bdg = craneLib.buildPackage {
          src = lib.cleanSourceWith {
            src = craneLib.path ./.;
          };
          strictDeps = true;
          nativeBuildInputs = [
            pkgs.pkg-config
          ];
          buildInputs = with pkgs; [
            openssl
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
          ];
        };
      in
      with pkgs;
      {
        checks = {
          inherit bdg;
        };
        packages = {
          default = bdg;
        };
        apps = {
          default = {
            type = "app";
            program = "${self.packages.${system}.default}/bin/bdg";
          };
        };
        devShells.default = craneLib.devShell {
          checks = self.checks.${system};

          packages = [
            rust-analyzer
            git
            go-task
            sops
          ];
        };
      });
}
