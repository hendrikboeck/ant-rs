set shell := ["sh", "-c"]

alias r := run
alias b := build

version := `echo "$(cargo pkgid)" | sed 's/.*#//'`
name := "ant-rs"
pkgid := name + "_" + version

# runs the project with arguments
run *ARGS:
  RUST_BACKTRACE=1 RUST_LOG=trace cargo run -- {{ARGS}}

# builds the project in `--release` mode
build *ARGS:
  cargo build --release {{ARGS}}

# builds and installs the package. [possibile formats: deb, rpm]
package FORMAT:
  @just package-{{FORMAT}}

[private]
package-deb:
  rustup update
  cargo install cargo-deb
  cargo deb

[private]
package-rpm: build
  cargo install cargo-generate-rpm
  strip -s target/release/{{name}}
  cargo generate-rpm

# builds and installs the package. [possibile formats: cargo, deb, rpm]
install FORMAT:
  @just install-{{FORMAT}}

[private]
install-cargo:
  cargo install --path .

[private]
install-deb: package-deb
  sudo dpkg -i target/debian/{{name}}_*.deb
  sudo apt-get install -f

[private]
install-rpm: package-rpm
  sudo dnf localinstall target/generate-rpm/{{name}}-*.rpm

# creates all release files for a specific platform [possibile platforms: all, linux, windows, linux-gnu-x86_64, linux-musl-x86_64, linux-gnu-aarch64, linux-musl-aarch64, linux-gnu-riscv64, windows-x86_64, windows-msvc-x86_64]
release PLATFORM:
  @just release-{{PLATFORM}}

[private]
release-all: release-deb release-rpm release-linux release-windows-x86_64

[private]
release-deb:
  mkdir -p dist
  just package-deb
  cp target/debian/{{name}}_*.deb dist/

[private]
release-rpm:
  mkdir -p dist
  just package-rpm
  cp target/generate-rpm/{{name}}-*.rpm dist/

[private]
release-linux:
  @just release-linux-gnu-x86_64
  @just release-linux-musl-x86_64
  @just release-linux-gnu-aarch64
  @just release-linux-musl-aarch64
  @just release-linux-gnu-riscv64

[private]
prepare-release-linux:
  mkdir -p dist/{{pkgid}}/{bin,systemd,man,docs}
  cp README.md                 dist/{{pkgid}}/docs/README
  cp res/ant-rs.man            dist/{{pkgid}}/man/ant-rs.8
  cp res/ant-rs-daemon.service dist/{{pkgid}}/systemd/
  cp res/ant.systemd.yaml      dist/{{pkgid}}/systemd/ant.yaml
  cp res/ant.example.yaml      dist/{{pkgid}}/
  cp res/install.sh            dist/{{pkgid}}/
  chmod a+x dist/{{pkgid}}/install.sh

[private]
cleanup-release-linux:
  rm -rf dist/{{pkgid}}

[private]
release-linux-gnu-x86_64:
  just prepare-release-linux
  just build --target=x86_64-unknown-linux-gnu
  cp target/x86_64-unknown-linux-gnu/release/{{name}} dist/{{pkgid}}/bin/
  cd dist && tar -zcvf {{pkgid}}_linux_gnu.x86_64.tar.gz {{pkgid}}
  just cleanup-release-linux

[private]
release-linux-musl-x86_64:
  just prepare-release-linux
  just build --target=x86_64-unknown-linux-musl
  cp target/x86_64-unknown-linux-musl/release/{{name}} dist/{{pkgid}}/bin/
  cd dist && tar -zcvf {{pkgid}}_linux_musl.x86_64.tar.gz {{pkgid}}
  just cleanup-release-linux

[private]
release-linux-gnu-aarch64:
  just prepare-release-linux
  just build --target=aarch64-unknown-linux-gnu
  cp target/aarch64-unknown-linux-gnu/release/{{name}} dist/{{pkgid}}/bin/
  cd dist && tar -zcvf {{pkgid}}_linux_gnu.aarch64.tar.gz {{pkgid}}
  just cleanup-release-linux

[private]
release-linux-musl-aarch64:
  just prepare-release-linux
  just build --target=aarch64-unknown-linux-musl
  cp target/aarch64-unknown-linux-musl/release/{{name}} dist/{{pkgid}}/bin/
  cd dist && tar -zcvf {{pkgid}}_linux_musl.aarch64.tar.gz {{pkgid}}
  just cleanup-release-linux

[private]
release-linux-gnu-riscv64:
  just prepare-release-linux
  just build --target=riscv64gc-unknown-linux-gnu
  cp target/riscv64gc-unknown-linux-gnu/release/{{name}} dist/{{pkgid}}/bin/
  cd dist && tar -zcvf {{pkgid}}_linux_gnu.riscv64gc.tar.gz {{pkgid}}
  just cleanup-release-linux

[private]
release-windows-x86_64:
  @just release-windows-gnu-x86_64
  @just release-windows-msvc-x86_64

[private]
prepare-release-windows:
  mkdir -p dist/{{pkgid}}
  cp README.md            dist/{{pkgid}}/
  cp res/ant.example.yaml dist/{{pkgid}}/

[private]
cleanup-release-windows:
  rm -rf dist/{{pkgid}}

[private]
release-windows-gnu-x86_64:
  just prepare-release-windows
  just build --target=x86_64-pc-windows-gnu
  cp target/x86_64-pc-windows-gnu/release/{{name}}.exe dist/{{pkgid}}/
  cd dist && zip {{pkgid}}_windows_gnu.x86_64.zip {{pkgid}}/*
  just cleanup-release-windows

[private]
release-windows-msvc-x86_64:
  just prepare-release-windows
  just build --target=x86_64-pc-windows-msvc
  cp target/x86_64-pc-windows-msvc/release/{{name}}.exe dist/{{pkgid}}/
  cd dist && zip {{pkgid}}_windows_msvc.x86_64.zip {{pkgid}}/*
  just cleanup-release-windows

# removes all build & packaging files
clean:
  cargo clean
  rm -rf dist