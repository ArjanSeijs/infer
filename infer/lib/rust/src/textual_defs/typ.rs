/*
[OCaml Definition]
module Typ : sig
  type t =
    | Int  (** integer type *)
    | Float  (** float type *)
    | Null
    | Void  (** void type *)
    | Fun of function_prototype option  (** function type *)
    | Ptr of t  (** pointer type *)
    | Struct of TypeName.t  (** structured value type name *)
    | Array of t  (** array type *)
  [@@deriving equal]

  and function_prototype = {params_type: t list; return_type: t} [@@deriving equal]

  val pp : F.formatter -> t -> unit

  type annotated = {typ: t; attributes: Attr.t list}

  val is_annotated : f:(Attr.t -> bool) -> annotated -> bool

  val pp_annotated : F.formatter -> annotated -> unit

  val mk_without_attributes : t -> annotated
end
 */

use stable_mir::{mir::LocalDecl, ty::{RigidTy, TyKind}, CrateDef};

use crate::textual_defs::{attr, typename, basetypename, PrintTextual, PrintTextualWithSeperator};

#[derive(Debug,Clone)]
pub enum Typ {
    Int,
    Float,
    Null,
    Void,
    Fun(Option<Box<FunctionPrototype>>),
    Ptr(Box<Typ>),
    Struct(typename::TypeName),
    Array(Box<Typ>),
}

#[derive(Debug,Clone)]
pub struct FunctionPrototype {
    pub params_type: Vec<Typ>,
    pub return_type: Typ,
}

#[derive(Debug)]
pub struct Annotated {
    pub typ: Typ,
    pub attributes: Vec<attr::Attr>,
}

impl PrintTextual for Typ {
    fn pp(&self) -> String {
        match self {
            Typ::Int => String::from("int"),
            Typ::Float => String::from("float"),
            Typ::Null => String::from("null"),
            Typ::Void => String::from("void"),
            Typ::Fun(prototype) => match prototype {
                Some(prototype) => {
                    let param_types = prototype
                        .params_type.pp_comma_list();
                    let ret_type = prototype.return_type.pp();
                    format!("(fun ({}) -> {})", param_types, ret_type)
                }
                None => String::from("(fun _ -> _)"),
            },
            Typ::Ptr(t) => format!("*{}", t.pp()),
            Typ::Struct(t) => format!("{}", t.pp()),
            Typ::Array(t) => format!("array<{}>", t.pp()),
        }
    }
}

impl PrintTextual for Annotated {
    fn pp(&self) -> String {
        self.typ.pp()
    }
}

pub fn kind_to_textual(kind: &stable_mir::ty::TyKind) -> Typ {
    match kind {
        stable_mir::ty::TyKind::RigidTy(rigid_ty) => match rigid_ty {
            RigidTy::Bool | RigidTy::Char | RigidTy::Int(_) | RigidTy::Uint(_) => Typ::Int,
            RigidTy::Float(_) => Typ::Float,
            RigidTy::Array(ty, _) => Typ::Array(Box::new(kind_to_textual(&ty.kind()))),
            RigidTy::RawPtr(ty, _) => Typ::Ptr(Box::new(kind_to_textual(&ty.kind()))),
            RigidTy::Ref(_, ty, _) => Typ::Ptr(Box::new(kind_to_textual(&ty.kind()))),
            RigidTy::Tuple(items) => {
                if items.is_empty() {
                    Typ::Void
                } else {
                    // TODO: We need to differentiate between tuple and struct, for now returns a dummy value
                    Typ::Struct(typename::TypeName::new(basetypename::BaseTypeName::from_string("dummy-tuple".to_string())))
                }
            },
            RigidTy::Adt(def, _) => {
                let base_name = basetypename::BaseTypeName::from_string(def.name());
                let type_name = typename::TypeName::new(base_name);
            
                Typ::Struct(type_name)
            }
            s => todo!("kind_to_textual: {:?}", s),
        },
        s => todo!("kind_to_textual: {:?}", s),
    }
}

pub fn local_decl_to_type(local_decl: &LocalDecl) -> Typ {
    let typ = kind_to_textual(&local_decl.ty.kind());
    typ
}

pub fn local_decl_to_annotated_typ(local_decl: &LocalDecl) -> Annotated {
    Annotated {
        typ: local_decl_to_type(local_decl),
        attributes: vec![],
    }
}
