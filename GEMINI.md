# Project Overview

`round_view_derive` is a Rust procedural macro crate that provides derive macros
for generating "view" methods (`r2_view`, `r3_view`, and `r4_view`) for structs.
These methods return specific view types `RoundTwoView`, `RoundThreeView` and `RoundFourView`.

The macros expect the target struct to have specific fields:

- `rng_combs_r1`, `rng_combs_r2`, `rng_combs_r3`, `rng_combs_r4`
- `play`

Each derive macro (`R2View`, `R3View`, `R4View`) implements a method
that gathers references (some immutable, some mutable) to these fields into a view struct.

## Technologies

- **Language:** Rust (Edition 2024)
- **Core Libraries:** `syn` (v2.0), `quote` (v1.0), `proc-macro`

# Building and Running

Since this is a library crate containing procedural macros,
it is primarily used as a dependency.

- **Build:** `cargo build`
- **Test:** `cargo test` (Note: No tests were found in the `src` directory;
  tests might be in an integration test directory if added later).
- **Check:** `cargo check`

# Development Conventions

- **Macro Implementation:** Uses `syn` for parsing and `quote` for code generation.
- **Field Assumptions:** The macros assume the existence of specific fields (`rng_combs_rX` and `play`)
  on the struct they are derived for.
- **Inlining:** Generated methods are marked with `#[inline(always)]`.
- **Naming Convention:**
  - `R2View` derive generates `r2_view() -> RoundTwoView`
  - `R3View` derive generates `r3_view() -> RoundThreeView`
  - `R4View` derive generates `r4_view() -> RoundFourView`
