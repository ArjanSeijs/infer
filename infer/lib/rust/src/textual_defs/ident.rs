/*
[OCaml Definition]
    module Ident : sig
    type t [@@deriving equal]
*/

use crate::textual_defs::PrintTextual;
use std::sync::atomic::{AtomicU64, Ordering};

static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone)]
pub struct Ident {
    pub val: i128,
}

impl PrintTextual for Ident {
    fn pp(&self) -> String {
        format!("n{}", self.val)
    }
}

impl Ident {
    pub fn fresh() -> Self {
        let id = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        Ident { val: id as i128 }
    }

    pub fn from_name(_s: &str) -> Self {
        Ident::fresh()
    }
}