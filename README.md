#  🚰 `edinburgh`

 **📦  edinburgh sensor controller [🦀 **Rust**](https://github.com/lar-rs/edinburgh)**
 ** Hardware [💵 **edinburgh **]() **

🚧 _Work In Progress_ 🚧

[![travis build Status](https://travis-ci.com/greenhaus/edinburgh.svg?branch=master)](https://travis-ci.com/lar-rs/edinburgh)
[![builds.sr.ht status](https://builds.sr.ht/~asmolkov/edinburgh/.build.yml.svg)](https://builds.sr.ht/~asmolkov/lar-rs/.build.yml?)
[![open issue]][issue]
![Minimum Rust Version][min-rust-badge]

## 🎙️ Commands

`edinburgh` is a CLI tool and controller

  - ### 🦀⚙️ `serve`
    run driver and bind directory to wath data.
    All of the arguments and flags to this command are optional:
        - `path`: working directory default to`/var/run/automata/mio/edinburgh`

  - ### 🔧 `setup`
      Configure kopfmodul for user.

    ```
    edinburgh setup
    ```


## 🔩 Building

```rust
use edinburgh::Edinburgh;

fn main() {
    let mut edin = Edinburgh::open("/dev/ttyUSB0").unwrap();
    println!("CO₂ readout: {} ppm", mhz19.read().unwrap());
}
```
### raspberryPi
To cross-compile for the Raspberry Pi you will need an
`arm-unknown-linux-gnueabihf` GCC toolchain and Rust component installed. On
Arch Linux I built [arm-linux-gnueabihf-gcc] from the AUR. Add the Rust target
with `rustup target add arm-unknown-linux-gnueabihf`. Then you can
cross-compile with `cargo`:

    cargo build --release --target arm-unknown-linux-gnueabihf

After it is built copy `target/arm-unknown-linux-gnueabihf/release/lca2019` to
the Raspberry Pi.



## ⚓ Installation

1. Install `cargo`:

    Edinburgh is installed through [Cargo](https://github.com/rust-lang/cargo#compiling-from-source), a Rust package manager. Rustup, a tool for installing Rust, will also install Cargo. On Linux and macOS systems, `rustup` can be installed as follows:

    ```
    curl https://sh.rustup.rs -sSf | sh
    ```

    Additional installation methods are available [here](https://forge.rust-lang.org/other-installation-methods.html).

2. Install `edinburgh`:

    ```
    cargo install edinburgh
    ```


