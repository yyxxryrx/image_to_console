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
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystem = nixpkgs.lib.genAttrs supportedSystem;
      version = "0.2.0";
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
            version = version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            buildAndTestSubdir = "image-to-console";
          };

          full = rustPlatform.buildRustPackage {
            pname = "image_to_console";
            version = version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            buildInputs = with pkgs; [
              ffmpeg
              alsa-lib
            ];

            nativeBuildInputs = with pkgs; [
              rustPlatform.bindgenHook
              pkg-config
            ];
            buildAndTestSubdir = "image-to-console";
            cargoBuildFlags = "--all-features";
          };

          lite = rustPlatform.buildRustPackage {
            pname = "image_to_console";
            version = version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            buildAndTestSubdir = "image-to-console";
            cargoBuildFlags = "--no-default-features";
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
        in
        {
          default = pkgs.mkShell {
            inputsFrom = [ self.packages.${system}.default ];
            packages = with pkgs; [
              rust-analyzer
              clippy
            ];
          };
        }
      );
    };
}
