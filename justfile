set shell := ["sh", "-c"]

alias r := run
alias b := build

pkg_version := `echo "$(cargo pkgid)" | sed 's/.*@//'`

# runs the project with arguments
run *ARGS:
  RUST_BACKTRACE=1 RUST_LOG=trace cargo run -- {{ARGS}}

# builds the project
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
  strip -s target/release/ant
  cargo generate-rpm

# builds and installs the package. [possibile formats: cargo, deb, rpm]
install FORMAT:
  @just install-{{FORMAT}}

[private]
install-cargo:
  cargo install --path .

[private]
install-deb: package-deb
  sudo dpkg -i target/debian/ant_*.deb
  sudo apt-get install -f

[private]
install-rpm: package-rpm
  sudo dnf localinstall target/generate-rpm/ant-*.rpm

# creates all release files for a specific platform  [possibile platforms: linux, windows, all]
release PLATFORM:
  @just release-{{PLATFORM}}

[private]
release-all: release-deb release-rpm release-linux release-windows-x86_64

[private]
release-deb:
  mkdir -p dist
  just package-deb
  cp target/debian/ant_*.deb dist/

[private]
release-rpm:
  mkdir -p dist
  just package-rpm
  cp target/generate-rpm/ant-*.rpm dist/

[private]
release-linux:
  mkdir -p dist/ant_{{pkg_version}}
  @just release-linux-gnu-x86_64
  @just release-linux-musl-x86_64
  @just release-linux-gnu-aarch64
  @just release-linux-musl-aarch64
  @just release-linux-gnu-riscv64
  rm -rf dist/ant_{{pkg_version}}

[private]
release-linux-gnu-x86_64:
  just build --target=x86_64-unknown-linux-gnu
  cp target/x86_64-unknown-linux-gnu/release/ant dist/ant_{{pkg_version}}
  cd dist && tar -zcvf ant_{{pkg_version}}_linux_gnu.x86_64.tar.gz ant_{{pkg_version}}
  rm -rf dist/ant_{{pkg_version}}/ant

[private]
release-linux-musl-x86_64:
  just build --target=x86_64-unknown-linux-musl
  cp target/x86_64-unknown-linux-musl/release/ant dist/ant_{{pkg_version}}
  cd dist && tar -zcvf ant_{{pkg_version}}_linux_musl.x86_64.tar.gz ant_{{pkg_version}}
  rm -rf dist/ant_{{pkg_version}}/ant

[private]
release-linux-gnu-aarch64:
  CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
    CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
    CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++ \
    just build --target=aarch64-unknown-linux-gnu
  cp target/aarch64-unknown-linux-gnu/release/ant dist/ant_{{pkg_version}}
  cd dist && tar -zcvf ant_{{pkg_version}}_linux_gnu.aarch64.tar.gz ant_{{pkg_version}}
  rm -rf dist/ant_{{pkg_version}}/ant

[private]
release-linux-musl-aarch64:
  CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-musl-gcc \
    CC_aarch64_unknown_linux_musl=aarch64-linux-musl-gcc \
    CXX_aarch64_unknown_linux_musl=aarch64-linux-gnu-g++ \
    just build --target=aarch64-unknown-linux-musl
  cp target/aarch64-unknown-linux-musl/release/ant dist/ant_{{pkg_version}}
  cd dist && tar -zcvf ant_{{pkg_version}}_linux_musl.aarch64.tar.gz ant_{{pkg_version}}
  rm -rf dist/ant_{{pkg_version}}/ant

[private]
release-linux-gnu-riscv64:
  CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_LINKER=riscv64-linux-gnu-gcc \
    CC_riscv64gc_unknown_linux_gnu=riscv64-linux-gnu-gcc \
    CXX_riscv64gc_unknown_linux_gnu=riscv64-linux-gnu-g++ \
    just build --target=riscv64gc-unknown-linux-gnu
  cp target/riscv64gc-unknown-linux-gnu/release/ant dist/ant_{{pkg_version}}
  cd dist && tar -zcvf ant_{{pkg_version}}_linux_gnu.riscv64gc.tar.gz ant_{{pkg_version}}
  rm -rf dist/ant_{{pkg_version}}/ant

[private]
release-windows-x86_64:
  mkdir -p dist/ant_{{pkg_version}}
  @just release-windows-gnu-x86_64
  @just release-windows-msvc-x86_64
  rm -rf dist/ant_{{pkg_version}}

[private]
release-windows-gnu-x86_64:
  just build --target=x86_64-pc-windows-gnu
  cp target/x86_64-pc-windows-gnu/release/ant.exe dist/ant_{{pkg_version}}
  cd dist && zip ant_{{pkg_version}}_windows_gnu.x86_64.zip ant_{{pkg_version}}/ant.exe
  rm -rf dist/ant_{{pkg_version}}/ant.exe

[private]
release-windows-msvc-x86_64:
  just build --target=x86_64-pc-windows-msvc
  cp target/x86_64-pc-windows-msvc/release/ant.exe dist/ant_{{pkg_version}}
  cd dist && zip ant_{{pkg_version}}_windows_msvc.x86_64.zip ant_{{pkg_version}}/ant.exe
  rm -rf dist/ant_{{pkg_version}}/ant.exe

# removes all build & packaging files
clean:
  cargo clean
  rm -rf dist