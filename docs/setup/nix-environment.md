# Nix Development Environment Setup

## Prerequisites

### NixOS
Already installed! Enable flakes if not already:
```bash
# Add to /etc/nixos/configuration.nix
nix.settings.experimental-features = [ "nix-command" "flakes" ];
```

### Windows 11

1. **Install WSL2** (if not already installed):
```powershell
wsl --install
```

2. **Install Nix in WSL**:
```bash
# Inside WSL
curl -L https://nixos.org/nix/install | sh -s -- --daemon

# Enable flakes
mkdir -p ~/.config/nix
echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf
```

3. **Install direnv**:
```bash
# Inside WSL
nix profile install nixpkgs#direnv
echo 'eval "$(direnv hook bash)"' >> ~/.bashrc
source ~/.bashrc
```

## Using the Development Environment

### First Time Setup

```bash
# Clone repository
cd /path/to/HandPlusPlus

# Allow direnv to load .envrc
direnv allow

# Development shell will automatically activate
# Alternatively, manually enter:
nix develop
```

### Verify Setup

```bash
rustc --version   # Should show Rust stable
cargo --version
rust-analyzer --version
```

## Platform-Specific Notes

### Windows 11 + WSL
- Rust builds run in WSL (Linux environment)
- For Windows-native development, install Rust via rustup separately
- Use `cargo build --target x86_64-pc-windows-gnu` for Windows binaries from WSL

### NixOS
- X11 libraries are included for input capture/simulation
- Ensure user is in `input` group for evdev access:
  ```bash
  sudo usermod -a -G input $USER
  ```

## Troubleshooting

### "direnv: error .envrc is blocked"
```bash
direnv allow
```

### Missing X11 libraries
The flake should provide all dependencies. If issues persist:
```bash
nix develop --impure
```

### Windows-specific Rust toolchain
For native Windows development outside Nix:
```powershell
# Install rustup
winget install Rustlang.Rustup

# Install MSVC build tools
# Download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/
```
