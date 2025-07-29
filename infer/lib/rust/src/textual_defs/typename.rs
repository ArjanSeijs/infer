use crate::textual_defs::{basetypename::BaseTypeName, PrintTextual};

/*
[OCaml Definition]
module TypeName : sig
  (* structured value type name *)
  type t = {name: BaseTypeName.t; args: t list} [@@deriving compare, equal, hash]
*/

#[derive(Debug,Clone)]
pub struct TypeName {
    name: BaseTypeName,
    args: Vec<TypeName>,
}

impl TypeName {
  pub fn new(name: BaseTypeName) -> Self {
      TypeName {
          name,
          args: Vec::new(),
      }
  }

  pub fn with_args(name: BaseTypeName, args: Vec<TypeName>) -> Self {
      TypeName { name, args }
  }
}

impl PrintTextual for TypeName {
    fn pp(&self) -> String {
        if self.args.is_empty() {
            self.name.pp()
        } else {
            let args_str = self
                .args
                .iter()
                .map(|arg| arg.pp())
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}<{}>", self.name.pp(), args_str)
        }
    }
}
