{
  description = "A Nix-flake-based Protobuf development environment";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
        pkgs = import nixpkgs { inherit system; };
      });
    in
    {
      devShells = forEachSupportedSystem ({ pkgs }: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            protobuf
            ruby_3_2
            go
            python310
            python310Packages.protobuf
            python310Packages.grpcio-tools
            python310Packages.mypy-protobuf
            python310Packages.toml
          ];
          shellHook = ''
            # Custom environment manipulations can go here
            # Example: export PATH=$PATH:/path/to/custom/bin
            go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@v1.3.0
            python -m venv .venv
            source .venv/bin/activate
            python -m ensurepip --upgrade
            pip install --upgrade pip
            pip install protobuf_to_pydantic
            gem install grpc-tools --bindir=./bin
            export PATH=$(pwd)/bin:$PATH
          '';
        };

      });
    };
}
