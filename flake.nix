{
  description = "A fast MPS parser written in Rust";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/*.tar.gz";
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
        };
        inherit (pkgs) lib;

        rustPackageSet = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.75.0";
          packageFun = import ./Cargo.nix;
          extraRustComponents = [ "rustfmt" "clippy" ];
        };

        buildInputs = [
          pkgs.cargo
          pkgs.cargo-all-features
          pkgs.cargo-deny
          pkgs.cargo-insta
          pkgs.cargo-nextest
          pkgs.rustc
          pkgs.rustup
        ] ++ lib.optionals pkgs.stdenv.isLinux [
          pkgs.cargo-llvm-cov
        ] ++ lib.optionals pkgs.stdenv.isDarwin [
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
        ];

        mps = args: (rustPackageSet.workspace.mps ({ } // args)).overrideAttrs {
          inherit buildInputs;
        };

        workspaceShell = rustPackageSet.workspaceShell {
          packages = buildInputs;
        };
      in
      rec
      {
        packages = {
          default = mps { };
          tests = mps { compileMode = "test"; };
          ci = pkgs.rustBuilder.runTests mps {
            RUST_BACKTRACE = "full";
          };
        };

        devShell = workspaceShell;

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
