set shell := ["sh", "-c"]

alias r := run
alias b := build

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

# removes all build & packaging files
clean:
  rm -rf target