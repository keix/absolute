{
  description = "system-calls.com API";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
          config.allowUnfreePredicate = pkg:
            builtins.elem (nixpkgs.lib.getName pkg) [ "dynamodb-local" ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
          targets = [ "aarch64-unknown-linux-gnu" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustToolchain
            cargo-lambda
            zig
            nodejs_22
            awscli2
            dynamodb-local
            jq
            pkg-config
            openssl
          ];

          shellHook = ''
            export AWS_PAGER=""
            export RUST_LOG=''${RUST_LOG:-info,system_calls=debug}
            export DDB_ENDPOINT=''${DDB_ENDPOINT:-http://localhost:8000}
            export DDB_TABLE=''${DDB_TABLE:-system-calls-dev}
            export AWS_REGION=''${AWS_REGION:-ap-northeast-1}

            if [[ -z "''${AWS_PROFILE:-}" ]]; then
              export AWS_ACCESS_KEY_ID=''${AWS_ACCESS_KEY_ID:-local}
              export AWS_SECRET_ACCESS_KEY=''${AWS_SECRET_ACCESS_KEY:-local}
            fi

            echo "system-calls.com dev shell"
            echo "  rustc       : $(rustc --version)"
            echo "  cargo-lambda: $(cargo lambda --version 2>/dev/null || echo 'n/a')"
            echo "  node        : $(node --version)"
            echo "  aws         : $(aws --version 2>&1 | head -n1)"
            echo ""
            echo "Local DDB    : $DDB_ENDPOINT  (start with: dynamodb-local -port 8000)"
            echo "Local table  : $DDB_TABLE"
          '';
        };
      });
}
