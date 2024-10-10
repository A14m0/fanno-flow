{
  description = "Devshell for Rust";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
      devShells.default = with pkgs; mkShell.override{
          stdenv = pkgs.clangStdenv;
      } {
          
          LIBCLANG_PATH="${llvmPackages.libclang.lib}/lib";
          INCLUDES_PATH="${llvmPackages.libclang.lib}/includes";

          buildInputs = [
            pkg-config
            eza
            fd
            rust-bin.beta.latest.default
            pipewire
            libclang
          ];

          shellHook = ''
            alias ls=eza
            alias find=fd
          '';
        };
      }
    );
}

