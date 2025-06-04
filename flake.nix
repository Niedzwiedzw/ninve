{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
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
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        buildInputs = with pkgs; [
          openssl
          pkg-config
          rustToolchain
          mpv
          ffmpeg
        ];
        pkgConfigPath = pkgs.lib.makeSearchPathOutput "dev" "lib/pkgconfig" buildInputs;
        ldLibraryPath = pkgs.lib.makeLibraryPath buildInputs;
      in {
        devShells.default = with pkgs;
          mkShell {
            inherit buildInputs;
            PKG_CONFIG_PATH = pkgConfigPath;
            LD_LIBRARY_PATH = ldLibraryPath;
          };
      }
    );
}
