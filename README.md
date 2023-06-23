# 🐜 ANT

Another Network Tunnel; A simple program for local/remote port forwarding over a SSH tunnel.

![Recording](./.github/md/rec.svg)

## Table of Contents

- [Installation](#installation)
  - [Pre-requisites](#pre-requisites)
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

Your Mac should have a version of `ssh` already installed, otherwise you can add `openssh` via the [Homebrew](https://brew.sh/) package manager.

    brew install openssh

### Windows

<sup>[(Back to top)](#table-of-contents)</sup>

Since Windows 10 Microsoft ships a version of `win32-openssh` on your PC by default. If you want to install it nevertheless you can use the [Scoop](https://scoop.sh/) package manager. You can find futher information on SSH on Windows and Scoop [here](https://github.com/ScoopInstaller/Scoop/wiki/SSH-on-Windows).

    scoop install openssh

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
    🐜 ANT 0.1.5 - Command Line Tool
    --------------------------------------------------
    Another Network Tunnel; A simple program for local/remote port forwarding over a SSH tunnel.

    Usage: ant-rs [OPTIONS] <HOST>

    Arguments:
      <HOST>  Host to create tunnel(s) for. Has to be in `hosts` inside your ANT configuration file. This host has to be accessible without user input. (use identity_file, etc. for authentication)

    Options:
      -l, --log-level <LOG_LEVEL>  Log level of application [default: info]
      -c, --config <CONFIG>        Path to ANT configuration file `ant.yaml` [default: ~/.ssh/ant.yaml]
      -d, --daemon                 Run application in daemon mode. (will restart child process on child exit)
      -h, --help                   Print help
      -V, --version                Print version

# Configuration

<sup>[(Back to top)](#table-of-contents)</sup>

The tunnels are configured via the `ant.yaml` file (by default at `~/.ssh/ant.yaml`). An example file can be found in this repository at [`./ant.example.yaml`](./ant.example.yaml). The application uses the latest YAML standard to parse its configuration. Note that in the newest configuration only `true|True|TRUE|false|False|FALSE` are valid boolean values. `y|yes|...|n|no|...|off|...` will be read as String and may throw a hard error.

- **`version`**
  <br/>Set the version of the program that should consume the configuration file. Major and minor version should match the used version of `ant`. If this version does not mtach the application will throw a hard error.
  <br />**Type:** String
  <br />**Possible:** 0.1

- **`hosts`**
  <br />Array that contains the hosts, each host is identified via a unique name (further `$host`) and contains the configuration of the connection to the host and the tunnels.
  <br />**Type:** Host[ ]

- **`hosts.$host`**
  <br />Host entry and configuration. The host identification `$host` has to conform to default yaml key parameters.
  <br />**Type:** Host (see `hosts.$host.*`)

- **`hosts.$host.hostname`**
  <br />Specifies the real host name to log into. IP or FQDN of server you want to connect to. IP's do not need explicit `"`. Do not specifiy port of server here (See `hosts.$host.port`).
  <br />**Type:** String

- **`hosts.$host.port` (optional)**
  <br />Port of SSH service of server you want to connect to.
  <br />**Type:** u16
  <br />**Default:** 22

- **`hosts.$host.identity_file`**
  <br/>Specifies a file from which the user's DSA, ECDSA, authenticator-hosted ECDSA, Ed25519, authenticator-hosted Ed25519 or RSA authentication identity is read.
  <br />**Type:** String

- **`hosts.$host.user`**
  <br />Specifies the user to log in as.
  <br />**Type:** String

- **`hosts.$host.local_forward`**
  <br />Array of ports/addresses forwarded from the remote server to the local client. May be optional, if `remote_forward` is set.
  <br />**Type:** Item[ ]
  <br />**Item:**

  - `local`: String - IP/FQDN and port for client, where they should be accessible
  - `remote`: String - IP/FQDN and port for server, that should be forwarded

- **`hosts.$host.remote_forward`**
  <br />Array of ports/addresses forwarded from the local client to the remote server. May be optional, if `local_forward` is set.
  <br />**Type:** Item[ ]
  <br />**Item:**

  - `local`: String - IP/FQDN and port for client, that should be forwarded
  - `remote`: String - IP/FQDN and port for server, where they should be accessible

- **`hosts.$host.ssh_options` (optional)**
  <br />Additional SSH options that can be set for a specific host.
  <br />**Type:** SshOptions (see `hosts.$host.ssh_options.*`)

- **`hosts.$host.ssh_options.strict_host_key_checking` (optional)**
  <br />If this option is set to `yes`, `ssh` will never automatically add host keys to the `~/.ssh/known_hosts` file, and refuses to connect to hosts whose host key has changed. This provides maximum protection against man-in-the-middle (MITM) attacks, though it can be annoying when the `/etc/ssh/ssh_known_hosts` file is poorly maintained or when connections to new hosts are frequently made. This option forces the user to manually add all new hosts.
  <br />If this option is set to `accept-new` then `ssh` will automatically add new host keys to the user's known_hosts file, but will not permit connections to hosts with changed host keys. If this option is set to `no` or `off`, `ssh` will automatically add new host keys to the user known hosts files and allow connections to hosts with changed hostkeys to proceed, subject to some restrictions.
  <br />**Type:** String
  <br />**Default:** yes
  <br />**Possible:**

  - on, yes
  - off, no
  - accept-new

- **`hosts.$host.ssh_options.bind_address` (optional)**
  <br />Use the specified address on the local machine as the source address of the connection. Only useful on systems with more than one address.
  <br />**Type:** String
  <br />**Default:** null

- **`hosts.$host.ssh_options.batch_mode` (optional)**
  <br />If set to `yes`, user interaction such as password prompts and host key confirmation requests will be disabled. This option is useful in scripts and other batch jobs where no user is present to interact with `ssh`.
  <br />**Type:** bool
  <br />**Default:** true

- **`hosts.$host.ssh_options.compression` (optional)**
  <br />Specifies whether to use compression.
  <br />**Type:** bool
  <br />**Default:** false

- **`hosts.$host.ssh_options.connect_timeout` (optional)**
  <br />Specifies the timeout (in seconds) used when connecting to the SSH server, instead of using the default system TCP timeout. This timeout is applied both to establishing the connection and to performing the initial SSH protocol handshake and key exchange.
  <br />**Type:** usize
  <br />**Default:** 10

- **`hosts.$host.ssh_options.server_alive_interval` (optional)**
  <br />Sets a timeout interval in seconds after which if no data has been received from the server, `ssh` will send a message through the encrypted channel to request a response from the server.
  <br />**Type:** usize
  <br />**Default:** 600

- **`hosts.$host.ssh_options.exit_on_forward_failure` (optional)**
  <br />Specifies whether `ssh` should terminate the connection if it cannot set up all requested dynamic, tunnel, local, and remote port forwardings, (e.g. if either end is unable to bind and listen on a specified port).
  <br />**Type:** bool
  <br />**Default:** true

- **`hosts.$host.ssh_options.ciphers` (optional)**
  <br />Specifies the ciphers allowed and their order of preference. Multiple ciphers must be comma-separated. If the specified list begins with a `+` character, then the specified ciphers will be appended to the default set instead of replacing them. If the specified list begins with a `-` character, then the specified ciphers (including wildcards) will be removed from the default set instead of replacing them. If the specified list begins with a `^` character, then the specified ciphers will be placed at the head of the default set.
  <br />The supported ciphers are:

      3des-cbc
      aes128-cbc
      aes192-cbc
      aes256-cbc
      aes128-ctr
      aes192-ctr
      aes256-ctr
      aes128-gcm@openssh.com
      aes256-gcm@openssh.com
      chacha20-poly1305@openssh.com

  The default set is:

      chacha20-poly1305@openssh.com,aes128-ctr,aes192-ctr,aes256-ctr,
      aes128-gcm@openssh.com,aes256-gcm@openssh.com

  **Type:** String
  <br />**Default:** null

- **`hosts.$host.ssh_options.macs` (optional)**
  <br />Specifies the MAC (message authentication code) algorithms in order of preference. The MAC algorithm is used for data integrity protection. Multiple algorithms must be comma- separated. If the specified list begins with a `+` character, then the specified algorithms will be appended to the default set instead of replacing them. If the specified list begins with a `-` character, then the specified algorithms (including wildcards) will be removed from the default set instead of replacing them. If the specified list begins with a `^` character, then the specified algorithms will be placed at the head of the default set.
  <br />The algorithms that contain `-etm` calculate the MAC after encryption (encrypt-then-mac). These are considered safer and their use recommended.
  <br />The default is:

      umac-64-etm@openssh.com,umac-128-etm@openssh.com,hmac-sha2-256-etm@openssh.com,
      hmac-sha2-512-etm@openssh.com,hmac-sha1-etm@openssh.com,umac-64@openssh.com,
      umac-128@openssh.com,hmac-sha2-256,hmac-sha2-512,hmac-sha1

  **Type:** String
  <br />**Default:** null

# License

<sup>[(Back to top)](#table-of-contents)</sup>

ant, program for local/remote port forwarding over a SSH tunnel<br />
Copyright (C) 2023, Hendrik Böck <hendrikboeck.dev@protonmail.com>

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
