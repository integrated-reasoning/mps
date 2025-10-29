{
  description = "A fast MPS parser written in Rust";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/*.tar.gz";
    naersk.url = "github:nix-community/naersk";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        inherit (pkgs) lib;

        naersk' = pkgs.callPackage naersk { };

        additionalDevTools = [
          pkgs.cargo-all-features
          pkgs.cargo-deny
          pkgs.cargo-insta
          pkgs.cargo-nextest
          pkgs.rustfmt
          pkgs.clippy
        ] ++ lib.optionals pkgs.stdenv.isLinux [
          pkgs.cargo-llvm-cov
        ];

        buildDependencies = lib.optionals pkgs.stdenv.isDarwin [
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
        ];

        mps = naersk'.buildPackage {
          src = ./.;
          buildInputs = buildDependencies;
          cargoBuildOptions = x: x ++ [ "--features" "cli" ];
        };
      in
      rec
      {
        packages = {
          default = mps;
        };

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
          ] ++ additionalDevTools ++ buildDependencies;
        };

        image = pkgs.dockerTools.buildLayeredImage {
          name = "mps";
          tag = "latest";
          maxLayers = 120;
          contents = [
            packages.default
          ];
          config.Cmd = [ "mps" ];
        };
      }
    );
}
