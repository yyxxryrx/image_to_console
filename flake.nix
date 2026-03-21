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
      system = "x86_64-linux"; # 或 "aarch64-linux"
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
      packages.${system}.default = rustPlatform.buildRustPackage {
        pname = "your-project";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;

        nativeBuildInputs = with pkgs; [
          rustPlatform.bindgenHook # ← 核心配置
          pkg-config
        ];
      };

      devShells.${system}.default = pkgs.mkShell {
        inputsFrom = [ self.packages.${system}.default ];
        buildInputs = with pkgs; [
          rust-bin.stable.latest.rust-analyzer
          rust-bin.stable.latest.clippy
        ];
      };
    };
}
