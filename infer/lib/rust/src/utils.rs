use stable_mir::mir::{UnOp, BinOp};
use crate::textual_defs::typ::{Typ, IntKind};

pub fn fresh_id(i: usize) -> String {
    format!("var_{i}")
}

pub fn bytes_to_int(bytes: &Vec<Option<u8>>) -> i128 {
    bytes
        .iter()
        .enumerate()
        .map(|(i, b)| (b.unwrap_or_default() as i128) << i)
        .sum::<i128>()
}

pub fn bytes_to_float(bytes: &[Option<u8>]) -> f64 {
    let raw_bytes: Vec<u8> = bytes.iter().map(|b| b.unwrap_or(0)).collect();

    match raw_bytes.len() {
        8 => {
            let mut arr = [0u8; 8];
            arr.copy_from_slice(&raw_bytes[..8]);
            f64::from_bits(u64::from_le_bytes(arr))
        },
        4 => {
            let mut arr = [0u8; 4];
            arr.copy_from_slice(&raw_bytes[..4]);
            f32::from_bits(u32::from_le_bytes(arr)) as f64
        },
        n => {
            todo!("Float constant has unexpected byte size: {}", n);
        }
    }
}

pub fn bytes_to_cstr(bytes: &[Option<u8>]) -> String {
    let raw_bytes: Vec<u8> = bytes.iter().map(|b| b.unwrap_or(0)).collect();

    raw_bytes
        .iter()
        .take_while(|&&b| b != 0) // Stop at first null
        .map(|&b| b as char)
        .collect()
}

pub fn unop_to_proc_name(op: UnOp, typ: &Typ) -> String {
    use stable_mir::mir::UnOp::*;

    match (op, typ) {
        // Arithmetic: Neg
        (Neg, Typ::Int(_)) => "__sil_neg".into(),
        (Neg, Typ::Float) => "__sil_neg".into(), 

        // Boolean: Not
        (Not, Typ::Int(IntKind::Bool)) => "__sil_lnot".into(), 
        (Not, Typ::Int(_)) => "__sil_bnot".into(),      

        _ => todo!("unsupported unop/type combo: {:?}, {:?}", op, typ),
    }
}

pub fn binop_to_proc_name(op: BinOp, typ: &Typ) -> String {
    use stable_mir::mir::BinOp::*;

    match (op, typ) {
        // Arithmetic: Add
        (Add, Typ::Int(IntKind::I8)) | (Add, Typ::Int(IntKind::Char)) => "__sil_plusa_char".into(),
        (Add, Typ::Int(IntKind::I16)) => "__sil_plusa_short".into(),
        (Add, Typ::Int(IntKind::I32)) => "__sil_plusa_int".into(),
        (Add, Typ::Int(IntKind::I64)) => "__sil_plusa_long".into(),
        (Add, Typ::Int(IntKind::I128)) => "__sil_plusa_128".into(),
        (Add, Typ::Int(IntKind::U8)) => "__sil_plusa_uchar".into(),
        (Add, Typ::Int(IntKind::U16)) => "__sil_plusa_ushort".into(),
        (Add, Typ::Int(IntKind::U32)) => "__sil_plusa_uint".into(),
        (Add, Typ::Int(IntKind::U64)) => "__sil_plusa_ulong".into(),
        (Add, Typ::Int(IntKind::U128)) => "__sil_plusa_u128".into(),
        (Add, Typ::Int(IntKind::Bool)) => "__sil_plusa_bool".into(),

        // Arithmetic: Sub
        (Sub, Typ::Int(IntKind::I8)) | (Sub, Typ::Int(IntKind::Char)) => "__sil_minusa_char".into(),
        (Sub, Typ::Int(IntKind::I16)) => "__sil_minusa_short".into(),
        (Sub, Typ::Int(IntKind::I32)) => "__sil_minusa_int".into(),
        (Sub, Typ::Int(IntKind::I64)) => "__sil_minusa_long".into(),
        (Sub, Typ::Int(IntKind::I128)) => "__sil_minusa_128".into(),
        (Sub, Typ::Int(IntKind::U8)) => "__sil_minusa_uchar".into(),
        (Sub, Typ::Int(IntKind::U16)) => "__sil_minusa_ushort".into(),
        (Sub, Typ::Int(IntKind::U32)) => "__sil_minusa_uint".into(),
        (Sub, Typ::Int(IntKind::U64)) => "__sil_minusa_ulong".into(),
        (Sub, Typ::Int(IntKind::U128)) => "__sil_minusa_u128".into(),
        (Sub, Typ::Int(IntKind::Bool)) => "__sil_minusa_bool".into(),

        // Arithmetic: Mul
        (Mul, Typ::Int(IntKind::I8)) => "__sil_mult_char".into(),
        (Mul, Typ::Int(IntKind::Char)) => "__sil_mult_char".into(),
        (Mul, Typ::Int(IntKind::I16)) => "__sil_mult_short".into(),
        (Mul, Typ::Int(IntKind::I32)) => "__sil_mult_int".into(),
        (Mul, Typ::Int(IntKind::I64)) => "__sil_mult_long".into(),
        (Mul, Typ::Int(IntKind::I128)) => "__sil_mult_128".into(),
        (Mul, Typ::Int(IntKind::U8)) => "__sil_mult_uchar".into(),
        (Mul, Typ::Int(IntKind::U16)) => "__sil_mult_ushort".into(),
        (Mul, Typ::Int(IntKind::U32)) => "__sil_mult_uint".into(),
        (Mul, Typ::Int(IntKind::U64)) => "__sil_mult_ulong".into(),
        (Mul, Typ::Int(IntKind::U128)) => "__sil_mult_u128".into(),
        (Mul, Typ::Int(IntKind::Bool)) => "__sil_mult_bool".into(),

        // Division
        (Div, Typ::Int(_)) => "__sil_divi".into(),
        (Div, Typ::Float) => "__sil_divf".into(),

        // Modulo
        (Rem, _) => "__sil_mod".into(),

        // Shifts
        (Shl, _) => "__sil_shiftlt".into(),
        (Shr, _) => "__sil_shiftrt".into(),

        // Comparisons (type-agnostic)
        (Lt, _) => "__sil_lt".into(),
        (Gt, _) => "__sil_gt".into(),
        (Le, _) => "__sil_le".into(),
        (Ge, _) => "__sil_ge".into(),
        (Eq, _) => "__sil_eq".into(),
        (Ne, _) => "__sil_ne".into(),

        // Bitwise
        (BitAnd, _) => "__sil_band".into(),
        (BitOr, _) => "__sil_bor".into(),
        (BitXor, _) => "__sil_bxor".into(),

        // Pointer arithmetic
        (Add, Typ::Ptr(_)) => "__sil_pluspi".into(),     // Add pointer + int
        (Sub, Typ::Ptr(_)) => "__sil_minuspi".into(),    // Sub pointer - int
        (Sub, Typ::Struct(_)) => "__sil_minuspp".into(), // Sub ptr - ptr (assuming ptr-like Struct)

        _ => todo!("unsupported binop/type combo: {:?}, {:?}", op, typ),
    }
}
