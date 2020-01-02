# Changelog
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
