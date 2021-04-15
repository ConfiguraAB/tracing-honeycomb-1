[![tracing-distributed on crates.io](https://img.shields.io/crates/v/tracing-distributed)](https://crates.io/crates/tracing-distributed)
[![Documentation (latest release)](https://docs.rs/tracing-distributed/badge.svg)](https://docs.rs/tracing-distributed/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](../LICENSE)

# tracing-distributed

Current version: 0.3.1

This crate provides:
- `TelemetryLayer`, a generic tracing layer that handles publishing spans and events to arbitrary backends
- Utilities for implementing distributed tracing for arbitrary backends

As a tracing layer, `TelemetryLayer` can be composed with other layers to provide stdout logging, filtering, etc.

This crate is primarily intended to be used by people implementing their own backends.
A concrete implementation using honeycomb.io as a backend is available in the [`tracing-honeycomb` crate](https://crates.io/crates/tracing-honeycomb).

## License

MIT

<!--
README.md is generated from README.tpl by cargo readme. To regenerate:
cargo install cargo-readme
cargo readme > README.md
-->
