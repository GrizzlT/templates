{
  description = "Empty flake";

  outputs = { self, systems, nixpkgs, ... }: let
    eachSystem = c: nixpkgs.lib.genAttrs (import systems) (system: c (
      import nixpkgs {
        inherit system;
        overlays = [ self.overlays.default ];
      }
    ));
  in {
    packages = eachSystem (pkgs: {
    });

    devShells = eachSystem (pkgs: {
      default = pkgs.mkShell {
        buildInputs = with pkgs; [
        ];
      };
    });

    overlays.default = import ./overlay.nix;
  };

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.2311.tar.gz";

    systems.url = "github:nix-systems/x86_64-linux";
  };
}
