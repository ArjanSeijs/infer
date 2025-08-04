use std::collections::HashMap;

use stable_mir::{
    mir::{BasicBlock, BinOp, ConstOperand, LocalDecl, Operand, Place, Rvalue, StatementKind}, ty::{ConstantKind, FnDef, RigidTy, Span, TyKind}, CrateDef, CrateItem,
};

use crate::textual_defs::{
    ident::Ident,
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
    typ::{Annotated, Typ, local_decl_to_annotated_typ, local_decl_to_type, kind_to_textual},
    varname::VarName,
};

use crate::utils::{fresh_id, bytes_to_int, bytes_to_float, bytes_to_cstr, binop_to_proc_name, unop_to_proc_name};

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
    let (mut lhs_instrs, exp1, _typ1) = place_to_exp(place, place_map);
    let (mut rhs_instrs, exp2, typ2) = rvalue_to_exp(rvalue, place_map);

    let store = Instr::Store {
        exp1,
        typ: typ2.clone(),
        exp2,
        loc: Location::Unknown,
    };

    lhs_instrs.append(&mut rhs_instrs);
    lhs_instrs.push(store);
    lhs_instrs
}

fn rvalue_to_exp(rvalue: &Rvalue, place_map: &PlaceMap) -> (Vec<Instr>, Exp, Option<Typ>) {
    match rvalue {
        Rvalue::UnaryOp(op, operand) => {
            let (instrs, exp, typ) = operand_to_exp(operand, place_map);
            let typ = typ.expect("UnaryOp must yield a type");
            let proc_name = unop_to_proc_name(*op, &typ);

            let call = Exp::Call {
                proc: QualifiedProcName::new(proc_name, None),
                args: vec![exp],
                kind: CallKind::NonVirtual,
            };

            (instrs, call, Some(typ))
        }

        Rvalue::BinaryOp(op, op1, op2) => {
            let (instrs1, exp1, typ) = operand_to_exp(op1, place_map);
            let (instrs2, exp2, _) = operand_to_exp(op2, place_map);
            let typ = typ.expect("BinaryOp must yield a type");
            let proc_name = binop_to_proc_name(*op, &typ);

            let call = Exp::Call {
                proc: QualifiedProcName::new(proc_name, None),
                args: vec![exp1, exp2],
                kind: CallKind::NonVirtual,
            };

            let mut instrs = instrs1;
            instrs.extend(instrs2);
            (instrs, call, Some(typ))
        }

        Rvalue::Ref(_, _, place) | Rvalue::AddressOf(_, place) => {
            use stable_mir::mir::ProjectionElem;

            // &*x → eliminate both → equivalent to Copy(x)
            if let [ProjectionElem::Deref] = place.projection.as_slice() {
                let stripped_place = Place {
                    local: place.local,
                    projection: vec![],
                };
                return operand_to_exp(&Operand::Copy(stripped_place), place_map);
            }

            // General case: convert place into Exp, then wrap in Ref
            let (instrs, exp, typ) = place_to_exp(place, place_map);
            let ref_exp = Exp::Ref(Box::new(exp));
            let ref_typ = Typ::Ptr(Box::new(typ.clone()));

            (instrs, ref_exp, Some(ref_typ))
        }

        Rvalue::Use(op) => operand_to_exp(op, place_map),

        r => todo!("rvalue_to_exp: {:?}", r),
    }
}

fn operand_to_exp(op: &Operand, place_map: &PlaceMap) -> (Vec<Instr>, Exp, Option<Typ>) {
    match op {
        Operand::Copy(place) | Operand::Move(place) => {
            let (instrs, exp, typ) = place_to_exp(place, place_map);

            let tmp_id = Ident::fresh();
            let tmp_exp = Exp::Var(tmp_id.clone());

            let load_instr = Instr::Load {
                loc: Location::Unknown,
                id: tmp_id,
                exp: *Box::new(exp),
                typ: Some(typ.clone()),
            };

            let mut all_instrs = instrs;
            all_instrs.push(load_instr);

            (all_instrs, tmp_exp, Some(typ))
        }

        Operand::Constant(const_operand) => {
            let (exp, typ) = const_operand_to_exp(const_operand);
            (vec![], exp, typ.clone())
        }
    }
}

fn const_operand_to_exp(const_operand: &ConstOperand) -> (Exp, Option<Typ>) {
    let const_kind = const_operand.const_.kind();
    let ty_kind = const_operand.const_.ty().kind();
    let typ = kind_to_textual(&ty_kind);

    let const_ = match const_kind {
        ConstantKind::Allocated(alloc) => match &typ {
            Typ::Ptr(inner) if matches!(**inner, Typ::Array(_)) => {
                Const::Str(bytes_to_cstr(&alloc.bytes))
            }

            Typ::Float => {
                Const::Float(bytes_to_float(&alloc.bytes))
            }

            Typ::Int(_) | Typ::Ptr(_) | Typ::Fun(_) => {
                Const::Int(bytes_to_int(&alloc.bytes))
            }

            Typ::Null | Typ::Void => Const::Null,

            other => {
                todo!("Unsupported Allocated type in constant: {:?}", other)
            }
        },

        ConstantKind::ZeroSized => Const::Null,

        ConstantKind::Unevaluated(_)
        | ConstantKind::Param(_)
        | ConstantKind::Ty(_) => {
            todo!("Unsupported constant kind: {:?}", const_kind)
        }
    };

    (Exp::Const(const_), Some(typ))
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
            args,
            destination,
            target,
            unwind:_,
        } => match const_.ty().kind() {
            TyKind::RigidTy(RigidTy::FnDef(FnDef(def), _)) => {
                let mut instrs = vec![];
                let mut arg_exps = vec![];

                for arg in args {
                    let (arg_instrs, arg_exp, _) = operand_to_exp(arg, place_map);
                    instrs.extend(arg_instrs);
                    arg_exps.push(arg_exp);
                }

                let (place_instrs, exp1, typ) = place_to_exp(destination, place_map);
                instrs.extend(place_instrs);

                let call = Exp::Call {
                    proc: QualifiedProcName::new(def.name(), Some(terminator.span)),
                    args: arg_exps,
                    kind: CallKind::NonVirtual,
                };

                let store = Instr::Store {
                    exp1,
                    exp2: call,
                    typ: Some(typ.clone()),
                    loc: Location::from_span(Some(terminator.span)),
                };

                instrs.push(store);

                let term = Terminator::jump(target, label_map);

                (instrs, term)
            },

            _ => todo!("Unsupported call target type: {:?}", const_.ty()),
        },
        stable_mir::mir::TerminatorKind::SwitchInt { discr, targets } => {
            let (instrs, cond_exp, typ) = operand_to_exp(discr, place_map);
        
            let mut branches = targets.branches();
            let (switch_val, target_then) = branches
                .next()
                .expect("Expected at least one branch in SwitchInt");
            assert!(branches.next().is_none(), "Only binary SwitchInt supported");
        
            let target_else = targets.otherwise();
        
            let target_then_idx = Some(target_then);
            let target_else_idx = Some(target_else);
        
            let then_terminator = Terminator::jump(&target_then_idx, label_map);
            let else_terminator = Terminator::jump(&target_else_idx, label_map);
        
            let proc_name = binop_to_proc_name(BinOp::Eq, typ.as_ref().expect("Expected type"));
            let cmp_exp = Exp::Call {
                proc: QualifiedProcName::new(proc_name, None),
                args: vec![
                    cond_exp,
                    Exp::Const(Const::Int(switch_val as i128)),
                ],
                kind: CallKind::NonVirtual,
            };
        
            (
                instrs,
                Terminator::If {
                    bexp: BoolExp::Exp(cmp_exp),
                    then: Box::new(then_terminator),
                    else_: Box::new(else_terminator),
                }
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

fn place_to_id<'a>(place: &'a Place, place_map: &'a PlaceMap) -> (&'a String, &'a Typ) {
    let (expr, typ) = place_map.get(&place.local).unwrap();
    (expr, typ)
}

fn place_to_exp(place: &Place, place_map: &PlaceMap) -> (Vec<Instr>, Exp, Typ) {
    use stable_mir::mir::ProjectionElem;

    // Base case: _n
    if place.projection.is_empty() {
        let (id, typ) = place_to_id(place, place_map);
        let var = VarName::new(id.clone(), None);
        return (vec![], Exp::LVar(var), typ.clone());
    }

    // Recursive case: handle projection chain
    let mut projections = place.projection.clone();
    let last_proj = projections.pop().unwrap();
    let base_place = Place {
        local: place.local,
        projection: projections,
    };

    let (mut _instrs, _base_exp, _base_typ) = place_to_exp(&base_place, place_map);

    match last_proj {
        ProjectionElem::Deref => {
            todo!("place_to_exp: ProjectionElem::Deref")
        }

        ProjectionElem::Field(_field, _ty) => {
            todo!("place_to_exp: ProjectionElem::Field")
        }

        ProjectionElem::Index(_) => {
            todo!("place_to_exp: ProjectionElem::Index")
        }

        other => todo!("Unsupported projection: {:?}", other),
    }
}
