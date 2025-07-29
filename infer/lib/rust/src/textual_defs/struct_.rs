use crate::textual_defs::{attr::{self, Attr}, fielddecl::{self, FieldDecl}, typename::{self, TypeName}, PrintTextual};

/*
[OCaml Definition]
module Struct : sig
  type t =
    {name: TypeName.t; supers: TypeName.t list; fields: FieldDecl.t list; attributes: Attr.t list}
*/

#[derive(Debug)]
pub struct Struct {
    pub name: TypeName,
    pub supers: Vec<TypeName>,
    pub fields: Vec<FieldDecl>,
    pub attributes: Vec<Attr>
}

impl PrintTextual for Struct {
  fn pp(&self) -> String {
      let mut out = String::new();

      for attr in &self.attributes {
          out.push_str(&format!("{} ", attr.pp()));
      }

      out.push_str(&self.name.pp());

      if !self.supers.is_empty() {
          let supers_text = self.supers.iter()
              .map(|s| s.pp())
              .collect::<Vec<_>>()
              .join(", ");
          out.push_str(&format!(" extends {}", supers_text));
      }

      let field_text = self.fields.iter()
          .map(|f| f.pp())
          .collect::<Vec<_>>()
          .join(";\n  ");

      out.push_str(" = {\n  ");
      out.push_str(&field_text);
      out.push_str("\n}");

      out
  }
}
