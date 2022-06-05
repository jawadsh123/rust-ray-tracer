# { nixPkgs ? import <nixpkgs> { }}:

let
  rustOverlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> {
    overlays = [ rustOverlay ];
  };
  rust = pkgs.rust-bin.stable.latest.default.override{
    # for rust-analyzer
    extensions = [ "rust-src" ];
  };
in
  pkgs.mkShell {
    name = "rust-ray-tracer";

    buildInputs = with pkgs; [
      rust
      rust-analyzer
      darwin.apple_sdk.frameworks.OpenGL
      darwin.apple_sdk.frameworks.AppKit
    ];

    RUST_BACKTRACE = 1;
  }