{
  description = "HandPlusPlus - Cross-platform hotkey automation in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        # Rust toolchain with components
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
        };

        # Platform-specific dependencies
        platformDeps = with pkgs; [
          # Common dependencies
          pkg-config
          openssl
          
          # Linux-specific (X11/Wayland)
          xorg.libX11
          xorg.libXi
          xorg.libXtst
          xorg.libXrandr
          libxkbcommon
          
          # For evdev support (optional)
          libevdev
        ] ++ lib.optionals stdenv.isDarwin [
          # macOS-specific (future support)
          darwin.apple_sdk.frameworks.Cocoa
          darwin.apple_sdk.frameworks.Carbon
        ];

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            rustToolchain
            
            # Build tools
            cargo-watch
            cargo-edit
            cargo-outdated
            cargo-audit
            
            # Platform dependencies
          ] ++ platformDeps ++ [
            # Development tools
            git
            just # Command runner (alternative to make)
            
            # Documentation
            mdbook
          ];

          # Environment variables
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          
          # For X11 development
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath platformDeps;
          
          shellHook = ''
            echo "🔧 HandPlusPlus Development Environment"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            echo "Rust:         $(rustc --version)"
            echo "Cargo:        $(cargo --version)"
            echo "Platform:     ${system}"
            echo ""
            echo "Available commands:"
            echo "  cargo build          - Build the project"
            echo "  cargo test           - Run tests"
            echo "  cargo watch -x run   - Auto-rebuild on changes"
            echo "  cargo clippy         - Run linter"
            echo ""
            echo "📚 Documentation: docs/arc42/"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
          '';
        };
      }
    );
}
