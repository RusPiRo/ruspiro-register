/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-register/0.4.3")]
#![no_std]
#![feature(llvm_asm, const_fn, doc_cfg)]

//! # System and MMIO register access
//! ## System register access
//!
//! Definitions and simple access to the system registers used in the different RusPiRo crates. Depending
//! on the target architecture the crate will be build for it contains the definitions of the Aarch64 or the
//! Aarch32 CP15 system register.
//!
//! ## Usage
//!
//! ```no_run
//! use ruspiro_register::system::*;
//!
//! #[cfg(target_arch="aarch64")]
//! fn enable_mmu() {
//!     // update the system control register in aarch64 to enable caching and the MMU in EL1
//!     sctlr_el1::write(
//!         sctlr_el1::M::ENABLE | // MMU
//!         sctlr_el1::C::ENABLE | // data cache
//!         sctlr_el1::I::ENABLE   // instruction cache
//!     );
//! }
//!
//! #[cfg(target_arch="arm")]
//! fn enable_mmu() {
//!     // update the system control register in aarch32 to enable caching and the MMU
//!     sctlr::write(
//!         sctlr::M::ENABLE | // MMU
//!         sctlr::C::ENABLE | // data cache
//!         sctlr::I::ENABLE   // instruction cache
//!     );
//! }
//! ```
//!
//! ## MMIO register abstraction
//!
//! Simple to use and compile time type safe abstraction of memory mapped (MMIO) registers of the
//! Raspberry Pi.
//!
//! ## Usage
//!
//! Register defintitions are simple and straight forward using the macros provided by this crate.
//! In your code you can define registers like this:
//! ```no_run
//! use ruspiro_register::*;
//!
//! // define a single register without any specific fields, like the free running system timer counter low value
//! // of the Raspberry Pi 3. Valid register size types are u8, u16, u32, u64.
//! define_mmio_register!( TIMERCLO<ReadOnly<u32>@(0x3F00_3004)> );
//!
//! // define a list of registers that may or may not contain a specific field configuration
//! define_mmio_register! [
//!     /// High 32Bit's of the free-running counter
//!     TIMERCHI<ReadOnly<u32>@(0x3F00_3008)>,
//!     /// The I²C control register
//!     I2C_C<ReadWrite<u32>@(0x3F80_4000)> {
//!         /// Flag to enable the I²C peripheral
//!         ENABLE     OFFSET(15),
//!         IRQ_RX     OFFSET(10),
//!         IRQ_TX     OFFSET(9),
//!         IRQ_DONE   OFFSET(8),
//!         STARTTRANS OFFSET(7),
//!         CLEAR      OFFSET(4) BITS(2),
//!         READ       OFFSET(1),
//!         WRITE      OFFSET(0)
//!     }
//! ];
//!
//! // With the name of the register given at the definition the contents and the fields are accessible like so:
//!
//! fn test_mmio_register() {
//!     let _ = TIMERCHI::Register.get(); // get the raw register value
//!     let _ = I2C_C::Register.read(I2C_C::ENABLE); // get the value of the requested register field
//!     I2C_C::Register.modify(I2C_C::CLEAR, 0x1); // update a specific field value of the register
//! }
//! ```
//!

mod macros;
pub use self::macros::*;

mod register;
pub use self::register::*;

pub mod system;

#[cfg(test)]
mod tests;
