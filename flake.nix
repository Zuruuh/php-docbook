{
  description = "";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, flake-utils, fenix, ... }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
      };
      toolchain = fenix.packages.${system}.fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-Hn2uaQzRLidAWpfmRwSRdImifGUCAb9HeAqTYFXWeQk=";
      };
    in
    {
      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          toolchain
          pkg-config
          libiconvReal
          libxml2
        ];
      };
    });
}
