This folder contains automated tests for converting Rust programs into Textual IR using `RustMir2TextualTest.exe`.

### Folder Structure

The `programs` directory contains subfolders for different Rust constructs (e.g., `operands`, `statements`, `rvalues`, etc.).

Inside each test subfolder, there are **three files** for each test case:

* **`.rs`** — The Rust program being tested.
* **`.ullbc`** — The output from Charon when processing the MIR of the Rust program.
* **`.sil`** — The expected Textual IR output for comparison.

---

### Generating `.ullbc` Files

The `.ullbc` file for a given Rust program is generated through the following command:

```
charon --ullbc --no-cargo path/to/program.rs -o path/to/output.ullbc
```

---

### Running the Tests

First, navigate to the `unit` folder:

```bash
cd infer/src/rust/unit
```

Build the test runner:

```bash
dune build ./RustMir2TextualTest.exe
```

---

#### Run **all** tests:

```bash
dune exec ./RustMir2TextualTest.exe
```

---

#### Run tests for a specific subfolder:

You can filter tests by setting the `RUN_UNDER` environment variable to the relative path under `programs`.

For example, to run only the tests in `operands/const`:

```bash
RUN_UNDER=operands/const dune exec ./RustMir2TextualTest.exe
```

---

### Output

* Results and diffs are printed directly to the terminal.
* Output statuses:

  * `OK` — Output matches the expected `.sil`
  * `MISS` — Missing `.sil` file
  * `DIFF` — Output differs from the expected `.sil`
  * `ERR` — Error occurred during translation
