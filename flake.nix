{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in with pkgs; rec {
        devShell = pkgs.mkShell {
  packages = with pkgs; [
    rustup
    cmake
    clang
    pkg-config
    wayland
    glfw
  ];
  LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
    libGL
    xorg.libXrandr
    xorg.libXinerama
    xorg.libXcursor
    xorg.libXi
  ];
  LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
};
      });
}