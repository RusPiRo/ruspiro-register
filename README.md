# RusPiRo Register

The crate provides the definitions to conviniently work with register field values that are typically presented by a set of bit fields.

[![Travis-CI Status](https://api.travis-ci.org/RusPiRo/ruspiro-register.svg?branch=release)](https://travis-ci.org/RusPiRo/ruspiro-register)
[![Latest Version](https://img.shields.io/crates/v/ruspiro-register.svg)](https://crates.io/crates/ruspiro-register)
[![Documentation](https://docs.rs/ruspiro-register/badge.svg)](https://docs.rs/ruspiro-register)
[![License](https://img.shields.io/crates/l/ruspiro-register.svg)](https://github.com/RusPiRo/ruspiro-register#license)

## Usage

To use this crate simply add the dependency to your ``Cargo.toml`` file:

```toml
[dependencies]
ruspiro-register = "0.5.0"
```

A single register field is specified with its bit mask and the bit shift. The `RegisterField` structure can be instantiated for the types `u8`, `u16`, `u32` and `u64`.

```rust
use ruspiro_register::*;

fn main() {
    let field = RegisterField::<u32>::new(0x3, 6);
}
```

To represent a specific value of a register field the `RegisterFieldValue` structure is used. It is available for the same scalar types as the `RegisterField`: `u8`, `u16`, `u32` and `u64`.

```rust
use ruspiro_register::*;

fn main() {
    let field = RegisterField::<u8>::new(0x3, 2);
    // the value to the regsiter field will be shifted and masked internally
    // so it will be provided without any shifting
    let value = RegisterFieldValue::<u8>::new(field, 0b10);
    println!("{:?}", value);
}
```

The register field value printed will look like this then:

```text
RegisterFieldValue { field: RegisterField {
    Bits: [3:2]
    Mask: 0b1100
}, value: 2, raw_value: 8 }
```

It is quite unlikely those definitions will be directly used as the represantation of a full register with its fields depends on the type of the register and the implementation of the functions to modify the register contents.

Typically macros will be used to reduce the complexity of the register definitions. Examples can be seen in the [ruspiro-mmio-register](https://crates.io/crates/ruspiro-mmio-register) and the [ruspiro-arch-aarch64](https://crates.io/crates/ruspiro-arch-aarch64) crates.

## License

Licensed under Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0) or MIT ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)) at your choice.