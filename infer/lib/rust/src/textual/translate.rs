use std::collections::HashMap;

use stable_mir::{
    mir::{BasicBlock, BinOp, ConstOperand, LocalDecl, Operand, Place, Rvalue, StatementKind, UnOp}, ty::{ConstantKind, FnDef, RigidTy, Span, TyKind}, CrateDef, CrateItem,
};

use crate::textual_defs::{
    boolexp::BoolExp,
    constant::Const,
    exp::{CallKind, Exp},
    instr::Instr,
    location::Location,
    node::Node,
    nodename::NodeName,
    procdecl::ProcDecl,
    procdesc::ProcDesc,
    qualifiedprocname::QualifiedProcName,
    terminator::Terminator,
    typ::{Annotated, Typ, local_decl_to_annotated_typ, local_decl_to_type, kind_to_textual, IntKind},
    varname::VarName,
};

use crate::utils::{fresh_id, bytes_to_int};

type LabelMap = HashMap<usize, String>;
type PlaceMap = HashMap<usize, (String, Typ)>;

pub fn item_to_procdesc(item: &CrateItem) -> ProcDesc {
    let body = item.expect_body();
    let def = item.def_id();
    let name = &def.name();
    let span = item.span();

    let arg_locals = body.arg_locals();
    let ret_local = body.ret_local();
    // let inner_locals = body.inner_locals();
    let blocks = &body.blocks;

    let label_map = blocks_to_labels(blocks);
    let (place_map, locals) = decls_to_locals(body.locals());
    let nodes: Vec<_> = blocks
        .iter()
        .enumerate()
        .map(|(idx, block)| block_to_node(idx, block, &label_map, &place_map))
        .collect();
    ProcDesc {
        proc_decl: to_proc_decl(name, arg_locals, ret_local, span),
        nodes: nodes,
        start: NodeName::new("node_0".to_string(), None),
        params: vec![],
        locals: locals,
        exit_loc: Location::from_span_end(Some(span)),
    }
}

pub fn to_proc_decl(
    name: &String,
    arg_locals: &[LocalDecl],
    ret_local: &LocalDecl,
    span: Span,
) -> ProcDecl {
    let attributes = vec![];
    let result_type = local_decl_to_annotated_typ(ret_local);
    let qualified_name = QualifiedProcName::new(name.clone(), Some(span));
    let formal_types = arg_locals
        .iter()
        .map(|decl| local_decl_to_annotated_typ(decl))
        .collect();
    ProcDecl {
        qualified_name,
        formal_types,
        result_type,
        attributes,
    }
}

fn blocks_to_labels(blocks: &Vec<BasicBlock>) -> LabelMap {
    blocks
        .iter()
        .enumerate()
        .map(|(i, _)| (i, format!("node_{}", i)))
        .collect()
}

fn block_to_node(
    idx: usize,
    block: &BasicBlock,
    label_map: &LabelMap,
    place_map: &PlaceMap,
) -> Node {
    let statements = &block.statements;
    let terminator = &block.terminator;

    let value = label_map.get(&idx).unwrap();
    let label = NodeName::new(value.clone(), statements.get(0).map(|s| s.span));

    let ssa_parameters = vec![];
    let exn_succs = vec![];

    let mut instrs: Vec<Instr> = statements
        .iter()
        .flat_map(|stmt| statement_to_instr(stmt, &place_map))
        .collect();
    let (instrs2, last) = terminator_to_textual(terminator, place_map, label_map);

    instrs.extend(instrs2);
    let last_loc = Location::Unknown;
    let label_loc = Location::Unknown;
    Node {
        label,
        ssa_parameters,
        exn_succs,
        last,
        instrs,
        last_loc,
        label_loc,
    }
}

fn statement_to_instr(stmt: &stable_mir::mir::Statement, place_map: &PlaceMap) -> Vec<Instr> {
    match &stmt.kind {
        StatementKind::Assign(place, rvalue) => assign_statement_to_instr(place, rvalue, place_map),
        StatementKind::StorageLive(_) => vec![],
        StatementKind::StorageDead(_) => vec![],
        s => todo!("Statement to textual: {:?}", s),
    }
}

fn assign_statement_to_instr(
    place: &stable_mir::mir::Place,
    rvalue: &Rvalue,
    place_map: &PlaceMap,
) -> Vec<Instr> {
    let (id, _) = place_to_id(place, place_map);
    let (exp2, typ) = rvalue_to_exp(rvalue, place_map);
    vec![Instr::Store {
        exp1: Exp::LVar(VarName::new(id.clone(), None)),
        typ,
        exp2,
        loc: Location::Unknown,
    }]
}

pub fn unop_to_proc_name(op: UnOp, typ: &Typ) -> String {
    use UnOp::*;

    match (op, typ) {
        (Neg, Typ::Int(_)) => "__sil_neg".into(),
        (Neg, Typ::Float) => "__sil_neg".into(), 

        (Not, Typ::Int(IntKind::Bool)) => "__sil_lnot".into(), 
        (Not, Typ::Int(_)) => "__sil_bnot".into(),      

        _ => panic!("unsupported unop/type combo: {:?}, {:?}", op, typ),
    }
}

pub fn binop_to_proc_name(op: BinOp, typ: &Typ) -> String {
    use BinOp::*;

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

        _ => panic!("unsupported binop/type combo: {:?}, {:?}", op, typ),
    }
}

fn rvalue_to_exp(rvalue: &Rvalue, place_map: &PlaceMap) -> (Exp, Option<Typ>) {
    match rvalue {
        Rvalue::BinaryOp(op, op1, op2) => {
            let (exp1, typ) = operand_to_exp(op1, place_map);
            let (exp2, _) = operand_to_exp(op2, place_map);
            let typ = typ.expect("BinaryOp must yield a type");
            let proc_name = binop_to_proc_name(*op, &typ);

            (
                Exp::Call {
                    proc: QualifiedProcName::new(proc_name, None),
                    args: vec![exp1, exp2],
                    kind: CallKind::NonVirtual,
                },
                Some(typ),
            )
        }

        Rvalue::UnaryOp(op, operand) => {
            let (exp, typ) = operand_to_exp(operand, place_map);
            let typ = typ.expect("UnaryOp must yield a type");
            let proc_name = unop_to_proc_name(*op, &typ);

            (
                Exp::Call {
                    proc: QualifiedProcName::new(proc_name, None),
                    args: vec![exp],
                    kind: CallKind::NonVirtual,
                },
                Some(typ),
            )
        }

        Rvalue::Ref(_, _, place) | Rvalue::AddressOf(_, place) => {
            let (var_name, typ) = place_to_id(place, place_map);
            let exp = Exp::LVar(VarName::new(var_name.clone(), None));
            let ptr_typ = Typ::Ptr(Box::new(typ.clone()));
            (exp, Some(ptr_typ))
        }        

        Rvalue::Use(op) => operand_to_exp(op, place_map),
        s => todo!("{:?}", s),
    }
}

fn operand_to_exp(op: &Operand, place_map: &PlaceMap) -> (Exp, Option<Typ>) {
    match op {
        Operand::Copy(place) => (
            // Instr::Load {
            //     id: ,
            //     exp: Exp::LVar(VarName::from_place(place, place_map)),
            //     typ: None,
            //     loc: Location::Unknown,
            // },
            Exp::LVar(VarName::from_place(place, place_map)),
            Some(Typ::Int(IntKind::I32)),
        ),
        Operand::Move(place) => (
            Exp::LVar(VarName::from_place(place, place_map)),
            Some(Typ::Int(IntKind::I32)),
        ),
        Operand::Constant(const_operand) => {
            let (exp, typ) = const_operand_to_exp(const_operand);
            (exp, typ)
        }
    }
}

fn const_operand_to_exp(const_operand: &ConstOperand) -> (Exp, Option<Typ>) {
    let const_kind = const_operand.const_.kind();
    let typ = kind_to_textual(&const_operand.const_.ty().kind());

    let const_ = match const_kind {
        ConstantKind::Allocated(alloc) => {
            decode_allocated(&alloc.bytes, &typ)
        }

        ConstantKind::ZeroSized => Const::Null,

        ConstantKind::Unevaluated(_)
        | ConstantKind::Param(_)
        | ConstantKind::Ty(_) => {
            // TODO: Extend later to support more constant kinds if needed
            // TODO: Implement proper error handling
            debug_assert!(false, "Unsupported constant kind encountered: {:?}", const_kind);
            Const::Int(0)
        }
    };

    (Exp::Const(const_), Some(typ))
}

pub fn decode_allocated(bytes: &Vec<Option<u8>>, typ: &Typ) -> Const {
    let raw_bytes: Vec<u8> = bytes.iter().map(|b| b.unwrap_or(0)).collect();
    
    match typ {
        Typ::Ptr(inner) if matches!(**inner, Typ::Array(_)) => {
            let s: String = raw_bytes
                .iter()
                .take_while(|b| **b != 0)
                .map(|b| *b as char)
                .collect();
            Const::Str(s)
        }

        Typ::Float => {
            if raw_bytes.len() >= 8 {
                let mut arr = [0u8; 8];
                arr.copy_from_slice(&raw_bytes[..8]);
                Const::Float(f64::from_bits(u64::from_le_bytes(arr)))
            } else {
                Const::Float(0.0)
            }
        }

        Typ::Int(_) | Typ::Ptr(_) | Typ::Fun(_) => Const::Int(bytes_to_int(bytes)),

        Typ::Null | Typ::Void => Const::Null,

        // TODO: Implement proper exception handling
        _ => {
            debug_assert!(false, "decode_allocated: unsupported type {:?}", typ);
            Const::Int(0)
        }
    }
}

fn decls_to_locals(locals: &[LocalDecl]) -> (PlaceMap, Vec<(VarName, Annotated)>) {
    locals
        .iter()
        .enumerate()
        .map(|(place, local)| decl_to_local(place, local))
        .unzip()
}

fn decl_to_local(
    place: usize,
    local: &LocalDecl,
) -> ((usize, (String, Typ)), (VarName, Annotated)) {
    let id = fresh_id(place);
    let typ = local_decl_to_type(local);
    let varname = VarName::new(id.clone(), Some(local.span));
    let annotated = local_decl_to_annotated_typ(local);
    ((place, (id, typ)), (varname, annotated))
}

fn terminator_to_textual(
    terminator: &stable_mir::mir::Terminator,
    place_map: &PlaceMap,
    label_map: &LabelMap
) -> (Vec<Instr>, Terminator) {
    match &terminator.kind {
        stable_mir::mir::TerminatorKind::Return => (
            vec![],
            Terminator::Ret(Exp::LVar(VarName::from_index(0, place_map))),
        ),
        stable_mir::mir::TerminatorKind::Call {
            func:
                stable_mir::mir::Operand::Constant(ConstOperand {
                    span: _,
                    user_ty: _,
                    const_,
                }),
            args:_,
            destination,
            target,
            unwind:_,
        } => match const_.ty().kind() {
            TyKind::RigidTy(RigidTy::FnDef(FnDef(def), _)) => {
                let exp2 = Exp::Call { proc: QualifiedProcName::new(def.name(), Some(terminator.span)), args: vec![], kind: CallKind::NonVirtual};
                let term = Terminator::jump(target, &label_map);
                let (_, typ) = place_to_id(destination, place_map);
                let store = Instr::Store { exp1: Exp::LVar(VarName::from_place(&destination, place_map)), typ: Some(typ.clone()), exp2, loc: Location::from_span(Some(terminator.span))};
                (vec![store],term)
            },
            _ => todo!(),
        },
        stable_mir::mir::TerminatorKind::SwitchInt { discr, targets } => {
            let (cond_exp, typ) = operand_to_exp(discr, place_map);

            let mut branches = targets.branches();
            let (switch_val, target_then) = branches
                .next()
                .expect("Expected at least one branch in SwitchInt");

            assert!(
                branches.next().is_none(),
                "Only binary SwitchInt supported for now"
            );

            let target_else = targets.otherwise();

            let target_then_idx = Some(target_then);
            let target_else_idx = Some(target_else);

            let then_terminator = Terminator::jump(&target_then_idx, label_map);
            let else_terminator = Terminator::jump(&target_else_idx, label_map);

            let switch_val_i128 = i128::try_from(switch_val)
                .expect("SwitchInt constant value too large for i128");

            let proc_name = binop_to_proc_name(BinOp::Eq, typ.as_ref().expect("Expected type for switch discr"));

            let eq_expr = Exp::Call {
                proc: QualifiedProcName::new(proc_name, None),
                args: vec![
                    cond_exp,
                    Exp::Const(Const::Int(switch_val_i128)),
                ],
                kind: CallKind::NonVirtual,
            };

            (
                vec![],
                Terminator::If {
                    bexp: BoolExp::Exp(eq_expr),
                    then: Box::new(then_terminator),
                    else_: Box::new(else_terminator),
                },
            )
        }
        stable_mir::mir::TerminatorKind::Goto { target } => {
            let target_idx = Some(*target);
            let term = Terminator::jump(&target_idx, label_map);
            (vec![], term)
        }        
                   
        term => todo!("{:?}", term),
    }
}

fn place_to_id<'a>(place: &Place, place_map: &'a PlaceMap) -> (&'a String, &'a Typ) {
    let (expr, typ) = place_map.get(&place.local).unwrap();
    (expr, typ)
}
