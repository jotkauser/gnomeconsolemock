# GnomeConsoleMock

The (probably) only way to change the opened console when clicking "Open in console" in GNOME Files.

# Usage
1. Build the binary with `cargo build --release` or download from Actions
2. Find the binary and run it (you may put it in your PATH or as a system service)
3. After first run the app will generate a config file in `~/.config/gnomeconsolemock/config.jsonc`
4. Edit the config file to your liking (set terminal command, and service to mock)
5. Run the app again
6. Try to open console from nautilus

# Works with
- [x] Ptyxis (Default in Fedora Workstation)
- [x] kgx (Default in Arch Linux GNOME installation)

# Warning
When this is running GNOME Console won't open as it can't bind to the D-Bus name.