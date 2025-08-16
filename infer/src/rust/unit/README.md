### Folder Structure

The `programs` directory contains subfolders for different Rust constructs (e.g., `operands`, `statements`, `rvalues`, etc.).

Inside each test subfolder, there are **four files** for each test case:

* **`.mir`** - Final ULLBC before control flow reconstruction.
* **`.rs`** — The Rust program being tested.
* **`.sil`** — The expected Textual IR output for comparison.
* **`.ullbc`** — The output from Charon after processing the MIR of the Rust program.

Additionally, there is an .ml file in each subfolder that handles running the tests for that subfolder. 

---

### Generating `.ullbc` Files

The `.ullbc` file for a given Rust program is generated through the following command:

```
RUSTFLAGS="-A warnings" path-to-charon rustc --ullbc -- path-to-rust-program      
```

---

### Generating `.mir` Files

```
RUSTFLAGS="-A warnings" path-to-charon rustc --print-ullbc -- path-to-rust-program
```

---

### Running the Tests
To run all tests:

```bash
dune runtest
```