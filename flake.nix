{
  description = "Rust Devshell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    inputs@{
      self,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        formatter = pkgs.alejandra;
        devShells.default =
          with pkgs;
          mkShell {
            RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
            LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            shellHook = ''
              export LD_LIBRARY_PATH=${pkgs.wayland}/lib:$LD_LIBRARY_PATH
              export LD_LIBRARY_PATH=${pkgs.libxkbcommon}/lib:$LD_LIBRARY_PATH
              export LD_LIBRARY_PATH=${pkgs.libGL}/lib:$LD_LIBRARY_PATH
              export PATH=$PATH:/home/ishaan/.cargo/bin
            '';
            buildInputs = [
              rustup
              cargo-deb
              cargo-flamegraph
              cargo-binstall
              cargo-generate
              sccache # Improves build times
              pkg-config
              clang
              libxkbcommon
              llvmPackages_latest.llvm
              rustc
              cargo
              rustfmt
              rustPackages.clippy
              just
            ];
          };
      }
    );
}
