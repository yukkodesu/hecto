# Hecto

[Hecto](https://www.flenker.blog/hecto/), a basic text editor, Rust Practice Project by PhilippFlenker

Hecto is a re-implementation of [kilo](https://github.com/antirez/kilo) in Rust.

# About

Hecto itself is implemented using [termion](https://crates.io/crates/termion) crate which doesn't support some terminal environment (windows console host)

This version is using [crossterm](https://crates.io/crates/crossterm). 

### Should be working on
- Console Host
    - Windows 8.1 or later
- Ubuntu Desktop Terminal
    - Ubuntu 17.10
    - Pop!_OS ( Ubuntu ) 20.04
- (Arch, Manjaro) KDE Konsole
- (Arch) Kitty
- Linux Mint
- (OpenSuse) Alacritty
- (Chrome OS) Crostini
