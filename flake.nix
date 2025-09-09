{
  description = "Flake templates for diverse projects";

  outputs = { ... }: {
    templates = {
      empty = {
        description = "Empty flake with boilerplate inputs/outputs.";
        path = ./empty;
      };
      rustAsyncCli = {
        description = "Rust project with config file support and async main loop";
        path = ./rust_config_async;
      };
    };
  };
}
