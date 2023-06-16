# 🐜 ANT

Another Network Tunnel; A simple program for local/remote port forwarding over a SSH tunnel.

![Recording](./.github/md/rec.svg)

# Features

## Table of Contents

- [Installation](#installation)
  - [Pre-requisites](#pre-requisites)
    - [Linux](#linux)
    - [MacOs](#macos)
    - [Windows](#windows)
  - [Pre-packaged Binary](#pre-packaged-binary)
  - [Compilation](#compilation)
- [Usage](#usage)
- [Configuration](#configuration)
- [License](#license)

# Installation

<sup>[(Back to top)](#table-of-contents)</sup>

## Pre-requisites

<sup>[(Back to top)](#table-of-contents)</sup>

This application was written in the Rust language. If you plan on compiling the application from source, you have to have the Rust toolchain installed. See [Install Rust](https://www.rust-lang.org/tools/install) for more information.

For this application to work you have to have an `openssh` compatible client installed on your system and have it as the `ssh` command in your `$PATH`.

### Linux

<sup>[(Back to top)](#table-of-contents)</sup>

Most Linux distributions ship an `openssh` client out of the box. If your Linux distribution does not ship a client, you may find the installation command below for your system.

**Ubuntu/Debian**

    sudo apt install openssh-client

**Arch**

    sudo pacman -S openssh

**Fedora/RedHat**

    sudo dnf install openssh

**OpenSUSE**

    sudo zypper in openssh-clients

### MacOS

<sup>[(Back to top)](#table-of-contents)</sup>

### Windows

<sup>[(Back to top)](#table-of-contents)</sup>

## Pre-packaged Binary

<sup>[(Back to top)](#table-of-contents)</sup>

This application provides pre-packaged binaries for different architectures and Operating Systems under the [**Releases**](https://github.com/hendrikboeck/ant-rs/releases) tab of GitHub.

## Compilation

<sup>[(Back to top)](#table-of-contents)</sup>

To compile the programyou can use `just` scripts. You can append a `cargo build` flags as you wish, just the flag `--release` is set by default.

    just b

To install the program you can use `just install` and then specify the installation package manager. For the installation targets `deb` and `rpm` `sudo`-privileges may be required on your system.

    just install [cargo|deb|rpm]

# Usage

<sup>[(Back to top)](#table-of-contents)</sup>

    --------------------------------------------------
    🐜 ANT 0.1.2 - Command Line Tool
    --------------------------------------------------
    Another Network Tunnel; A simple program for local/remote port forwarding over a SSH tunnel.

    Usage: ant [OPTIONS] <HOST>

    Arguments:
      <HOST>  Host to create tunnel(s) for. Has to be in `hosts` inside your ANT configuration file. This host has to accessable without user input. (use identity_file, etc. for authentication)

    Options:
      -l, --log-level <LOG_LEVEL>  Log level of application [default: info]
      -c, --config <CONFIG>        Path to ANT configuration file `ant.yaml` [default: ~/.ssh/ant.yaml]
      -h, --help                   Print help
      -V, --version                Print version

# Configuration

<sup>[(Back to top)](#table-of-contents)</sup>

# License

<sup>[(Back to top)](#table-of-contents)</sup>

ant, program for port forwarding addresses over a SSH tunnel<br />
Copyright (C) 2023, Hendrik Böck <hendrikboeck.dev@protnmail.com>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.

See the file LICENSE for details.
