{
  description = "Empty flake";

  outputs = { self, systems, nixpkgs, ... }: let
    eachSystem = c: nixpkgs.lib.genAttrs (import systems) (system: c ({
      inherit system;
      pkgs = nixpkgs.legacyPackages.${system};
    }));
  in {
    packages = eachSystem ({ system, ... }: {
      hello = nixpkgs.legacyPackages.${system}.hello;
    });

    devShells = eachSystem ({ system, pkgs, ... }: {
      default = pkgs.mkShell {
        buildInputs = with pkgs; [
        ];
      };
    });
  };

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.2311.tar.gz";

    systems.url = "github:nix-systems/x86_64-linux";
  };
}
