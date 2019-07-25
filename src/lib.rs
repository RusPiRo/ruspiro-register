/*********************************************************************************************************************** 
 * Copyright (c) 2019 by the authors
 * 
 * Author: André Borrmann 
 * License: Apache License 2.0
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-register/0.0.0")]
#![no_std]
#![feature(const_fn)]

//! # RusPiRo MMIO register abstraction
//! 
//! The crate provides a hopefully simple to use compiletime type safe abstraction of MMIO registers of the Raspberry Pi.
//! 
//! # Usage
//! 
//! To use the crate add the following dependency to your Cargo.toml file:
//! ```
//! [dependencies]
//! ruspiro-register = { git = 'https://github.com/RusPiRo/ruspiro-register' }
//! ```
//! 
//! In your code you can define register as follows:
//! ```
//! use ruspiro_register::*;
//! 
//! // define a single register without any specific fields, like the free running system timer counter low value
//! // of the Raspberry Pi 3. Valid register size types are u8, u16, u32, u64.
//! define_register! {
//!    TIMERCLO: ReadOnly<u32>, 0x3F00_3004 => []
//! };
//! 
//! // define a list of registers that may ore may not contain a specific field configuration
//! define_registers! [
//!     TIMERCHI: ReadOnly<u32>, 0x3F00_3008 => [],
//!     I2C_C: ReadWrite<u32>,   0x3F80_4000 => [
//!         ENABLE     OFFSET(15),
//!         IRQ_RX     OFFSET(10),
//!         IRQ_TX     OFFSET(9),
//!         IRQ_DONE   OFFSET(8),
//!         STARTTRANS OFFSET(7),
//!         CLEAR      OFFSET(4) BITS(2),
//!         READ       OFFSET(1),
//!         WRITE      OFFSET(0)
//!     ]
//! ];
//! ```
//! With the name of the register given at the definition the contents and the fields are accessible like so:
//! ```
//! fn main() {
//!     let _ = TIMERCLO::Register.get(); // get the raw register value
//!     let _ = I2C_C::Register.read(I2C_C::ENABLE); // get the value of the requested register field
//!     I2C_C::Register.modify(I2C_C::CLEAR, 0x1); // update a specific field value of the register
//! }
//! ```
//! 

pub mod macros;
pub use self::macros::*;

pub mod register;
pub use self::register::*;