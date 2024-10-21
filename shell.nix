let
  pkgs = import <nixpkgs> { };
in with pkgs;
mkShell {
  name = "nannou";
  nativeBuildInputs = with pkgs; [
    pkg-config
    clang
    lld
  ];
  buildInputs = with pkgs; [
    wayland
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    vulkan-loader
    udev
    cmake
    gcc
    cargo
    rustc
  ];
  shellHook = ''
              export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
                pkgs.lib.makeLibraryPath [
                  udev
                  vulkan-loader
                ]
              }"'';
  RUST_SRC_PATH = rustPlatform.rustLibSrc;
}

