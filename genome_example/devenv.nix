{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  # Rust toolchain: latest stable via rust-overlay.
  # Provides rustc, cargo, clippy, rustfmt, rust-analyzer.
  languages.rust = {
    enable = true;
    channel = "stable";
  };

  packages = [ pkgs.git ];

  enterShell = ''
    rustc --version
    cargo --version
  '';
}
