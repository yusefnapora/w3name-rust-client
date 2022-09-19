{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };

        # native deps needed to build openssl and compile protobufs
        native-deps = with pkgs; [ protobuf perl ];
      in
      {
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;

          nativeBuildInputs = native-deps;

          cargoBuildOptions = opts: opts ++ ["--no-default-features"];
        };

        defaultApp = utils.lib.mkApp {
          drv = self.defaultPackage."${system}";
        };

        devShell = with pkgs; mkShell {
          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy ] ++ native-deps;
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
