/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Apache License 2.0
 **********************************************************************************************************************/

//! # Aarch64 System Register
//! Aarch64 register definitions are only available when compiled for **Aarch64** target architecture

pub mod actlr_el2;
pub mod actlr_el3;
pub mod ccsidr_el1;
pub mod clidr_el1;
pub mod cpacr_el1;
pub mod currentel;
pub mod esr_el1;
pub mod esr_el2;
pub mod esr_el3;
pub mod hcr_el2;
pub mod mair_el1;
pub mod mair_el2;
pub mod mpidr_el1;
pub mod sctlr_el1;
pub mod sctlr_el2;
pub mod tcr_el1;
pub mod tcr_el2;
pub mod ttbr0_el1;
pub mod ttbr0_el2;
pub mod ttbr1_el1;
pub mod vbar_el1;
pub mod vbar_el2;
