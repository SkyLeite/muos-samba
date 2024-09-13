{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/7a339d87931bba829f68e94621536cad9132971a";
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

        pkgsArm = import nixpkgs {
          config = { };
          overlays = [ ];
          system = "aarch64-linux";
        };

        pkgsCrosss = import nixpkgs {
          inherit system;
          overlays = [
            (self: super: {

              # we want to hack on SDL, don't want to hack on those. Some even don't cross-compile
              inherit (pkgsArm)
                xorg
                libpulseaudio
                libGL
                guile
                systemd
                libxkbcommon
                cairo
                pipewire
                libdecor
                mesa
                libclang
                ;

            })
          ];
          crossSystem = {
            config = "aarch64-unknown-linux-gnu";
          };
        };

        toolchain =
          with fenix.packages.${system};
          combine [
            complete.rustc
            complete.cargo
            complete.rust-src
            complete.rustfmt
            complete.rust-analyzer
            # targets.x86_64-unknown-linux-gnu.latest.rust-std
            targets.aarch64-unknown-linux-gnu.latest.rust-std
          ];

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };

        SDL2Arm = pkgsCrosss.SDL2.override {
          drmSupport = true;
          x11Support = false;
          waylandSupport = false;
          libdecorSupport = false;
          pipewireSupport = false;
          pulseaudioSupport = false;

          inherit (pkgsArm.buildPackages) wayland wayland-protocols;
        };

        packageConfig = isRelease: {
          src = ./.;

          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER =
            if isRelease then "aarch64-unknown-linux-gnu-cc" else null;

          CARGO_BUILD_TARGET = if isRelease then "aarch64-unknown-linux-gnu" else null;

          copyTarget = true;
          copyBins = true;
          copyLibs = true;

          nativeBuildInputs = with pkgs; [ pkg-config ];

          depsBuildBuild = with pkgs; [
            pkgs.cmake
            (if isRelease then SDL2Arm else SDL2)
            (if isRelease then (pkgsCrosss.SDL2_gfx.override { SDL2 = SDL2Arm; }) else SDL2_gfx)
            (if isRelease then pkgsCrosss.stdenv.cc else stdenv.cc)
            wayland
            wayland-scanner
            xorg.libXcursor
            xorg.libXrandr
            xorg.libX11
            xorg.libXi
            xorg.libXScrnSaver
          ];

          postInstall =
            if isRelease then
              ''
                patchelf --set-interpreter /lib/ld-linux-aarch64.so.1 $out/bin/muos-samba
              ''
            else
              null;
        };

        package = isRelease: naersk'.buildPackage (packageConfig isRelease);

        darwinPackages =
          if pkgs.stdenv.isDarwin then
            with pkgs;
            [
              darwin.apple_sdk.frameworks.Carbon
              darwin.apple_sdk.frameworks.Cocoa
              darwin.apple_sdk.frameworks.ScriptingBridge
              darwin.apple_sdk.frameworks.ForceFeedback
              darwin.apple_sdk.frameworks.GameController
              darwin.apple_sdk.frameworks.CoreHaptics
              iconv
            ]
          else
            [ ];
      in
      rec {
        # For `nix build` & `nix run`:
        defaultPackage = package false;

        packages.release = package true;

        # For `nix develop` (optional, can be skipped):
        devShell = pkgs.mkShell rec {
          nativeBuildInputs =
            with pkgs;
            [
              pkg-config
              toolchain
              cmake
              xorg.libXcursor
              xorg.libXrandr
              xorg.libX11
              xorg.libXi
              xorg.libXScrnSaver
              SDL2
              SDL2_gfx
            ]
            ++ darwinPackages;

          shellHook = ''
            LD_LIBRARY_PATH="''${LD_LIBRARY_PATH:+$LD_LIBRARY_PATH:}${pkgs.lib.makeLibraryPath nativeBuildInputs}"
            export LD_LIBRARY_PATH
          '';
        };
      }
    );
}
