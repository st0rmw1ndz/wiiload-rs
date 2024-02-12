# wiiload-rs

[![Rust](https://github.com/st0rmw1ndz/wiiload-rs/workflows/Rust/badge.svg)](https://github.com/st0rmw1ndz/wiiload-rs/actions/workflows/rust.yml)
[![Release](https://img.shields.io/github/v/release/st0rmw1ndz/wiiload-rs)](https://github.com/st0rmw1ndz/wiiload-rs/releases/latest)


Wiiload implementation in Rust.

> [!NOTE]
> This is my first Rust project. If you have any improvements, feel free to make a [pull request](https://github.com/st0rmw1ndz/wiiload-rs/pulls)!

## Usage

```
Wiiload implementation in Rust.

Usage: wiiload [OPTIONS] <IP> <PATH>

Arguments:
  <IP>    IP address to connect to
  <PATH>  Path to file to send (.dol, .elf, .zip)

Options:
  -p, --port <PORT>                 Port to connect to [default: 4299]
  -t, --timeout <TIMEOUT_DURATION>  Timeout seconds [default: 30]
  -h, --help                        Print help
  -V, --version                     Print version
```

## Future Features

- [x] Better descriptions in help menu
- [x] Checking for file compatibility (`.elf`, `.dol`, `.zip` only)
  - [ ] Checking `.zip` structure to validate the Homebrew Channel will accept it
- [ ] Saving information like IP address in a configuration file