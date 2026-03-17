{
  description = "Rust project with bindgen";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      supportedSystem = [
        "x84_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystem = nixpkgs.lib.genAttrs supportedSystem;
    in
    {
      packages = forAllSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
          rustPlatform = pkgs.makeRustPlatform {
            rustc = pkgs.rust-bin.stable.latest.default;
            cargo = pkgs.rust-bin.stable.latest.default;
          };
        in
        {
          default = rustPlatform.buildRustPackage {
            pname = "image_to_console";
            version = "0.1.18";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            buildInputs = with pkgs; [
              ffmpeg
              openssl
              alsa-lib
            ];

            nativeBuildInputs = with pkgs; [
              rustPlatform.bindgenHook # ← 核心配置
              pkg-config
            ];

            cargoBuildFlags = "--bin image_to_console --all-features";
          };
        }
      );

      devShells = forAllSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
          rustPlatform = pkgs.makeRustPlatform {
            rustc = pkgs.rust-bin.stable.latest.default;
            cargo = pkgs.rust-bin.stable.latest.default;
          };
        in
        {
          default = pkgs.mkShell {
            inputsFrom = [ self.packages.${system}.default ];
            buildInputs = with pkgs.rust-bin.stable; [
              rust-analyzer
              clippy
            ];
          };
        }
      );
    };
}
