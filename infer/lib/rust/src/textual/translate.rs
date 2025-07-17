use std::collections::HashMap;

use stable_mir::{
    mir::{BasicBlock, BinOp, ConstOperand, LocalDecl, Operand, Place, Rvalue, StatementKind}, ty::{Allocation, ConstantKind, FnDef, RigidTy, Span, TyKind}, CrateDef, CrateItem
};

use crate::textual_defs::{
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
    typ::{Annotated, Typ, local_decl_to_annotated_typ, local_decl_to_type},
    varname::VarName,
};

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

fn rvalue_to_exp(rvalue: &Rvalue, place_map: &PlaceMap) -> (Exp, Option<Typ>) {
    match rvalue {
        Rvalue::BinaryOp(BinOp::Add, op1, op2) => {
            let (exp1, typ) = operand_to_exp(op1, place_map);
            let (exp2, _) = operand_to_exp(op2, place_map);
            (
                Exp::Call {
                    proc: QualifiedProcName::new("__sil_plusa_int".to_string(), None), //TODO Generalize
                    args: vec![exp1, exp2],
                    kind: CallKind::NonVirtual,
                },
                typ,
            )
        }
        Rvalue::Use(op) => operand_to_exp(op, place_map),
        s => todo!("{:?}", s),
    }
}

fn operand_to_exp(op: &Operand, place_map: &PlaceMap) -> (Exp, Option<Typ>) {
    match op {
        Operand::Copy(place) => (
            Exp::LVar(VarName::from_place(place, place_map)),
            Some(Typ::Int),
        ),
        Operand::Move(place) => (
            Exp::LVar(VarName::from_place(place, place_map)),
            Some(Typ::Int),
        ),
        Operand::Constant(const_operand) => (const_operand_to_exp(&const_operand), Some(Typ::Int)), //TODO Constant types
    }
}

fn const_operand_to_exp(const_operand: &ConstOperand) -> Exp {
    let const_ = const_operand.const_.kind();
    let const_ = match const_ {
        ConstantKind::Allocated(Allocation {
            bytes,
            provenance: _,
            align: _,
            mutability: _,
        }) => Const::Int(bytes_to_int(bytes)),
        ConstantKind::ZeroSized => Const::Null, //TODO Handling of unit type
        s => todo!("Const to textual: {:?}", s),
    };
    Exp::Const(const_)
}

fn bytes_to_int(bytes: &Vec<Option<u8>>) -> i128 {
    bytes
        .iter()
        .enumerate()
        .map(|(i, b)| (b.unwrap_or_default() as i128) << i)
        .sum::<i128>()
}

fn decls_to_locals(locals: &[LocalDecl]) -> (PlaceMap, Vec<(VarName, Annotated)>) {
    locals
        .iter()
        .enumerate()
        .map(|(place, local)| decl_to_local(place, local))
        .unzip()
}

fn fresh_id(i: usize) -> String {
    format!("var_{i}")
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
        term => todo!("{:?}", term),
    }
}

fn place_to_id<'a>(place: &Place, place_map: &'a PlaceMap) -> (&'a String, &'a Typ) {
    let (expr, typ) = place_map.get(&place.local).unwrap();
    (expr, typ)
}
