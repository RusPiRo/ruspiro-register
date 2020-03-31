# Register abstraction crate for the RusPiRo kernel

This crate provides easy to use and compile time safe access abstraction to MMIO (memory mapped input output) registers of the Raspberry Pi.
This crate also provides definitions for some aarch64 and aarch32 cp15 system register. Whether the aarch64 or aarch32 register are available depends on the target architecture used while building this crate.

[![Travis-CI Status](https://api.travis-ci.org/RusPiRo/ruspiro-register.svg?branch=master)](https://travis-ci.org/RusPiRo/ruspiro-register)
[![Latest Version](https://img.shields.io/crates/v/ruspiro-register.svg)](https://crates.io/crates/ruspiro-register)
[![Documentation](https://docs.rs/ruspiro-register/badge.svg)](https://docs.rs/ruspiro-register)
[![License](https://img.shields.io/crates/l/ruspiro-register.svg)](https://github.com/RusPiRo/ruspiro-register#license)

## Usage

To use this crate simply add the dependency to your ``Cargo.toml`` file:
```
[dependencies]
ruspiro-register = "0.4"
```

In any rust file the register could be defined with their access type, size, address and optional a detailed field definition.
The register access types are ``ReadOnly``, ``WriteOnly`` and ``ReadWrite``. The supported register sizes are `u8`, ``u16``, ``u32``, ``u64``.

```
use ruspiro_register::*;

define_mmio_register! [
    RO_REGISTER<ReadOnly<u8>@(0xFF00_0000)>,
    WO_REGISTER<WriteOnly<u16>@(0xFF00_0004)> {
        FLAG1   OFFSET(0) BITS(2),
        FLAG2   OFFSET(2),
        FLAG3   OFFSET(3) BITS(4)
    },
    RW_REGISTER<ReadWrite<u32>@(0xFF00_0008)> 
];

fn main() {
    let _ = RO_REGISTER::Register.get(); // read raw value from register

    WO_REGISTER::Register.write_value(WO_REGISTER::FLAG3::with_value(0b1010)); // write only field FLAG3 into the register
    
    RW_REGISTER::Register.set(0xFFF); // write raw value to register
}
```

If access to the system registers is needed this could be done like so:
```
use ruspiro_register::system::*;

fn main64() {
    // update the system control register of EL2 in aarch64 mode to deactivate the MMU
    sctlr_el2::write(
        sctlr::M::DISABLE
    );
}

fn main32() {
    // update the system control register in aarch32 mode to deactivate the MMU
    sctlr::write(
        sctlr::M::DISABLE
    );
}
```

## License
Licensed under Apache License, Version 2.0, ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)