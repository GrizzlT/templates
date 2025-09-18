{
  description = "Flake templates for diverse projects";

  outputs = { ... }: {
    templates = {
      empty = {
        description = "Empty flake with boilerplate inputs/outputs.";
        path = ./empty;
      };
      rustAsyncCli = {
        description = "Rust project flake with openssl and rust-overlay";
        path = ./rust_config_async/nix;
      };
      typescript = {
        description = "Simple typescript setup";
        path = ./typescript/nix;
      };
    };
  };
}
