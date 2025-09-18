{
  outputs = { nixpkgs, systems, ... }@inputs:
  let
    pkgs' = system: import nixpkgs {
      inherit system;
    };
    forSystems = f: nixpkgs.lib.genAttrs (import systems) (system: let
      pkgs = pkgs' system;

      nodejs = pkgs.nodejs_22;

      mkDevShell = args: pkgs.mkShell (let
        options = let
          first = args options;
        in {
          buildInputs = first.buildInputs ++ [
            nodejs
            pkgs.typescript-language-server
          ];
        } // (builtins.removeAttrs first [ "buildInputs" ]);
      in options);

      args = { inherit pkgs mkDevShell; };
    in f args);
  in {
    devShells = forSystems ({ pkgs, mkDevShell, ... }: {
      default = mkDevShell (self: {});
    });
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
  };
}

