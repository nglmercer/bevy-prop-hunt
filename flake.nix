{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = {
    nixpkgs,
    fenix,
    ...
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};
    toolchain = fenix.packages.${system}.stable.toolchain;
  in {
    devShells.${system}.default = pkgs.mkShell rec {
      packages = with pkgs; [
        toolchain
        pkg-config

        clang
        lld

        stdenv.cc.cc.lib
        alsa-lib
        udev
        libxkbcommon
        libxkbcommon.dev
        wayland
        wayland-protocols
        libX11
        libXcursor
        libXrandr
        libXi
        vulkan-loader
      ];

      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath packages;
    };
  };
}
