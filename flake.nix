{
  description = "DDosFTW — fake hacking CLI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      perSystem =
        { pkgs, lib, ... }:
        let
          cargoToml = lib.fromTOML (builtins.readFile ./Cargo.toml);
        in
        {
          packages.default = pkgs.rustPlatform.buildRustPackage {
            pname = cargoToml.package.name;
            version = cargoToml.package.version;

            src = lib.cleanSource ./.;
            cargoLock.lockFile = ./Cargo.lock;

            meta = {
              description = "Fake hacking CLI for trolling";
              mainProgram = cargoToml.package.name;
            };
          };

          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              cargo
              rustc
              rust-analyzer
              gnumake
            ];

            NIX_ENFORCE_PURITY = 0;

            shellHook = ''
              rustc --version
              echo "Autobuilding project..."
              make build
            '';
          };
        };
    };
}
