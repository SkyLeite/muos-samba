An application for [muOS](https://muos.dev/) that lets you browse and download files off the internet. Initially made to support the SMB protocol, but will likely be extended to support others as well.

# Compiling

1. Install the [Nix Package Manager](https://nixos.org/download/)
2. Run `nix build -L .#release`
3. Copy the file in `./release/bin/muos-samba` to your muOS device

This will create an `aarch64` executable that you can run in your muOS device.

# Development

1. Install the [Nix Package Manager](https://nixos.org/download/)
2. Run `nix run`

This will pull the necessary dependencies (`SDL2`, `SDL2_gfx`), build the project for your system and run it. A window should show up in the same resolution as the RG35XXSP.

## Note on Windows

The above steps will not work on Windows. They might work on [WSL](https://learn.microsoft.com/en-us/windows/wsl/install), however.
