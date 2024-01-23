{
  description = "Flake templates for diverse projects";

  outputs = { ... }: {
    templates = {
      empty = {
        description = "Empty flake with boilerplate inputs/outputs.";
        path = ./empty;
      };
    };
  };
}
