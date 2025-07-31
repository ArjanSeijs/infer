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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntKind {
    Bool,
    Char,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    I128,
    U128,
    Isize,
    Usize,
    ILongLong,
    ULongLong,
}

#[derive(Debug,Clone)]
pub enum Typ {
    Int(IntKind),
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
            Typ::Int(kind) => match kind {
                IntKind::I8 => String::from("char") ,
                IntKind::U8 => String::from("uchar"),
                IntKind::I16 => String::from("short"),
                IntKind::U16 => String::from("ushort"),
                IntKind::I32 => String::from("int"),
                IntKind::U32 => String::from("uint"),
                IntKind::I64 => String::from("long"),
                IntKind::U64 => String::from("ulong"),
                IntKind::I128 => String::from("128"),
                IntKind::U128 => String::from("u128"),
                IntKind::ILongLong => String::from("longlong"),
                IntKind::ULongLong => String::from("ulonglong"),
                IntKind::Isize => String::from("long"),
                IntKind::Usize => String::from("ulong"),
                IntKind::Bool => String::from("bool"),
                IntKind::Char => String::from("char"),
            },
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
    use stable_mir::ty::{IntTy, UintTy, FloatTy, RigidTy, TyKind};

    match kind {
        TyKind::RigidTy(rigid_ty) => match rigid_ty {
            RigidTy::Bool => Typ::Int(IntKind::Bool),
            RigidTy::Char => Typ::Int(IntKind::Char),

            RigidTy::Int(int_ty) => Typ::Int(match int_ty {
                IntTy::I8 => IntKind::I8,
                IntTy::I16 => IntKind::I16,
                IntTy::I32 => IntKind::I32,
                IntTy::I64 => IntKind::I64,
                IntTy::I128 => IntKind::I128,
                IntTy::Isize => IntKind::Isize,
            }),

            RigidTy::Uint(uint_ty) => Typ::Int(match uint_ty {
                UintTy::U8 => IntKind::U8,
                UintTy::U16 => IntKind::U16,
                UintTy::U32 => IntKind::U32,
                UintTy::U64 => IntKind::U64,
                UintTy::U128 => IntKind::U128,
                UintTy::Usize => IntKind::Usize,
            }),

            RigidTy::Float(_) => Typ::Float,
            RigidTy::Array(ty, _) => Typ::Array(Box::new(kind_to_textual(&ty.kind()))),
            RigidTy::RawPtr(ty, _) => Typ::Ptr(Box::new(kind_to_textual(&ty.kind()))),
            RigidTy::Ref(_, ty, _) => Typ::Ptr(Box::new(kind_to_textual(&ty.kind()))),

            RigidTy::Tuple(items) => {
                if items.is_empty() {
                    Typ::Void
                } else {
                    // We can't use struct directly for tuples at the moment although they are similar
                    todo!("Non-empty tuple not implemented");
                    Typ::Void // Temporary dummy value
                }
            }

            RigidTy::Adt(def, _) => {
                let base = basetypename::BaseTypeName::from_string(def.name());
                Typ::Struct(typename::TypeName::new(base))
            }

            other => todo!("Unhandled rigid type: {:?}", other),
        },

        other => todo!("Unhandled TyKind variant: {:?}", other),
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
