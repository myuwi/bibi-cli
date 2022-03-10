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
- ... and that's it. For now.

## Usage

Simply running the command without any arguments will print the schedule of current and upcoming streams.

![Usage](assets/usage.png)

For help, run `bibi -h`.

## Installation

There is currently no "automatic" installation method so you'll have to build from the source code.

### Manual installation

1. Install the dependencies: Rust, Cargo
2. Clone the repository

    ```sh
    git clone https://github.com/myuwi/bibi-cli.git
    cd bibi-cli
    ```

3. Build the app

    ```sh
    cargo build --release
    ```

4. Then copy the compiled binary to the `/usr/bin` directory.

    ```sh
    sudo install -s -Dm755 ./target/release/bibi -t /usr/bin
    ```

### Manual development installation

Alternatively you can symlink the binary to `/usr/bin` to keep it up to date easier. This method requires you to not remove the build directory, as removing it will also remove the actual binary that's symlinked to `/usr/bin`.

1. Install the dependencies: Rust, Cargo
2. Clone the repository

    ```sh
    git clone https://github.com/myuwi/bibi-cli.git
    cd bibi-cli
    ```

3. Build the app

    ```sh
    cargo build --release
    ```

4. Then symlink the compiled binary to `/usr/bin/bibi`.

    ```sh
    sudo ln -s "$(pwd)"/target/release/bibi /usr/bin/bibi
    ```

## Uninstallation

1. To uninstall Bibi, you can simply just run the following command:

    ```sh
    sudo rm -f "$(which bibi)"
    ```

## Why the name "Bibi"?

Bibi is the name of [Tokoyami Towa][towa-yt]'s (a Hololive member) "hat", who is also her "pet guardian".

*Go listen to Towa's Music*

- [-ERROR/常闇トワ(cover)][towa-error]
- [【オリジナル曲】　Palette/常闇トワ　【フルMV】][towa-palette]
- [【MV】マドロミ / 天音かなた・常闇トワ【ディープインサニティ アサイラム ゲーム主題歌】][towa-madoromi]
- [灰色と青/常闇トワ×星街すいせい(cover)][towa-haiirotoao]

<p align="right">(<a href="#top">back to top</a>)</p>

[towa-yt]: https://www.youtube.com/channel/UC1uv2Oq6kNxgATlCiez59hw
[towa-error]: https://youtu.be/3UV8OZj2olg
[towa-palette]: https://youtu.be/Ud73fm4Uoq0
[towa-madoromi]: https://youtu.be/23nEnPOXLEk
[towa-haiirotoao]: https://youtu.be/0firv69LkgI
