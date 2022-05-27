<div align="top"></div>
<br />
<div align="center">

![Bibi](assets/Bibi.png)

### Bibi

A simple Hololive Stream Schedule CLI tool written in Rust.

[Features](#features)
•
[Usage](#usage)
•
[Installation](#installation)

</div>

## Features

- View the schedules of Hololive members
- ... and that's it.

## Usage

Simply running `bibi` without any arguments will print the current and upcoming streams.

![Usage](assets/usage.png)

For help, run `bibi -h`.

```
bibi 0.2.0
A simple Hololive Schedule CLI tool.
Shows current and upcoming streams by default.

USAGE:
    bibi [OPTIONS]

OPTIONS:
    -a, --ascii       Print a cute Bibi ascii art
    -e, --ended       Show streams that have ended
    -h, --help        Print help information
    -l, --live        Show streams that are currently live
    -u, --upcoming    Show streams that have not started yet
    -V, --version     Print version information
```

## Installation

There is currently no "automatic" installation method so you'll have to build from the source code.

### Manual installation

1. Install dependencies: Rust, Cargo
2. Clone the repository

   ```sh
   git clone https://github.com/myuwi/bibi-cli.git
   cd bibi-cli
   ```

3. Build

   ```sh
   cargo build --release
   ```

4. Copy the executable to the `/usr/bin` directory.

   ```sh
   sudo install -s -Dm755 ./target/release/bibi -t /usr/bin
   ```

## Uninstallation

To uninstall Bibi, you can simply just run the following command:

```sh
sudo rm -f "$(which bibi)"
```

## Why the name "Bibi"?

From the [Hololive fan wiki][towa-wiki]: "The hat she ([Tokoyami Towa][towa-yt]) has on is actually her pet Bibi (ビビ), a guardian and protector given to her by her mother in order to look after her during her studies."

[_Go listen to Towa's music_][towa-music]

<p align="right">(<a href="#top">back to top</a>)</p>

[towa-wiki]: https://hololive.wiki/wiki/Tokoyami_Towa
[towa-yt]: https://www.youtube.com/channel/UC1uv2Oq6kNxgATlCiez59hw
[towa-music]: https://youtu.be/F1_OQXNF8b8
