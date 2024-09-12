{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      flake-utils,
      naersk,
      nixpkgs,
      fenix,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) { inherit system; };

        toolchain =
          with fenix.packages.${system};
          combine [
            complete.rustc
            complete.cargo
            complete.rust-src
            # targets.x86_64-unknown-linux-gnu.latest.rust-std
            targets.aarch64-unknown-linux-gnu.latest.rust-std
          ];

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };

      in
      rec {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;

          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "aarch64-unknown-linux-gnu-cc";
          CARGO_BUILD_TARGET = "aarch64-unknown-linux-gnu";
          SLINT_STYLE = "native";

          copyTarget = true;
          copyBins = true;
          copyLibs = true;
          singleStep = true;

          depsBuildBuild = with pkgs; [ pkgsCross.aarch64-multiplatform.stdenv.cc ];
        };

        # For `nix develop` (optional, can be skipped):
        devShell = pkgs.mkShell {
          CARGO_BUILD_TARGET = "aarch64-unknown-linux-gnu";
          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "aarch64-unknown-linux-gnu-cc";

          nativeBuildInputs = with pkgs; [
            toolchain
            pkgsCross.aarch64-multiplatform.stdenv.cc
          ];
        };
      }
    );
}
