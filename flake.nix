{
  inputs = {
    naersk.url = "github:nix-community/naersk";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = (import nixpkgs) {
          inherit system overlays;
        };
        naersk' = pkgs.callPackage naersk { };
        buildInputs = with pkgs; [
          alsa-lib
          clang
          libGL
          libxkbcommon
          mold
          pkg-config
          udev
          vulkan-loader
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
        ];
        nativeBuildInputs = with pkgs; [
          (rust-bin.selectLatestNightlyWith
            (toolchain: toolchain.default.override {
              extensions = [ "rust-src" "clippy" ];
            }))
        ];
        all_deps = with pkgs; [
          cargo-flamegraph
          cargo-expand
          nixpkgs-fmt
          cmake
        ] ++ buildInputs ++ nativeBuildInputs;
      in
      rec {
        # For `nix build` & `nix run`:
        defaultPackage = packages.bevy_template;
        packages = rec {
          bevy_template = naersk'.buildPackage {
            src = ./.;
            nativeBuildInputs = nativeBuildInputs;
            buildInputs = buildInputs;
            postInstall = ''
              cp -r assets $out/bin/
            '';
            # Disables dynamic linking when building with Nix
            cargoBuildOptions = x: x ++ [ "--no-default-features" ];
          };
        };

        # For `nix develop`:
        devShell = pkgs.mkShell {
          nativeBuildInputs = all_deps;
          RUSTFLAGS = "-C link-args=-Wl,-rpath,${pkgs.lib.makeLibraryPath all_deps}";
          shellHook = ''
            export DEV_SHELL='gol '
            export CARGO_MANIFEST_DIR=$(pwd)
          '';
        };
      }
    );
}
