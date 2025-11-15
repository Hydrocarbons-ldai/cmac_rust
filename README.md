# cmac_rust

CMAC (Cerebellar Model Articulation Controller) utilities for imputing missing `f64`
observations that are marked as `NaN` values.

## Features

- Predict missing samples in `f64` sequences via CMAC receptive fields.
- Configurable learning rate and epoch count for simple experimentation.
- Ergonomic `Cmac` type re-exported as `CMAC` for backward compatibility.
- Ships with unit tests, doctests, and an executable example for quick checks.

## Installation

Add the crate to your `Cargo.toml`:

```toml
cmac_rust = "0.1"
```

## Quick Start

```rust
use cmac_rust::Cmac;
use std::f64;

let data = vec![1.0, f64::NAN, 3.0, f64::NAN, 5.0];
let mut cmac = Cmac::new(data, 2);
let imputed = cmac.impute(0.5, 4);

assert!(!imputed[1].is_nan());
assert!(!imputed[3].is_nan());
```

## Examples

Run the bundled example to see the imputer in action:

```bash
cargo run --example impute
```

## Documentation

You can find the API documentation on [docs.rs](https://docs.rs/cmac_rust).

## Contributing

Contributions are welcome! Please open issues or pull requests on GitHub.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
