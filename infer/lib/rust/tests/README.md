# MIR → Textual SIL Tests

This directory contains programs that test the Rust MIR to Textual SIL translation implementations.

Tests are organized into folders based on major MIR concept categories:

- `Rules`
- `Operands`
- `Statements`
- `RValues`
- `Terminator`
- `BasicBlocks`
- `Function`
- `LocalDecls`
- `Types`

Each category resides in a top-level folder named after it. These folders contain subfolders or files corresponding to specific MIR constructs. For example, the `operands` folder contains subfolders such as `const`, `copy`, and `move`. Each subfolder contains test programs targeting that specific MIR concept.

Each test consists of three files sharing the same base name:

- `.rs` — the original Rust source file designed to trigger specific MIR constructs
- `.mir` — the MIR output generated from the Rust file
- `.sil` — the expected output in Textual SIL, written manually

---

## Generating `.mir` Files

To generate MIR for a given test, use the following `rustc` command with nightly Rust:

```sh
rustc +nightly -Z unpretty=mir -Z mir-opt-level=0 --crate-type=lib -C opt-level=3 \
  -A dead_code -A unused_variables -A unused_must_use \
  ./path/to/test_case.rs
