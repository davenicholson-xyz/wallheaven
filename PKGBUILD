# Maintainer: Dave Nicholson <me@davenicholson.xyz>

pkgname=wallheaven
pkgver=0.1.2
pkgrel=1
pkgdesc='wallheaven is a CLI tool for fetching random wallpapers from wallhaven.cc.'
url='https://github.com/davenicholson-xyz/wallhaven'
license=('MIT')
makedepends=(cargo)
depends=()
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
source=("https://github.com/davenicholson-xyz/$pkgname/releases/download/v$pkgver/source.tar.gz")
sha256sums=('e82993b3890011027c181e7d6a48f7a0b0f63b3699ae9a958130c0608e0c717e')

prepare() {
  export RUSTUP_TOOLCHAIN=stable
  cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release --all-features
}

# check() {
#   export RUSTUP_TOOLCHAIN=stable
#   cargo test --frozen --all-features
# }

package() {
  install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
}
