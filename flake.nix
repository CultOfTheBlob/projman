{
  description = "Rust development environment.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    rust-overlay.url = "github:oxalica/rust-overlay";

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    naersk,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {inherit system overlays;};
      naerskLib = pkgs.callPackage naersk {};
    in {
      packages.default = naerskLib.buildPackage {
        src = self;

        buildInputs = with pkgs; [
          glib
          libxcb
          libxkbcommon
          fontconfig
          vulkan-loader
          pango
          atk
          gtk3
          openssl
        ];
        nativeBuildInputs = with pkgs; [pkg-config makeWrapper];

        postInstall = ''
          wrapProgram $out/bin/projman \
            --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath (with pkgs; [
            libxcb
            wayland
            libxkbcommon
            libGL
            mesa
            vulkan-loader
          ])}
        '';
      };

      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          (rust-bin.nightly."2026-02-01".default.override {
            extensions = ["rust-src" "rust-analyzer" "clippy" "rustfmt"];
          })
          glib
          just

          libxcb
          libxkbcommon
          fontconfig
          pango
          atk
          gtk3
          openssl
        ];

        LD_LIBRARY_PATH = with pkgs;
          pkgs.lib.makeLibraryPath [
            libxcb
            wayland
            libxkbcommon
            libGL
            mesa
            vulkan-loader
          ];

        nativeBuildInputs = [pkgs.pkg-config];
      };
    });
}
