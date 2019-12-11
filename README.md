# :electric_plug: `edinburgh`

**📦  LAR edinburgh sensors driver rewrite in [🦀 **Rust**](https://github.com/lar-rs/edinburgh)**

🚧 _Work In Progress_ 🚧

[![travis build Status](https://travis-ci.com/lar-rs/sensors.svg?branch=master)](https://travis-ci.com/lar-rs/edinburgh)
[![builds.sr.ht status](https://builds.sr.ht/~asmolkov/sensors/.build.yml.svg)](https://builds.sr.ht/~asmolkov/lar-rs/edinburgh.build.yml?)
![open issue][issue]
![Minimum Rust Version][min-rust-badge]


## Hardwarehersteller
 * [sensirion](https://www.sensirion.com/de)
    * [CO2](https://www.sensirion.com/de/umweltsensoren/kohlendioxidsensoren-co2)
**Gascard NG**

Infrared Gas Sensor
The Gascard NG infrared gas sensor is designed for ease of integration with a wide range of gas detection systems that require high quality, accurate and reliable measurement of CO, CO2, CH4 gas concentrations.
It includes real-time temperature and atmospheric pressure correction via on-board sensors and has the flexibility to incorporate additional gas detection technologies. It has onboard true RS232 communications along with the option of TCP/IP communications protocol.
All orders are shipped with free logging software information on a USB key. You will simply need to purchase a cable.
j
## Project Description

Data example:
  **Data  "N 0.0414 0.0000 0.0000 0.00 0.0000 22942 992.6";**
  **Regex::new(r"N (?P<fsr>\d{1}.\d{4}) \d{1}.\d{4} \d{1}.\d{4} \d{1}.\d{2} \d{1}.\d{4} (?P<dig>\d{5}) (?P<ppm>\d{1}.\d{4}) \d{1}").unwrap()**

## Background

## Software Documentation

## 🎙️ Commands

`edinburgh` noch nicht implementiert

  - ⚙️ `cwhhserve`
    run driver and bind directory to wath data.
    All of the arguments and flags to this command are optional:

  - 🔧 `setup`
      Configure kopfmodul for user.

    ```
    sensors setup
    ```

<!-- links -->
[file issues]: https://github.com/lar-rs/edinburgh/issues/
[Rust]: https://www.rust-lang.org/
[async-std]:https://docs.rs/async-std/0.99.10/async_std
[edinburg]:https://edinburghsensors.com/products/oem-co2-sensor/gascard-ng/
[CONTRIBUTING.md]: CONTRIBUTING.md
[CC-BY 4.0]: https://opendefinition.org/licenses/cc-by/
[MIT]: https://opensource.org/licenses/MIT
[The Rust Book]: https://doc.rust-lang.org/book/
[building a command-line program]: https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html
[building a multithreaded web server]: https://doc.rust-lang.org/stable/book/ch20-00-final-project-a-web-server.html
[clippy]: https://github.com/rust-lang/rust-clippy/
[criterion]: https://github.com/bheisler/criterion.rs
[crossbeam]: https://github.com/crossbeam-rs/crossbeam
[plan]: ./docs/lesson-plan.md
[the roadmap]: ./docs/roadmap.md
[post-project surveys]: ./docs/lesson-plan.md#user-content-making-pna-rust-better
[pre]: ./docs/prerequisites.md
[rustfmt]: https://github.com/rust-lang/rustfmt/
[serde]: https://github.com/serde-rs/serde
[sp]: https://en.wikipedia.org/wiki/System_programming
[Rust]: https://www.rust-lang.org/
<!-- Badges -->
[irc]:          https://webirc.hackint.org/#irc://irc.hackint.org/#lar
[issue]: https://img.shields.io/github/issues/lar-rs/edinburgh?style=flat-square
[min-rust-badge]: https://img.shields.io/badge/rustc-1.38+-blue.svg
