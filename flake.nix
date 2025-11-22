{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    bevy-flake = {
      url = "github:swagtop/bevy-flake/dev";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      bevy-flake,
      fenix,
      ...
    }:
    let
      bf = bevy-flake.configure ({ pkgs, ... }: {
        src = ./.;
        rustToolchainFor =
          targets:
          let
            fx = fenix.packages.${pkgs.stdenv.hostPlatform.system};
            channel = "stable"; # For nightly, use "latest".
          in
          fx.combine (
            [ (fx.${channel}.completeToolchain or fx.channel.toolchain) ]
            ++ map (target: fx.targets.${target}.${channel}.rust-std) targets
          );
      });
    in
    {
      inherit (bf) devShells formatter;

      packages = bf.eachSystem (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          systemTarget = pkgs.stdenv.hostPlatform.config;
          manifest = nixpkgs.lib.importTOML ./Cargo.toml;
        in
        bf.packages.${system}
        // {
          default =
            if pkgs.stdenv.isLinux then
              pkgs.writeShellScriptBin manifest.package.name ''
                exec ${pkgs.steam-run-free}/bin/steam-run "${
                  bf.packages.${system}.targets.${systemTarget}
                }/bin/${manifest.package.name}"
              ''
            else
              pkgs.rustPlatform.buildRustPackage {
                inherit (manifest.package) version;
                pname = manifest.package.name;
                src = ./.;
                cargoLock.lockFile = ./Cargo.lock;
              };
        }
      );
    };
}
