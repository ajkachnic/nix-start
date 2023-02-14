# nix-start

A simple utility for starting nix-shell based on templates.

## Example

Create the directory `~/.config/nix-start`. Then define the file `~/.config/nix-start/binary.nix` like so:

```nix
# Utilities for basic binary manipulation
with (import <nixpkgs> {});
mkShell {
    buildInputs = with pkgs; [
        binutils
        elfkickers # Tools for stripping binaries
        radare2 # UNIX-like reverse engineering framework
    ];
}
```

Now, if you run the command `nix-start binary`, it will load you into that environment.
