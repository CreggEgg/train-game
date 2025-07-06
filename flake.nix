{
  description = "devshell for train-game";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
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
        devShells.default =
          with pkgs;
          mkShell rec {
            buildInputs = [
              rustc
              rust-analyzer
              rustfmt
              cargo

              pkg-config
              udev
              alsa-lib-with-plugins
              vulkan-loader
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr # To use the x11 feature
              libxkbcommon
              wayland # To use the wayland feature
              clang
              mold
            ];

            shellHook = ''
              export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
            '';
          };
      }
    );
}
