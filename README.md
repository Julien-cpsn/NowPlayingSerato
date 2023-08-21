Now Playing SERATO
===

[![Rust](https://github.com/Julien-cpsn/NowPlayingSerato/actions/workflows/rust.yml/badge.svg)](https://github.com/Julien-cpsn/NowPlayingSerato/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This is a little script to display the currently playing tracks of the Serato DJ software.

## How to use

You can run the script using :
```bash
cargo run -- "C:\\Users\\<User>\\Music\\_Serato_"
```

OR

```bash
cargo build
./target/debug/now-playing-serato "C:\\Users\\<User>\\Music\\_Serato_"
```

## Optional arguments

You can add `-t` or `--tracks` to change the number of tracks to display. The default is 1.

```shell
cargo run -- "C:\\Users\\<User>\\Music\\_Serato_" --tracks 5
```