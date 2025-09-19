{
  description = "Rust development environment for Gensokio";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixos-unstable";
    nixgl.url = "github:guibou/nixGL";
  };

  outputs = {
    self,
    nixpkgs-unstable,
    utils,
    nixgl,
    ...
  }:
    utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = import nixpkgs-unstable {
          inherit system;
          overlays = [nixgl.overlay];
        };
      in rec
      {
        # Executed by `nix build`
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "gensokio";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };

        # Executed by `nix run`
        apps.default = utils.lib.mkApp {drv = packages.default;};

        # Used by `nix develop`
        devShells.default = pkgs.mkShell rec {
          shellHook = ''export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
            pkgs.vulkan-loader
            pkgs.udev
            pkgs.alsa-lib
            pkgs.libxkbcommon
            pkgs.wayland

            pkgs.xorg.libX11 pkgs.xorg.libXcursor pkgs.xorg.libXi pkgs.xorg.libXrandr
          ]}"'';
          buildInputs = with pkgs; [
            xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
            libxkbcommon wayland # To use the wayland feature
            udev alsa-lib vulkan-loader

            clang pkg-config
          ];
        };
      }
    );
}
