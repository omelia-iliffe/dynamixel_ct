# Dynamixel CT ChangeLog

## [unreleased]
### Features
- **(generator)** Switch source to ROBOTIS docs platform
- **(ct)** [**breaking**] Add unit and scale to register data
- **(generator)** Derive register units from the docs unit column
- **(ct)** Add access and area to register data
- **(ct)** Add indirect register blocks and sync helpers
### Bug Fixes
- [**breaking**] Incorrect XM335_T323 model number
- **(ct)** Use comma-separated model names in model! macro
### Refactor
- **(generator)** Simplify parsing and generation
- **(generator)** Drop unused control_table_from_model
### Miscellaneous Tasks
- Update readme generation section
- Add git-cliff config and regenerate changelog

## [0.6.0] - 2026-02-18
### Features
- [**breaking**] Add `RegisterError`
- **(ct)** Added XM335_T323 support
### Bug Fixes
- **(ct)** Formating inside `model!` macro
- **(ct)** Move `control_table_from_model_group` behind `std` feature
- **(generator)** Improve model handling
### Miscellaneous Tasks
- Update readme, release v0.6.0

## [0.5.0] - 2025-04-14
### Features
- [**breaking**] Add `#[non_exhaustive]` to `Model` and `ModelGroup`
### Refactor
- Expose register data as associated consts instead of a const fn

## [0.4.0] - 2025-03-31
### Bug Fixes
- Fix `no_std` builds

## [0.3.0] - 2025-03-21
### Features
- Add `no_std` support behind a `std` feature
### Refactor
- Restructure the model types to prepare for `no_std`

## [0.2.0] - 2025-03-11
### Features
- Split the workspace into `dynamixel_registers` and `dynamixel_ct` crates
- Add the `generate_control_tables` crate to scrape control tables
- Support multiple models per `model!` invocation
- Add `ControlTable` conversions from `Model` and `ModelGroup`
- **(ct)** Add `ModelGroup::contains`
- Add the `debug_full_ct` feature
- Add `serde` support for `ControlTable`
- Derive `Display`, `Ord`, `PartialOrd` and `EnumString` for `Register`
### Bug Fixes
- Replace the model error enum with an `UnknownModel` struct
- Deserialize `Model` by enum name instead of numeric repr

## [0.1.1] - 2024-11-20
### Bug Fixes
- Fix `serde` deserialization of signed ints for `Model`
### Testing
- Add toml support test

## [0.1.0] - 2024-11-20
### Features
- Initial release
- Support for XM430, XM540, XC330 and YM070 models
