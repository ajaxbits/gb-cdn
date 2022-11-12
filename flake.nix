{
  description = "Grace Bobber CDN";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rust-env = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src"];
        };
      in rec
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            openssl
            pkgconfig
            (docker.override (args: {buildxSupport = true;}))

            rust-env

            exiftool

            flyctl
          ];
        };
      }
    );
}
