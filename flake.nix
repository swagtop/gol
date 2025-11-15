{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    bevy-flake = {
      url = "github:swagtop/bevy-flake/dev";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { nixpkgs, bevy-flake, ... }:
    let
      bf = bevy-flake.override {
        buildSource = ./.;
      };
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
        {
          default = pkgs.writeShellScriptBin manifest.package.name ''
            exec ${pkgs.steam-run-free}/bin/steam-run "${
              bf.packages.${system}.targets.${systemTarget}
            }/bin/${manifest.package.name}"
          '';
        }
      );
    };
}
