{
  description = "Development shell for cross-compiling to 32-bit Windows MSVC";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      crane,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rust-toolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [ ];
        };

        buildTools = with pkgs; [
          rust-toolchain
          libllvm
          clang
          lld
        ];
      in
      {
        nixpkgs.config.allowUnfree = true;

        devShells.default = pkgs.mkShell {
          buildInputs = buildTools;
        };
      }
    );
}
