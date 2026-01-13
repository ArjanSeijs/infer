# Current Supported Features

## Running rust

The translation of rust is done through the library of  [charon](https://github.com/AeneasVerif/charon) which is internally called by infer.

In short to run a rust program in infer can be done by the following command:
`infer <infer commands> -- rustc <charon commands> -- <rust commands>`

For example, to run a simple file you can be done by `infer -- rustc -- file.rs` or `infer -- rustc -- file.rs --crate-type=lib` for lib type files.



## Supported Features

Currently only a small core language is supported with support for references and raw pointers.

* References,
* Raw pointers,
* Primitives (ints/ bools/ floats/ unit)
* Control flow (while/ if)
* Tuples, 
* Arrays, 
* Structs
* Basic Operands (addition etc)
* Function Definitions


### Error Detection
* Null Pointers
* Lifetimes 
* Stack Escapes

## Partially supported
* Trait functions, only those disambugated by mir/charon

## Limitations

* Closures / Functions as values
* Dynamics
* Enum Support
* Union Support
* Opaque Types:
  * Types from external crates, some can be included with the charon commands `--extract-opaque-bodies` or `--include=<namespace::function>`
  * Internal types marked as opaque e.g. `fn foo() -> impl Trait`
* Panics/unwind
* Slices
