# Register abstraction crate for the RusPiRo kernel

This Crate provides easy to use and compile time safe access abstraction to MMIO (memory mapped input output) registers of the Raspberry Pi.

[![Travis-CI Status](https://api.travis-ci.org/RusPiRo/ruspiro-register.svg?branch=master)](https://travis-ci.org/RusPiRo/ruspiro-register)
[![Latest Version](https://img.shields.io/crates/v/ruspiro-register.svg)](https://crates.io/crates/ruspiro-register)
[![Documentation](https://docs.rs/ruspiro-register/badge.svg)](https://docs.rs/ruspiro-register)
[![License](https://img.shields.io/crates/l/ruspiro-register.svg)](https://github.com/RusPiRo/ruspiro-register#license)

## Usage

To use this crate simply add the dependency to your ``Cargo.toml`` file:
```
[dependencies]
ruspiro-register = "0.1.0"
```

In any rust file the register could be defined with their access type, size, address and optional a detailed field definition.
The register access types are ``ReadOnly``, ``WriteOnly`` and ``ReadWrite``. The supported register sizes are `u8`, ``u16``, ``u32``, ``u64``.

```
use ruspiro_register::*;

define_registers! [
    RO_REGISTER: ReadOnly<u32>, 0xFF00_0000 => [],
    WO_REGISTER: WriteOnly<u8>, 0xFF00_0004 => [
        FLAG1   OFFSET(0) BITS(2),
        FLAG2   OFFSET(2),
        FLAG3   OFFSET(3) BITS(4)
    ],
    RW_REGISTER: ReadWrite<u16>, 
];

fn main() {
    let _ = RO_REGISTER::Register.get(); // read raw value from register

    WO_REGISTER::Register.write(WO_REGISTER::FLAG3, 0b1010); // write only field FLAG3 into the register
    
    RW_REGISTER::Register.set(0xFFF); // write raw value to register
}
```

## License
Licensed under Apache License, Version 2.0, ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)