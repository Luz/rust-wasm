{ pkgs ? import <nixpkgs> {} }:
let
  # In case a specific one is needed:
  # rustupToolchain = "stable-x86_64-unknown-linux-gnu";

  rustBuildTarget = "wasm32-unknown-unknown";
in
pkgs.mkShell rec {
  buildInputs = with pkgs; [
    rustup
    wasm-pack
    nodejs_21
  ];
  # Avoid home dir pollution
  RUSTUP_HOME = toString ./.rustup;
  CARGO_HOME = toString ./.cargo;
  # RUSTUP_TOOLCHAIN = rustupToolchain;
  # Too lazy to always add "--target=..."
  CARGO_BUILD_TARGET = rustBuildTarget;
  
  shellHook = ''
    rustup install stable
    rustup default stable
    rustup target add "${rustBuildTarget}"
  '';
}
