# Changelog

## :peach: v0.5.1

This is mainly a maintenance version migrating the build pipeline to GitHub Actions.

- ### :wrench: Maintenance

  - move CI/CD to github actions
  - define a custom build target that suites RaspBerry Pi Aarch64

## :peach: v0.5.0

  This version contains a major refactoring. The whole crate has been split into actually 3 crates in total. The `ruspiro-register` crate continues to contain the definitions of `RegisterType`, `RegisterField` and `RegisterFieldValue` only. Those definitions are highly generic and can be usefull in various other crates that do not require the other functionality originally beeing part of this crate as well.

  > The system register specific functions and API is moved into the `ruspiro-arch-aarch64` crate.
  >
  > The MMIO register specific functions and API is moved into the `ruspiro-mmio-register` crate.

  Both crates utilizes this one for the register type and field definitions. 

  - ### :wrench: Maintenance

    - Whole refactoring of the crate.
    - Introduce a proper Travis-CI pipeline setting to support the lifecycle of the crate incl. publishing to crates.io

## :banana: v0.4.3
  - ### :detective: Fixes
    - remove `asm!` macro usages and replace with `llvm_asm!`
    - use `cargo make` to stabilize cross-platform builds
    
## :pizza: v0.4.2
  - ### :detective: Fixes
    - Fix the offset values in the AARCH32 DACR register definition.
    
## :pizza: v0.4.1
  - ### :detective: Fixes
    - Fix issue that only single line comments were allowed in ``define_mmio_register!`` macro expansion
    
## :pizza: v0.4.0
  - ### :bulb: Features
    1. Enable MMIO register definitions to set visibility in defining crate.<br>
    2. Add new aarch64 system register:
      - ACTLR_EL2
      - ACTLR_EL3
      - CCSIDR_EL1
      - CLIDR_EL1
      - ESR_EL1
      - ESR_EL2
      - ESR_EL3
      - MPIDR_EL1
      - VBAR_EL1
      - VBAR_EL2
      
  - ### :detective: Fixes
    
  - ### :wrench: Maintenance
    1. Streamline all system register field names to be UPPER case
    2. Update register field value assignments in the macro to use **=** instead of **:**
    
  - ### :book: Documentation
    - Usage of register definition macros now supports propper documentation of the items during macro
    expansion.
    - Started to provide system register documentation, this is an ongoing task for the upcomming releases
