set shell := ["sh", "-c"]

alias r := run
alias b := build

version := `echo "$(cargo pkgid)" | sed 's/.*#//'`
name := "ant-rs"

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
  mkdir -p dist/{{name}}_{{version}}
  @just release-{{PLATFORM}}
  rm -rf dist/{{name}}_{{version}}

[private]
release-all: release-deb release-rpm release-linux release-windows-x86_64

[private]
release-deb:
  just package-deb
  cp target/debian/{{name}}_*.deb dist/

[private]
release-rpm:
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
release-linux-gnu-x86_64:
  just build --target=x86_64-unknown-linux-gnu
  cp target/x86_64-unknown-linux-gnu/release/{{name}} dist/{{name}}_{{version}}
  cd dist && tar -zcvf {{name}}_{{version}}_linux_gnu.x86_64.tar.gz {{name}}_{{version}}
  rm -rf dist/{{name}}_{{version}}/{{name}}

[private]
release-linux-musl-x86_64:
  just build --target=x86_64-unknown-linux-musl
  cp target/x86_64-unknown-linux-musl/release/{{name}} dist/{{name}}_{{version}}
  cd dist && tar -zcvf {{name}}_{{version}}_linux_musl.x86_64.tar.gz {{name}}_{{version}}
  rm -rf dist/{{name}}_{{version}}/{{name}}

[private]
release-linux-gnu-aarch64:
  just build --target=aarch64-unknown-linux-gnu
  cp target/aarch64-unknown-linux-gnu/release/{{name}} dist/{{name}}_{{version}}/
  cd dist && tar -zcvf {{name}}_{{version}}_linux_gnu.aarch64.tar.gz {{name}}_{{version}}
  rm -rf dist/{{name}}_{{version}}/{{name}}

[private]
release-linux-musl-aarch64:
  just build --target=aarch64-unknown-linux-musl
  cp target/aarch64-unknown-linux-musl/release/{{name}} dist/{{name}}_{{version}}
  cd dist && tar -zcvf {{name}}_{{version}}_linux_musl.aarch64.tar.gz {{name}}_{{version}}
  rm -rf dist/{{name}}_{{version}}/{{name}}

[private]
release-linux-gnu-riscv64:
  just build --target=riscv64gc-unknown-linux-gnu
  cp target/riscv64gc-unknown-linux-gnu/release/{{name}} dist/{{name}}_{{version}}
  cd dist && tar -zcvf {{name}}_{{version}}_linux_gnu.riscv64gc.tar.gz {{name}}_{{version}}
  rm -rf dist/{{name}}_{{version}}/{{name}}

[private]
release-windows-x86_64:
  @just release-windows-gnu-x86_64
  @just release-windows-msvc-x86_64

[private]
release-windows-gnu-x86_64:
  just build --target=x86_64-pc-windows-gnu
  cp target/x86_64-pc-windows-gnu/release/{{name}}.exe dist/{{name}}_{{version}}
  cd dist && zip {{name}}_{{version}}_windows_gnu.x86_64.zip {{name}}_{{version}}/{{name}}.exe
  rm -rf dist/{{name}}_{{version}}/{{name}}.exe

[private]
release-windows-msvc-x86_64:
  just build --target=x86_64-pc-windows-msvc
  cp target/x86_64-pc-windows-msvc/release/{{name}}.exe dist/{{name}}_{{version}}
  cd dist && zip {{name}}_{{version}}_windows_msvc.x86_64.zip {{name}}_{{version}}/{{name}}.exe
  rm -rf dist/{{name}}_{{version}}/{{name}}.exe

# removes all build & packaging files
clean:
  cargo clean
  rm -rf dist