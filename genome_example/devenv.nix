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

  # `lldb` provides the `lldb` CLI (manual stepping) plus the `lldb-dap` /
  # `lldb-vscode` debug adapter that nvim-dap drives. The Rust LLDB
  # pretty-printers ship with the rust-overlay toolchain (see `rust-lldb`).
  packages = [ pkgs.git pkgs.lldb ];

  enterShell = ''
    rustc --version
    cargo --version
  '';
}
