// x86 recipe predicates.
fn recipe_predicate_rexop1u_id(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryImm { imm, .. } = *inst {
        return predicates::is_signed_int(imm, 32, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1ldwithindex(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::LoadComplex { offset, .. } = *inst {
        return predicates::is_equal(offset, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1ldwithindexdisp8(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::LoadComplex { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1ldwithindexdisp32(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::LoadComplex { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 32, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1stwithindex(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::StoreComplex { offset, .. } = *inst {
        return predicates::is_equal(offset, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1stwithindexdisp8(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::StoreComplex { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1stwithindexdisp32(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::StoreComplex { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 32, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1st(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::Store { offset, .. } = *inst {
        return predicates::is_equal(offset, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1stdisp8(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::Store { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1ld(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::Load { offset, .. } = *inst {
        return predicates::is_equal(offset, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1lddisp8(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::Load { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1lddisp32(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::Load { offset, .. } = *inst {
        return predicates::is_signed_int(offset, 32, 0);
    }
    unreachable!();
}
fn recipe_predicate_op1adjustsp_ib(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryImm { imm, .. } = *inst {
        return predicates::is_signed_int(imm, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_op2f32imm_z(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryIeee32 { imm, .. } = *inst {
        return predicates::is_zero_32_bit_float(imm);
    }
    unreachable!();
}
fn recipe_predicate_mp2f64imm_z(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryIeee64 { imm, .. } = *inst {
        return predicates::is_zero_64_bit_float(imm);
    }
    unreachable!();
}
fn recipe_predicate_mp3furmi_rnd(isap: crate::settings::PredicateView, _: &ir::InstructionData) -> bool {
    isap.test(25)
}
fn recipe_predicate_op2fcscc(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::FloatCompare { cond, .. } = *inst {
        return predicates::is_equal(cond, ir::condcodes::FloatCC::Ordered) || predicates::is_equal(cond, ir::condcodes::FloatCC::Unordered) || predicates::is_equal(cond, ir::condcodes::FloatCC::OrderedNotEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::GreaterThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::GreaterThanOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrLessThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrLessThanOrEqual);
    }
    unreachable!();
}
fn recipe_predicate_dynrexop1r_ib(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::BinaryImm64 { imm, .. } = *inst {
        return predicates::is_signed_int(imm, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_dynrexop1r_id(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::BinaryImm64 { imm, .. } = *inst {
        return predicates::is_signed_int(imm, 32, 0);
    }
    unreachable!();
}
fn recipe_predicate_dynrexop1icscc_ib(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::IntCompareImm { imm, .. } = *inst {
        return predicates::is_signed_int(imm, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_dynrexop1icscc_id(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::IntCompareImm { imm, .. } = *inst {
        return predicates::is_signed_int(imm, 32, 0);
    }
    unreachable!();
}
fn recipe_predicate_mp2r_ib_unsigned_fpr(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::BinaryImm8 { imm, .. } = *inst {
        return predicates::is_unsigned_int(imm, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_mp3fa_ib(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::TernaryImm8 { imm, .. } = *inst {
        return predicates::is_unsigned_int(imm, 8, 0);
    }
    unreachable!();
}
fn recipe_predicate_op2pfcmp(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::FloatCompare { cond, .. } = *inst {
        return predicates::is_equal(cond, ir::condcodes::FloatCC::Equal) || predicates::is_equal(cond, ir::condcodes::FloatCC::LessThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::LessThanOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::Unordered) || predicates::is_equal(cond, ir::condcodes::FloatCC::NotEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrGreaterThanOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrGreaterThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::Ordered);
    }
    unreachable!();
}
fn recipe_predicate_op1brfb(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::BranchFloat { cond, .. } = *inst {
        return predicates::is_equal(cond, ir::condcodes::FloatCC::Ordered) || predicates::is_equal(cond, ir::condcodes::FloatCC::Unordered) || predicates::is_equal(cond, ir::condcodes::FloatCC::OrderedNotEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::GreaterThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::GreaterThanOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrLessThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrLessThanOrEqual);
    }
    unreachable!();
}
fn recipe_predicate_rexop1jt_entry(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::BranchTableEntry { imm, .. } = *inst {
        return predicates::is_equal(imm, 1) || predicates::is_equal(imm, 2) || predicates::is_equal(imm, 4) || predicates::is_equal(imm, 8);
    }
    unreachable!();
}
fn recipe_predicate_trapff(_: crate::settings::PredicateView, inst: &ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::FloatCondTrap { cond, .. } = *inst {
        return predicates::is_equal(cond, ir::condcodes::FloatCC::Ordered) || predicates::is_equal(cond, ir::condcodes::FloatCC::Unordered) || predicates::is_equal(cond, ir::condcodes::FloatCC::OrderedNotEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::GreaterThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::GreaterThanOrEqual) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrLessThan) || predicates::is_equal(cond, ir::condcodes::FloatCC::UnorderedOrLessThanOrEqual);
    }
    unreachable!();
}

/// x86 recipe predicate table.
///
/// One entry per recipe, set to Some only when the recipe is guarded by a predicate.
pub static RECIPE_PREDICATES: [RecipePredicate; 363] = [
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_rexop1u_id),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp32),
    Some(recipe_predicate_op1ldwithindexdisp32),
    Some(recipe_predicate_op1ldwithindexdisp32),
    Some(recipe_predicate_op1ldwithindexdisp32),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1stdisp8),
    Some(recipe_predicate_op1stdisp8),
    Some(recipe_predicate_op1stdisp8),
    Some(recipe_predicate_op1stdisp8),
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1stdisp8),
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp32),
    Some(recipe_predicate_op1lddisp32),
    Some(recipe_predicate_op1lddisp32),
    Some(recipe_predicate_op1lddisp32),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op1adjustsp_ib),
    Some(recipe_predicate_rexop1u_id),
    Some(recipe_predicate_op1adjustsp_ib),
    Some(recipe_predicate_rexop1u_id),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp32),
    Some(recipe_predicate_op1lddisp32),
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp32),
    Some(recipe_predicate_op1ldwithindexdisp32),
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1stdisp8),
    Some(recipe_predicate_op1stdisp8),
    None,
    None,
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1stwithindexdisp32),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op2f32imm_z),
    Some(recipe_predicate_mp2f64imm_z),
    Some(recipe_predicate_op2f32imm_z),
    Some(recipe_predicate_mp2f64imm_z),
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_mp3furmi_rnd),
    Some(recipe_predicate_mp3furmi_rnd),
    None,
    None,
    Some(recipe_predicate_op2fcscc),
    Some(recipe_predicate_op2fcscc),
    Some(recipe_predicate_op2fcscc),
    Some(recipe_predicate_op2fcscc),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_dynrexop1r_ib),
    Some(recipe_predicate_dynrexop1r_ib),
    Some(recipe_predicate_dynrexop1r_id),
    Some(recipe_predicate_dynrexop1r_id),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_dynrexop1icscc_ib),
    Some(recipe_predicate_dynrexop1icscc_ib),
    Some(recipe_predicate_dynrexop1icscc_id),
    Some(recipe_predicate_dynrexop1icscc_id),
    None,
    None,
    Some(recipe_predicate_dynrexop1r_ib),
    Some(recipe_predicate_dynrexop1r_ib),
    Some(recipe_predicate_dynrexop1r_id),
    Some(recipe_predicate_dynrexop1r_id),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_mp2r_ib_unsigned_fpr),
    Some(recipe_predicate_mp2r_ib_unsigned_fpr),
    None,
    None,
    Some(recipe_predicate_mp3fa_ib),
    Some(recipe_predicate_mp3fa_ib),
    None,
    Some(recipe_predicate_mp3fa_ib),
    Some(recipe_predicate_mp3fa_ib),
    Some(recipe_predicate_mp3fa_ib),
    Some(recipe_predicate_mp3fa_ib),
    Some(recipe_predicate_mp3fa_ib),
    None,
    None,
    Some(recipe_predicate_mp2r_ib_unsigned_fpr),
    Some(recipe_predicate_mp2r_ib_unsigned_fpr),
    Some(recipe_predicate_mp2r_ib_unsigned_fpr),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1st),
    Some(recipe_predicate_op1stdisp8),
    Some(recipe_predicate_op1stdisp8),
    None,
    None,
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindex),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp8),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1stwithindexdisp32),
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp32),
    Some(recipe_predicate_op1lddisp32),
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp32),
    Some(recipe_predicate_op1ldwithindexdisp32),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1ld),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp8),
    Some(recipe_predicate_op1lddisp32),
    Some(recipe_predicate_op1lddisp32),
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindex),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp8),
    Some(recipe_predicate_op1ldwithindexdisp32),
    Some(recipe_predicate_op1ldwithindexdisp32),
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_dynrexop1r_ib),
    Some(recipe_predicate_dynrexop1r_ib),
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op2pfcmp),
    Some(recipe_predicate_op2pfcmp),
    Some(recipe_predicate_op2pfcmp),
    Some(recipe_predicate_op2pfcmp),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_op1brfb),
    Some(recipe_predicate_op1brfb),
    Some(recipe_predicate_op1brfb),
    Some(recipe_predicate_op1brfb),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_rexop1jt_entry),
    Some(recipe_predicate_rexop1jt_entry),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(recipe_predicate_trapff),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];

// x86 instruction predicates.
fn inst_predicate_0(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryImm { imm, .. } = *inst {
        let _ = func;
        return predicates::is_unsigned_int(imm, 32, 0);
    }
    unreachable!();
}
fn inst_predicate_1(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryImm { imm, .. } = *inst {
        let _ = func;
        return predicates::is_zero_int(imm);
    }
    unreachable!();
}
fn inst_predicate_2(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I16
}
fn inst_predicate_3(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I32
}
fn inst_predicate_4(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I64
}
fn inst_predicate_5(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I8
}
fn inst_predicate_6(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::B1
}
fn inst_predicate_7(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::B8
}
fn inst_predicate_8(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::B16
}
fn inst_predicate_9(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::B32
}
fn inst_predicate_10(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::B64
}
fn inst_predicate_11(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::LoadComplex { ref args, .. } = *inst {
        let _ = func;
        return predicates::has_length_of(args, 2, func);
    }
    unreachable!();
}
fn inst_predicate_12(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::StoreComplex { ref args, .. } = *inst {
        let _ = func;
        return predicates::has_length_of(args, 3, func);
    }
    unreachable!();
}
fn inst_predicate_13(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::F32
}
fn inst_predicate_14(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::F64
}
fn inst_predicate_15(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryIeee32 { imm, .. } = *inst {
        let _ = func;
        return predicates::is_zero_32_bit_float(imm);
    }
    unreachable!();
}
fn inst_predicate_16(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryIeee64 { imm, .. } = *inst {
        let _ = func;
        return predicates::is_zero_64_bit_float(imm);
    }
    unreachable!();
}
fn inst_predicate_17(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[1]) == ir::types::I8
}
fn inst_predicate_18(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[1]) == ir::types::I16
}
fn inst_predicate_19(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[1]) == ir::types::I32
}
fn inst_predicate_20(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::B8X16
}
fn inst_predicate_21(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::B16X8
}
fn inst_predicate_22(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::B32X4
}
fn inst_predicate_23(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::B64X2
}
fn inst_predicate_24(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I8X16
}
fn inst_predicate_25(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I16X8
}
fn inst_predicate_26(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I32X4
}
fn inst_predicate_27(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::I64X2
}
fn inst_predicate_28(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::F32X4
}
fn inst_predicate_29(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    let args = inst.arguments(&func.dfg.value_lists);
    func.dfg.value_type(args[0]) == ir::types::F64X2
}
fn inst_predicate_30(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryConst { constant_handle, .. } = *inst {
        let _ = func;
        return predicates::is_all_zeroes(func.dfg.constants.get(constant_handle));
    }
    unreachable!();
}
fn inst_predicate_31(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryConst { constant_handle, .. } = *inst {
        let _ = func;
        return predicates::is_all_ones(func.dfg.constants.get(constant_handle));
    }
    unreachable!();
}
fn inst_predicate_32(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::IntCompare { cond, .. } = *inst {
        let _ = func;
        return predicates::is_equal(cond, IntCC::Equal);
    }
    unreachable!();
}
fn inst_predicate_33(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::IntCompare { cond, .. } = *inst {
        let _ = func;
        return predicates::is_equal(cond, IntCC::SignedGreaterThan);
    }
    unreachable!();
}
fn inst_predicate_34(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::FuncAddr { func_ref, .. } = *inst {
        let _ = func;
        return predicates::is_colocated_func(func_ref, func);
    }
    unreachable!();
}
fn inst_predicate_35(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::UnaryGlobalValue { global_value, .. } = *inst {
        let _ = func;
        return predicates::is_colocated_data(global_value, func);
    }
    unreachable!();
}
fn inst_predicate_36(func: &crate::ir::Function, inst: &crate::ir::InstructionData) -> bool {
    if let crate::ir::InstructionData::Call { func_ref, .. } = *inst {
        let _ = func;
        return predicates::is_colocated_func(func_ref, func);
    }
    unreachable!();
}

/// x86 instruction predicate table.
///
/// One entry per instruction predicate, so the encoding bytecode can embed indexes into this
/// table.
pub static INST_PREDICATES: [InstPredicate; 37] = [
    inst_predicate_0,
    inst_predicate_1,
    inst_predicate_2,
    inst_predicate_3,
    inst_predicate_4,
    inst_predicate_5,
    inst_predicate_6,
    inst_predicate_7,
    inst_predicate_8,
    inst_predicate_9,
    inst_predicate_10,
    inst_predicate_11,
    inst_predicate_12,
    inst_predicate_13,
    inst_predicate_14,
    inst_predicate_15,
    inst_predicate_16,
    inst_predicate_17,
    inst_predicate_18,
    inst_predicate_19,
    inst_predicate_20,
    inst_predicate_21,
    inst_predicate_22,
    inst_predicate_23,
    inst_predicate_24,
    inst_predicate_25,
    inst_predicate_26,
    inst_predicate_27,
    inst_predicate_28,
    inst_predicate_29,
    inst_predicate_30,
    inst_predicate_31,
    inst_predicate_32,
    inst_predicate_33,
    inst_predicate_34,
    inst_predicate_35,
    inst_predicate_36,
];

/// x86 encoding lists.
///
/// This contains the entire encodings bytecode for every single instruction; the encodings
/// interpreter knows where to start from thanks to the initial lookup in the level 1 and level 2
/// table entries below.
pub static ENCLISTS: [u16; 2792] = [
    // 000000: adjust_sp_down.i64 (I64)
    // --> [RexOp1adjustsp#8029] and stop
    0x00c9, 0x8029,
    // end of adjust_sp_down.i64 (I64)
    // 000002: band.i64 (I64)
    // --> [RexOp1rr#8021] and stop
    // 000002: band.b64 (I64)
    // --> [RexOp1rr#8021] and stop
    0x014f, 0x8021,
    // end of band.b64 (I64)
    // end of band.i64 (I64)
    // 000004: band_imm.i64 (I64)
    // --> [RexOp1r_ib#c083]
    0x015e, 0xc083,
    // --> [RexOp1r_id#c081] and stop
    0x0163, 0xc081,
    // end of band_imm.i64 (I64)
    // 000008: bint.i64 (I64)
    // skip 4 unless inst_predicate_6
    0x5006,
    // --> [RexOp2urm_noflags#4b6]
    0x0022, 0x04b6,
    // --> [Op2urm_noflags_abcd#4b6]
    0x0020, 0x04b6,
    // skip 4 unless inst_predicate_7
    0x5007,
    // --> [RexOp2urm_noflags#4b6]
    0x0022, 0x04b6,
    // --> [Op2urm_noflags_abcd#4b6]
    0x0020, 0x04b6,
    // stop unless inst_predicate_10
    0x100a,
    // --> [RexOp2urm_noflags#4b6]
    0x0022, 0x04b6,
    // --> [Op2urm_noflags_abcd#4b6] and stop
    0x0021, 0x04b6,
    // end of bint.i64 (I64)
    // 000017: bitcast.i64 (I64)
    // stop unless inst_predicate_14
    0x100e,
    // --> [RexMp2rfumr#857e] and stop
    0x00d9, 0x857e,
    // end of bitcast.i64 (I64)
    // 00001a: bnot.i64 (I64)
    // --> [RexOp1ur#a0f7] and stop
    // 00001a: bnot.b64 (I64)
    // --> [RexOp1ur#a0f7] and stop
    0x0167, 0xa0f7,
    // end of bnot.b64 (I64)
    // end of bnot.i64 (I64)
    // 00001c: bor.i64 (I64)
    // --> [RexOp1rr#8009] and stop
    // 00001c: bor.b64 (I64)
    // --> [RexOp1rr#8009] and stop
    0x014f, 0x8009,
    // end of bor.b64 (I64)
    // end of bor.i64 (I64)
    // 00001e: bor_imm.i64 (I64)
    // --> [RexOp1r_ib#9083]
    0x015e, 0x9083,
    // --> [RexOp1r_id#9081] and stop
    0x0163, 0x9081,
    // end of bor_imm.i64 (I64)
    // 000022: brnz.i64 (I64)
    // --> [RexOp1tjccb#8075]
    0x02a0, 0x8075,
    // --> [RexOp1tjccd#8085] and stop
    0x02a5, 0x8085,
    // end of brnz.i64 (I64)
    // 000026: brz.i64 (I64)
    // --> [RexOp1tjccb#8074]
    0x02a0, 0x8074,
    // --> [RexOp1tjccd#8084] and stop
    0x02a5, 0x8084,
    // end of brz.i64 (I64)
    // 00002a: bxor.i64 (I64)
    // --> [RexOp1rr#8031] and stop
    // 00002a: bxor.b64 (I64)
    // --> [RexOp1rr#8031] and stop
    0x014f, 0x8031,
    // end of bxor.b64 (I64)
    // end of bxor.i64 (I64)
    // 00002c: bxor_imm.i64 (I64)
    // --> [RexOp1r_ib#e083]
    0x015e, 0xe083,
    // --> [RexOp1r_id#e081] and stop
    0x0163, 0xe081,
    // end of bxor_imm.i64 (I64)
    // 000030: call_indirect.i64 (I64)
    // --> [RexOp1call_r#20ff]
    0x0286, 0x20ff,
    // --> [Op1call_r#20ff] and stop
    // 000032: call_indirect.i32 (I32)
    // --> [Op1call_r#20ff] and stop
    0x0285, 0x20ff,
    // end of call_indirect.i32 (I32)
    // end of call_indirect.i64 (I64)
    // 000034: clz.i64 (I64)
    // stop unless PredicateView(23)
    0x103c,
    // --> [RexMp2urm#86bd] and stop
    0x0187, 0x86bd,
    // end of clz.i64 (I64)
    // 000037: const_addr.i64 (I64)
    // --> [RexOp1const_addr#808d] and stop
    0x027d, 0x808d,
    // end of const_addr.i64 (I64)
    // 000039: copy.i64 (I64)
    // --> [RexOp1umr#8089] and stop
    // 000039: copy.r64 (I64)
    // --> [RexOp1umr#8089] and stop
    0x0007, 0x8089,
    // end of copy.r64 (I64)
    // end of copy.i64 (I64)
    // 00003b: copy_nop.i64 (I64)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.i32 (I64)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.i8 (I64)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.i16 (I64)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.f64 (I64)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.f32 (I64)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.b8x16 (I64)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.b16x8 (I64)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.b32x4 (I64)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.b64x2 (I64)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.i8x16 (I64)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.i16x8 (I64)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.i32x4 (I64)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.i64x2 (I64)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.f32x4 (I64)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.f64x2 (I64)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.i32 (I32)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.i8 (I32)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.i16 (I32)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.i64 (I32)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.f64 (I32)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.f32 (I32)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.b8x16 (I32)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.b16x8 (I32)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.b32x4 (I32)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.b64x2 (I32)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.i8x16 (I32)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.i16x8 (I32)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.i32x4 (I32)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.i64x2 (I32)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.f32x4 (I32)
    // --> [stacknull#00] and stop
    // 00003b: copy_nop.f64x2 (I32)
    // --> [stacknull#00] and stop
    0x00c5, 0x0000,
    // end of copy_nop.f64x2 (I32)
    // end of copy_nop.f32x4 (I32)
    // end of copy_nop.i64x2 (I32)
    // end of copy_nop.i32x4 (I32)
    // end of copy_nop.i16x8 (I32)
    // end of copy_nop.i8x16 (I32)
    // end of copy_nop.b64x2 (I32)
    // end of copy_nop.b32x4 (I32)
    // end of copy_nop.b16x8 (I32)
    // end of copy_nop.b8x16 (I32)
    // end of copy_nop.f32 (I32)
    // end of copy_nop.f64 (I32)
    // end of copy_nop.i64 (I32)
    // end of copy_nop.i16 (I32)
    // end of copy_nop.i8 (I32)
    // end of copy_nop.i32 (I32)
    // end of copy_nop.f64x2 (I64)
    // end of copy_nop.f32x4 (I64)
    // end of copy_nop.i64x2 (I64)
    // end of copy_nop.i32x4 (I64)
    // end of copy_nop.i16x8 (I64)
    // end of copy_nop.i8x16 (I64)
    // end of copy_nop.b64x2 (I64)
    // end of copy_nop.b32x4 (I64)
    // end of copy_nop.b16x8 (I64)
    // end of copy_nop.b8x16 (I64)
    // end of copy_nop.f32 (I64)
    // end of copy_nop.f64 (I64)
    // end of copy_nop.i16 (I64)
    // end of copy_nop.i8 (I64)
    // end of copy_nop.i32 (I64)
    // end of copy_nop.i64 (I64)
    // 00003d: copy_to_ssa.i64 (I64)
    // --> [RexOp1umr_reg_to_ssa#8089] and stop
    // 00003d: copy_to_ssa.r64 (I64)
    // --> [RexOp1umr_reg_to_ssa#8089] and stop
    0x002f, 0x8089,
    // end of copy_to_ssa.r64 (I64)
    // end of copy_to_ssa.i64 (I64)
    // 00003f: ctz.i64 (I64)
    // stop unless PredicateView(22)
    0x103b,
    // --> [RexMp2urm#86bc] and stop
    0x0187, 0x86bc,
    // end of ctz.i64 (I64)
    // 000042: fill.i64 (I64)
    // --> [RexOp1fillSib32#808b] and stop
    // 000042: fill.r64 (I64)
    // --> [RexOp1fillSib32#808b] and stop
    0x00b3, 0x808b,
    // end of fill.r64 (I64)
    // end of fill.i64 (I64)
    // 000044: fill_nop.i64 (I64)
    // --> [fillnull#00] and stop
    // 000044: fill_nop.i32 (I64)
    // --> [fillnull#00] and stop
    // 000044: fill_nop.r64 (I64)
    // --> [fillnull#00] and stop
    // 000044: fill_nop.b1 (I64)
    // --> [fillnull#00] and stop
    // 000044: fill_nop.i8 (I64)
    // --> [fillnull#00] and stop
    // 000044: fill_nop.i16 (I64)
    // --> [fillnull#00] and stop
    // 000044: fill_nop.r32 (I64)
    // --> [fillnull#00] and stop
    // 000044: fill_nop.i32 (I32)
    // --> [fillnull#00] and stop
    // 000044: fill_nop.r32 (I32)
    // --> [fillnull#00] and stop
    // 000044: fill_nop.b1 (I32)
    // --> [fillnull#00] and stop
    // 000044: fill_nop.i8 (I32)
    // --> [fillnull#00] and stop
    // 000044: fill_nop.i16 (I32)
    // --> [fillnull#00] and stop
    // 000044: fill_nop.i64 (I32)
    // --> [fillnull#00] and stop
    // 000044: fill_nop.r64 (I32)
    // --> [fillnull#00] and stop
    0x00b9, 0x0000,
    // end of fill_nop.r64 (I32)
    // end of fill_nop.i64 (I32)
    // end of fill_nop.i16 (I32)
    // end of fill_nop.i8 (I32)
    // end of fill_nop.b1 (I32)
    // end of fill_nop.r32 (I32)
    // end of fill_nop.i32 (I32)
    // end of fill_nop.r32 (I64)
    // end of fill_nop.i16 (I64)
    // end of fill_nop.i8 (I64)
    // end of fill_nop.b1 (I64)
    // end of fill_nop.r64 (I64)
    // end of fill_nop.i32 (I64)
    // end of fill_nop.i64 (I64)
    // 000046: func_addr.i64 (I64)
    // skip 2 unless PredicateView(15)
    0x3034,
    // --> [RexOp1fnaddr8#80b8]
    0x0266, 0x80b8,
    // skip 2 unless PredicateView(13)
    0x3032,
    // --> [RexOp1allones_fnaddr8#80b8]
    0x026a, 0x80b8,
    // skip 2 unless inst_predicate_34
    0x3022,
    // --> [RexOp1pcrel_fnaddr8#808d]
    0x026c, 0x808d,
    // stop unless PredicateView(14)
    0x1033,
    // --> [RexOp1got_fnaddr8#808b] and stop
    0x026f, 0x808b,
    // end of func_addr.i64 (I64)
    // 000052: get_pinned_reg.i64 (I64)
    // --> [get_pinned_reg#00] and stop
    0x0001, 0x0000,
    // end of get_pinned_reg.i64 (I64)
    // 000054: iadd.i64 (I64)
    // --> [RexOp1rr#8001] and stop
    0x014f, 0x8001,
    // end of iadd.i64 (I64)
    // 000056: iadd_ifcarry.i64 (I64)
    // --> [RexOp1rio#8011] and stop
    0x015b, 0x8011,
    // end of iadd_ifcarry.i64 (I64)
    // 000058: iadd_ifcin.i64 (I64)
    // --> [RexOp1rin#8011] and stop
    0x0157, 0x8011,
    // end of iadd_ifcin.i64 (I64)
    // 00005a: iadd_ifcout.i64 (I64)
    // --> [RexOp1rout#8001] and stop
    0x0153, 0x8001,
    // end of iadd_ifcout.i64 (I64)
    // 00005c: iadd_imm.i64 (I64)
    // --> [RexOp1r_ib#8083]
    0x015e, 0x8083,
    // --> [RexOp1r_id#8081] and stop
    0x0163, 0x8081,
    // end of iadd_imm.i64 (I64)
    // 000060: icmp.i64 (I64)
    // --> [RexOp1icscc#8039] and stop
    0x018f, 0x8039,
    // end of icmp.i64 (I64)
    // 000062: icmp_imm.i64 (I64)
    // --> [RexOp1icscc_ib#f083]
    0x0192, 0xf083,
    // --> [RexOp1icscc_id#f081] and stop
    0x0197, 0xf081,
    // end of icmp_imm.i64 (I64)
    // 000066: iconst.i64 (I64)
    // skip 4 unless inst_predicate_0
    0x5000,
    // --> [RexOp1pu_id#b8]
    0x0010, 0x00b8,
    // --> [Op1pu_id#b8]
    0x000e, 0x00b8,
    // --> [RexOp1u_id#80c7]
    0x0012, 0x80c7,
    // --> [RexOp1pu_iq#80b8]
    0x0014, 0x80b8,
    // stop unless inst_predicate_1
    // 00006f: iconst.i16 (I64)
    // stop unless inst_predicate_1
    0x1001,
    // --> [RexOp1u_id_z#31]
    // --> [RexOp1u_id_z#31]
    0x001c, 0x0031,
    // --> [Op1u_id_z#31] and stop
    // --> [Op1u_id_z#31] and stop
    0x001b, 0x0031,
    // end of iconst.i16 (I64)
    // end of iconst.i64 (I64)
    // 000074: ifcmp.i64 (I64)
    // --> [RexOp1rcmp#8039] and stop
    0x019b, 0x8039,
    // end of ifcmp.i64 (I64)
    // 000076: ifcmp_imm.i64 (I64)
    // --> [RexOp1rcmp_ib#f083]
    0x019e, 0xf083,
    // --> [RexOp1rcmp_id#f081] and stop
    0x01a3, 0xf081,
    // end of ifcmp_imm.i64 (I64)
    // 00007a: ifcmp_sp.i64 (I64)
    // --> [RexOp1rcmp_sp#8039] and stop
    0x01a7, 0x8039,
    // end of ifcmp_sp.i64 (I64)
    // 00007c: imul.i64 (I64)
    // --> [RexOp2rrx#84af] and stop
    0x016f, 0x84af,
    // end of imul.i64 (I64)
    // 00007e: indirect_jump_table_br.i64 (I64)
    // --> [RexOp1indirect_jmp#40ff]
    0x02b8, 0x40ff,
    // --> [Op1indirect_jmp#40ff] and stop
    // 000080: indirect_jump_table_br.i32 (I32)
    // --> [Op1indirect_jmp#40ff] and stop
    0x02bb, 0x40ff,
    // end of indirect_jump_table_br.i32 (I32)
    // end of indirect_jump_table_br.i64 (I64)
    // 000082: ishl.i64 (I64)
    // --> [RexOp1rc#c0d3] and stop
    0x0183, 0xc0d3,
    // end of ishl.i64 (I64)
    // 000084: ishl_imm.i64 (I64)
    // --> [RexOp1r_ib#c0c1] and stop
    0x015f, 0xc0c1,
    // end of ishl_imm.i64 (I64)
    // 000086: istore16.i64 (I64)
    // --> [RexMp1st#189]
    // 000086: istore16.i32 (I64)
    // --> [RexMp1st#189]
    0x0078, 0x0189,
    // --> [Mp1st#189]
    // --> [Mp1st#189]
    0x0076, 0x0189,
    // --> [RexMp1stDisp8#189]
    // --> [RexMp1stDisp8#189]
    0x0080, 0x0189,
    // --> [Mp1stDisp8#189]
    // --> [Mp1stDisp8#189]
    0x007e, 0x0189,
    // --> [RexMp1stDisp32#189]
    // --> [RexMp1stDisp32#189]
    0x0088, 0x0189,
    // --> [Mp1stDisp32#189] and stop
    // --> [Mp1stDisp32#189] and stop
    0x0087, 0x0189,
    // end of istore16.i32 (I64)
    // end of istore16.i64 (I64)
    // 000092: istore16_complex.i64 (I64)
    // stop unless inst_predicate_12
    // 000092: istore16_complex.i32 (I64)
    // stop unless inst_predicate_12
    0x100c,
    // --> [RexMp1stWithIndex#189]
    // --> [RexMp1stWithIndex#189]
    0x0054, 0x0189,
    // --> [Mp1stWithIndex#189]
    // --> [Mp1stWithIndex#189]
    0x0052, 0x0189,
    // --> [RexMp1stWithIndexDisp8#189]
    // --> [RexMp1stWithIndexDisp8#189]
    0x005c, 0x0189,
    // --> [Mp1stWithIndexDisp8#189]
    // --> [Mp1stWithIndexDisp8#189]
    0x005a, 0x0189,
    // --> [RexMp1stWithIndexDisp32#189]
    // --> [RexMp1stWithIndexDisp32#189]
    0x0064, 0x0189,
    // --> [Mp1stWithIndexDisp32#189] and stop
    // --> [Mp1stWithIndexDisp32#189] and stop
    0x0063, 0x0189,
    // end of istore16_complex.i32 (I64)
    // end of istore16_complex.i64 (I64)
    // 00009f: istore32.i64 (I64)
    // --> [RexOp1st#89]
    // 00009f: store.i32 (I64)
    // --> [RexOp1st#89]
    // 00009f: store.r32 (I64)
    // --> [RexOp1st#89]
    0x0074, 0x0089,
    // --> [Op1st#89]
    // --> [Op1st#89]
    // --> [Op1st#89]
    0x0072, 0x0089,
    // --> [RexOp1stDisp8#89]
    // --> [RexOp1stDisp8#89]
    // --> [RexOp1stDisp8#89]
    0x007c, 0x0089,
    // --> [Op1stDisp8#89]
    // --> [Op1stDisp8#89]
    // --> [Op1stDisp8#89]
    0x007a, 0x0089,
    // --> [RexOp1stDisp32#89]
    // --> [RexOp1stDisp32#89]
    // --> [RexOp1stDisp32#89]
    0x0084, 0x0089,
    // --> [Op1stDisp32#89] and stop
    // --> [Op1stDisp32#89] and stop
    // --> [Op1stDisp32#89] and stop
    0x0083, 0x0089,
    // end of store.r32 (I64)
    // end of store.i32 (I64)
    // end of istore32.i64 (I64)
    // 0000ab: istore8.i64 (I64)
    // --> [RexOp1st#88]
    // 0000ab: istore8.i32 (I64)
    // --> [RexOp1st#88]
    0x0074, 0x0088,
    // --> [Op1st_abcd#88]
    // --> [Op1st_abcd#88]
    0x008a, 0x0088,
    // --> [RexOp1stDisp8#88]
    // --> [RexOp1stDisp8#88]
    0x007c, 0x0088,
    // --> [Op1stDisp8_abcd#88]
    // --> [Op1stDisp8_abcd#88]
    0x008c, 0x0088,
    // --> [RexOp1stDisp32#88]
    // --> [RexOp1stDisp32#88]
    0x0084, 0x0088,
    // --> [Op1stDisp32_abcd#88] and stop
    // --> [Op1stDisp32_abcd#88] and stop
    0x008f, 0x0088,
    // end of istore8.i32 (I64)
    // end of istore8.i64 (I64)
    // 0000b7: istore8_complex.i64 (I64)
    // stop unless inst_predicate_12
    // 0000b7: istore8_complex.i32 (I64)
    // stop unless inst_predicate_12
    0x100c,
    // --> [RexOp1stWithIndex_abcd#88]
    // --> [RexOp1stWithIndex_abcd#88]
    0x0068, 0x0088,
    // --> [Op1stWithIndex_abcd#88]
    // --> [Op1stWithIndex_abcd#88]
    0x0066, 0x0088,
    // --> [RexOp1stWithIndexDisp8_abcd#88]
    // --> [RexOp1stWithIndexDisp8_abcd#88]
    0x006c, 0x0088,
    // --> [Op1stWithIndexDisp8_abcd#88]
    // --> [Op1stWithIndexDisp8_abcd#88]
    0x006a, 0x0088,
    // --> [RexOp1stWithIndexDisp32_abcd#88]
    // --> [RexOp1stWithIndexDisp32_abcd#88]
    0x0070, 0x0088,
    // --> [Op1stWithIndexDisp32_abcd#88] and stop
    // --> [Op1stWithIndexDisp32_abcd#88] and stop
    0x006f, 0x0088,
    // end of istore8_complex.i32 (I64)
    // end of istore8_complex.i64 (I64)
    // 0000c4: isub.i64 (I64)
    // --> [RexOp1rr#8029] and stop
    0x014f, 0x8029,
    // end of isub.i64 (I64)
    // 0000c6: isub_ifbin.i64 (I64)
    // --> [RexOp1rin#8019] and stop
    0x0157, 0x8019,
    // end of isub_ifbin.i64 (I64)
    // 0000c8: isub_ifborrow.i64 (I64)
    // --> [RexOp1rio#8019] and stop
    0x015b, 0x8019,
    // end of isub_ifborrow.i64 (I64)
    // 0000ca: isub_ifbout.i64 (I64)
    // --> [RexOp1rout#8029] and stop
    0x0153, 0x8029,
    // end of isub_ifbout.i64 (I64)
    // 0000cc: jump_table_base.i64 (I64)
    // --> [RexOp1jt_base#808d] and stop
    0x02b5, 0x808d,
    // end of jump_table_base.i64 (I64)
    // 0000ce: jump_table_entry.i64 (I64)
    // --> [RexOp1jt_entry#8063] and stop
    0x02b1, 0x8063,
    // end of jump_table_entry.i64 (I64)
    // 0000d0: load.i64 (I64)
    // --> [RexOp1ld#808b]
    // 0000d0: load.r64 (I64)
    // --> [RexOp1ld#808b]
    0x009a, 0x808b,
    // --> [RexOp1ldDisp8#808b]
    // --> [RexOp1ldDisp8#808b]
    0x00a2, 0x808b,
    // --> [RexOp1ldDisp32#808b] and stop
    // --> [RexOp1ldDisp32#808b] and stop
    0x00ab, 0x808b,
    // end of load.r64 (I64)
    // end of load.i64 (I64)
    // 0000d6: load_complex.i64 (I64)
    // stop unless inst_predicate_11
    // 0000d6: load_complex.r64 (I64)
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexOp1ldWithIndex#808b]
    // --> [RexOp1ldWithIndex#808b]
    0x0038, 0x808b,
    // --> [RexOp1ldWithIndexDisp8#808b]
    // --> [RexOp1ldWithIndexDisp8#808b]
    0x0040, 0x808b,
    // --> [RexOp1ldWithIndexDisp32#808b] and stop
    // --> [RexOp1ldWithIndexDisp32#808b] and stop
    0x0049, 0x808b,
    // end of load_complex.r64 (I64)
    // end of load_complex.i64 (I64)
    // 0000dd: popcnt.i64 (I64)
    // stop unless PredicateView(24)
    0x103d,
    // --> [RexMp2urm#86b8] and stop
    0x0187, 0x86b8,
    // end of popcnt.i64 (I64)
    // 0000e0: regfill.i64 (I64)
    // --> [RexOp1regfill32#808b] and stop
    // 0000e0: regfill.r64 (I64)
    // --> [RexOp1regfill32#808b] and stop
    0x00b7, 0x808b,
    // end of regfill.r64 (I64)
    // end of regfill.i64 (I64)
    // 0000e2: regmove.i64 (I64)
    // --> [RexOp1rmov#8089] and stop
    // 0000e2: regmove.r64 (I64)
    // --> [RexOp1rmov#8089] and stop
    0x000d, 0x8089,
    // end of regmove.r64 (I64)
    // end of regmove.i64 (I64)
    // 0000e4: regspill.i64 (I64)
    // --> [RexOp1regspill32#8089] and stop
    // 0000e4: regspill.r64 (I64)
    // --> [RexOp1regspill32#8089] and stop
    0x0097, 0x8089,
    // end of regspill.r64 (I64)
    // end of regspill.i64 (I64)
    // 0000e6: rotl.i64 (I64)
    // --> [RexOp1rc#80d3] and stop
    0x0183, 0x80d3,
    // end of rotl.i64 (I64)
    // 0000e8: rotl_imm.i64 (I64)
    // --> [RexOp1r_ib#80c1] and stop
    0x015f, 0x80c1,
    // end of rotl_imm.i64 (I64)
    // 0000ea: rotr.i64 (I64)
    // --> [RexOp1rc#90d3] and stop
    0x0183, 0x90d3,
    // end of rotr.i64 (I64)
    // 0000ec: rotr_imm.i64 (I64)
    // --> [RexOp1r_ib#90c1] and stop
    0x015f, 0x90c1,
    // end of rotr_imm.i64 (I64)
    // 0000ee: selectif.i64 (I64)
    // --> [RexOp2cmov#8440] and stop
    // 0000ee: selectif_spectre_guard.i64 (I64)
    // --> [RexOp2cmov#8440] and stop
    0x01b3, 0x8440,
    // end of selectif_spectre_guard.i64 (I64)
    // end of selectif.i64 (I64)
    // 0000f0: set_pinned_reg.i64 (I64)
    // --> [RexOp1set_pinned_reg#8089]
    0x0002, 0x8089,
    // --> [RexOp1set_pinned_reg#8089] and stop
    0x0003, 0x8089,
    // end of set_pinned_reg.i64 (I64)
    // 0000f4: sextend.i64 (I64)
    // skip 2 unless inst_predicate_5
    0x3005,
    // --> [RexOp2urm_noflags#84be]
    0x0022, 0x84be,
    // skip 2 unless inst_predicate_2
    0x3002,
    // --> [RexOp2urm_noflags#84bf]
    0x0022, 0x84bf,
    // stop unless inst_predicate_3
    0x1003,
    // --> [RexOp1urm_noflags#8063] and stop
    0x0027, 0x8063,
    // end of sextend.i64 (I64)
    // 0000fd: sload16.i64 (I64)
    // --> [RexOp2ld#84bf]
    0x009e, 0x84bf,
    // --> [RexOp2ldDisp8#84bf]
    0x00a6, 0x84bf,
    // --> [RexOp2ldDisp32#84bf] and stop
    0x00af, 0x84bf,
    // end of sload16.i64 (I64)
    // 000103: sload16_complex.i64 (I64)
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexOp2ldWithIndex#84bf]
    0x003c, 0x84bf,
    // --> [RexOp2ldWithIndexDisp8#84bf]
    0x0044, 0x84bf,
    // --> [RexOp2ldWithIndexDisp32#84bf] and stop
    0x004d, 0x84bf,
    // end of sload16_complex.i64 (I64)
    // 00010a: sload16x4.i64 (I64)
    // stop unless PredicateView(26)
    // 00010a: sload16x4.i32 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3fld#923]
    // --> [DynRexMp3fld#923]
    0x022e, 0x0923,
    // --> [DynRexMp3fldDisp8#923]
    // --> [DynRexMp3fldDisp8#923]
    0x0232, 0x0923,
    // --> [DynRexMp3fldDisp32#923] and stop
    // --> [DynRexMp3fldDisp32#923] and stop
    0x0237, 0x0923,
    // end of sload16x4.i32 (I64)
    // end of sload16x4.i64 (I64)
    // 000111: sload32.i64 (I64)
    // --> [RexOp1ld#8063]
    0x009a, 0x8063,
    // --> [RexOp1ldDisp8#8063]
    0x00a2, 0x8063,
    // --> [RexOp1ldDisp32#8063] and stop
    0x00ab, 0x8063,
    // end of sload32.i64 (I64)
    // 000117: sload32x2.i64 (I64)
    // stop unless PredicateView(26)
    // 000117: sload32x2.i32 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3fld#925]
    // --> [DynRexMp3fld#925]
    0x022e, 0x0925,
    // --> [DynRexMp3fldDisp8#925]
    // --> [DynRexMp3fldDisp8#925]
    0x0232, 0x0925,
    // --> [DynRexMp3fldDisp32#925] and stop
    // --> [DynRexMp3fldDisp32#925] and stop
    0x0237, 0x0925,
    // end of sload32x2.i32 (I64)
    // end of sload32x2.i64 (I64)
    // 00011e: sload8.i64 (I64)
    // --> [RexOp2ld#84be]
    0x009e, 0x84be,
    // --> [RexOp2ldDisp8#84be]
    0x00a6, 0x84be,
    // --> [RexOp2ldDisp32#84be] and stop
    0x00af, 0x84be,
    // end of sload8.i64 (I64)
    // 000124: sload8_complex.i64 (I64)
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexOp2ldWithIndex#84be]
    0x003c, 0x84be,
    // --> [RexOp2ldWithIndexDisp8#84be]
    0x0044, 0x84be,
    // --> [RexOp2ldWithIndexDisp32#84be] and stop
    0x004d, 0x84be,
    // end of sload8_complex.i64 (I64)
    // 00012b: sload8x8.i64 (I64)
    // stop unless PredicateView(26)
    // 00012b: sload8x8.i32 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3fld#920]
    // --> [DynRexMp3fld#920]
    0x022e, 0x0920,
    // --> [DynRexMp3fldDisp8#920]
    // --> [DynRexMp3fldDisp8#920]
    0x0232, 0x0920,
    // --> [DynRexMp3fldDisp32#920] and stop
    // --> [DynRexMp3fldDisp32#920] and stop
    0x0237, 0x0920,
    // end of sload8x8.i32 (I64)
    // end of sload8x8.i64 (I64)
    // 000132: spill.i64 (I64)
    // --> [RexOp1spillSib32#8089] and stop
    // 000132: spill.r64 (I64)
    // --> [RexOp1spillSib32#8089] and stop
    0x0093, 0x8089,
    // end of spill.r64 (I64)
    // end of spill.i64 (I64)
    // 000134: sshr.i64 (I64)
    // --> [RexOp1rc#f0d3] and stop
    0x0183, 0xf0d3,
    // end of sshr.i64 (I64)
    // 000136: sshr_imm.i64 (I64)
    // --> [RexOp1r_ib#f0c1] and stop
    0x015f, 0xf0c1,
    // end of sshr_imm.i64 (I64)
    // 000138: stack_addr.i64 (I64)
    // --> [RexOp1spaddr_id#808d] and stop
    0x0279, 0x808d,
    // end of stack_addr.i64 (I64)
    // 00013a: store.i64 (I64)
    // --> [RexOp1st#8089]
    // 00013a: store.r64 (I64)
    // --> [RexOp1st#8089]
    0x0074, 0x8089,
    // --> [RexOp1stDisp8#8089]
    // --> [RexOp1stDisp8#8089]
    0x007c, 0x8089,
    // --> [RexOp1stDisp32#8089] and stop
    // --> [RexOp1stDisp32#8089] and stop
    0x0085, 0x8089,
    // end of store.r64 (I64)
    // end of store.i64 (I64)
    // 000140: store_complex.i64 (I64)
    // stop unless inst_predicate_12
    // 000140: store_complex.r64 (I64)
    // stop unless inst_predicate_12
    0x100c,
    // --> [RexOp1stWithIndex#8089]
    // --> [RexOp1stWithIndex#8089]
    0x0050, 0x8089,
    // --> [RexOp1stWithIndexDisp8#8089]
    // --> [RexOp1stWithIndexDisp8#8089]
    0x0058, 0x8089,
    // --> [RexOp1stWithIndexDisp32#8089] and stop
    // --> [RexOp1stWithIndexDisp32#8089] and stop
    0x0061, 0x8089,
    // end of store_complex.r64 (I64)
    // end of store_complex.i64 (I64)
    // 000147: symbol_value.i64 (I64)
    // skip 2 unless PredicateView(16)
    0x3035,
    // --> [RexOp1gvaddr8#80b8]
    0x0272, 0x80b8,
    // skip 3 unless PredicateView(14)
    0x4033,
    // skip 2 unless inst_predicate_35
    0x3023,
    // --> [RexOp1pcrel_gvaddr8#808d]
    0x0274, 0x808d,
    // stop unless PredicateView(14)
    0x1033,
    // --> [RexOp1got_gvaddr8#808b] and stop
    0x0277, 0x808b,
    // end of symbol_value.i64 (I64)
    // 000151: uextend.i64 (I64)
    // skip 4 unless inst_predicate_5
    0x5005,
    // --> [RexOp2urm_noflags#4b6]
    0x0022, 0x04b6,
    // --> [Op2urm_noflags_abcd#4b6]
    0x0020, 0x04b6,
    // skip 4 unless inst_predicate_2
    0x5002,
    // --> [RexOp2urm_noflags#4b7]
    0x0022, 0x04b7,
    // --> [Op2urm_noflags#4b7]
    0x0024, 0x04b7,
    // stop unless inst_predicate_3
    0x1003,
    // --> [RexOp1umr#89]
    // 00015c: copy.b1 (I64)
    // --> [RexOp1umr#89]
    // 00015c: copy.i8 (I64)
    // --> [RexOp1umr#89]
    // 00015c: copy.i16 (I64)
    // --> [RexOp1umr#89]
    0x0006, 0x0089,
    // --> [Op1umr#89] and stop
    // --> [Op1umr#89] and stop
    // --> [Op1umr#89] and stop
    // --> [Op1umr#89] and stop
    // 00015e: copy.r32 (I32)
    // --> [Op1umr#89] and stop
    // 00015e: copy.b1 (I32)
    // --> [Op1umr#89] and stop
    // 00015e: copy.i8 (I32)
    // --> [Op1umr#89] and stop
    // 00015e: copy.i16 (I32)
    // --> [Op1umr#89] and stop
    0x0009, 0x0089,
    // end of copy.i16 (I32)
    // end of copy.i8 (I32)
    // end of copy.b1 (I32)
    // end of copy.r32 (I32)
    // end of copy.i16 (I64)
    // end of copy.i8 (I64)
    // end of copy.b1 (I64)
    // end of uextend.i64 (I64)
    // 000160: uload16.i64 (I64)
    // --> [RexOp2ld#84b7]
    0x009e, 0x84b7,
    // --> [RexOp2ldDisp8#84b7]
    0x00a6, 0x84b7,
    // --> [RexOp2ldDisp32#84b7] and stop
    0x00af, 0x84b7,
    // end of uload16.i64 (I64)
    // 000166: uload16_complex.i64 (I64)
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexOp2ldWithIndex#84b7]
    0x003c, 0x84b7,
    // --> [RexOp2ldWithIndexDisp8#84b7]
    0x0044, 0x84b7,
    // --> [RexOp2ldWithIndexDisp32#84b7] and stop
    0x004d, 0x84b7,
    // end of uload16_complex.i64 (I64)
    // 00016d: uload16x4.i64 (I64)
    // stop unless PredicateView(26)
    // 00016d: uload16x4.i32 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3fld#933]
    // --> [DynRexMp3fld#933]
    0x022e, 0x0933,
    // --> [DynRexMp3fldDisp8#933]
    // --> [DynRexMp3fldDisp8#933]
    0x0232, 0x0933,
    // --> [DynRexMp3fldDisp32#933] and stop
    // --> [DynRexMp3fldDisp32#933] and stop
    0x0237, 0x0933,
    // end of uload16x4.i32 (I64)
    // end of uload16x4.i64 (I64)
    // 000174: uload32.i64 (I64)
    // --> [RexOp1ld#8b]
    // 000174: load.i32 (I64)
    // --> [RexOp1ld#8b]
    // 000174: load.r32 (I64)
    // --> [RexOp1ld#8b]
    0x009a, 0x008b,
    // --> [Op1ld#8b]
    // --> [Op1ld#8b]
    // --> [Op1ld#8b]
    0x0098, 0x008b,
    // --> [RexOp1ldDisp8#8b]
    // --> [RexOp1ldDisp8#8b]
    // --> [RexOp1ldDisp8#8b]
    0x00a2, 0x008b,
    // --> [Op1ldDisp8#8b]
    // --> [Op1ldDisp8#8b]
    // --> [Op1ldDisp8#8b]
    0x00a0, 0x008b,
    // --> [RexOp1ldDisp32#8b]
    // --> [RexOp1ldDisp32#8b]
    // --> [RexOp1ldDisp32#8b]
    0x00aa, 0x008b,
    // --> [Op1ldDisp32#8b] and stop
    // --> [Op1ldDisp32#8b] and stop
    // --> [Op1ldDisp32#8b] and stop
    0x00a9, 0x008b,
    // end of load.r32 (I64)
    // end of load.i32 (I64)
    // end of uload32.i64 (I64)
    // 000180: uload32x2.i64 (I64)
    // stop unless PredicateView(26)
    // 000180: uload32x2.i32 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3fld#935]
    // --> [DynRexMp3fld#935]
    0x022e, 0x0935,
    // --> [DynRexMp3fldDisp8#935]
    // --> [DynRexMp3fldDisp8#935]
    0x0232, 0x0935,
    // --> [DynRexMp3fldDisp32#935] and stop
    // --> [DynRexMp3fldDisp32#935] and stop
    0x0237, 0x0935,
    // end of uload32x2.i32 (I64)
    // end of uload32x2.i64 (I64)
    // 000187: uload8.i64 (I64)
    // --> [RexOp2ld#84b6]
    0x009e, 0x84b6,
    // --> [RexOp2ldDisp8#84b6]
    0x00a6, 0x84b6,
    // --> [RexOp2ldDisp32#84b6] and stop
    0x00af, 0x84b6,
    // end of uload8.i64 (I64)
    // 00018d: uload8_complex.i64 (I64)
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexOp2ldWithIndex#84b6]
    0x003c, 0x84b6,
    // --> [RexOp2ldWithIndexDisp8#84b6]
    0x0044, 0x84b6,
    // --> [RexOp2ldWithIndexDisp32#84b6] and stop
    0x004d, 0x84b6,
    // end of uload8_complex.i64 (I64)
    // 000194: uload8x8.i64 (I64)
    // stop unless PredicateView(26)
    // 000194: uload8x8.i32 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3fld#930]
    // --> [DynRexMp3fld#930]
    0x022e, 0x0930,
    // --> [DynRexMp3fldDisp8#930]
    // --> [DynRexMp3fldDisp8#930]
    0x0232, 0x0930,
    // --> [DynRexMp3fldDisp32#930] and stop
    // --> [DynRexMp3fldDisp32#930] and stop
    0x0237, 0x0930,
    // end of uload8x8.i32 (I64)
    // end of uload8x8.i64 (I64)
    // 00019b: ushr.i64 (I64)
    // --> [RexOp1rc#d0d3] and stop
    0x0183, 0xd0d3,
    // end of ushr.i64 (I64)
    // 00019d: ushr_imm.i64 (I64)
    // --> [RexOp1r_ib#d0c1] and stop
    0x015f, 0xd0c1,
    // end of ushr_imm.i64 (I64)
    // 00019f: x86_bsf.i64 (I64)
    // --> [RexOp2bsf_and_bsr#84bc] and stop
    0x018b, 0x84bc,
    // end of x86_bsf.i64 (I64)
    // 0001a1: x86_bsr.i64 (I64)
    // --> [RexOp2bsf_and_bsr#84bd] and stop
    0x018b, 0x84bd,
    // end of x86_bsr.i64 (I64)
    // 0001a3: x86_cvtt2si.i64 (I64)
    // skip 2 unless inst_predicate_13
    0x300d,
    // --> [RexMp2rfurm#862c]
    0x0132, 0x862c,
    // stop unless inst_predicate_14
    0x100e,
    // --> [RexMp2rfurm#872c] and stop
    0x0133, 0x872c,
    // end of x86_cvtt2si.i64 (I64)
    // 0001a9: x86_pop.i64 (I64)
    // --> [RexOp1popq#58]
    0x00c2, 0x0058,
    // --> [Op1popq#58] and stop
    // 0001ab: x86_pop.i32 (I32)
    // --> [Op1popq#58] and stop
    0x00c1, 0x0058,
    // end of x86_pop.i32 (I32)
    // end of x86_pop.i64 (I64)
    // 0001ad: x86_push.i64 (I64)
    // --> [RexOp1pushq#50]
    0x00be, 0x0050,
    // --> [Op1pushq#50] and stop
    // 0001af: x86_push.i32 (I32)
    // --> [Op1pushq#50] and stop
    0x00bd, 0x0050,
    // end of x86_push.i32 (I32)
    // end of x86_push.i64 (I64)
    // 0001b1: x86_sdivmodx.i64 (I64)
    // --> [RexOp1div#f0f7] and stop
    0x0173, 0xf0f7,
    // end of x86_sdivmodx.i64 (I64)
    // 0001b3: x86_smulx.i64 (I64)
    // --> [RexOp1mulx#d0f7] and stop
    0x0177, 0xd0f7,
    // end of x86_smulx.i64 (I64)
    // 0001b5: x86_udivmodx.i64 (I64)
    // --> [RexOp1div#e0f7] and stop
    0x0173, 0xe0f7,
    // end of x86_udivmodx.i64 (I64)
    // 0001b7: x86_umulx.i64 (I64)
    // --> [RexOp1mulx#c0f7] and stop
    0x0177, 0xc0f7,
    // end of x86_umulx.i64 (I64)
    // 0001b9: band.i32 (I64)
    // --> [DynRexOp1rr#21] and stop
    // 0001b9: band.b32 (I64)
    // --> [DynRexOp1rr#21] and stop
    // 0001b9: band.i32 (I32)
    // --> [DynRexOp1rr#21] and stop
    // 0001b9: band.b32 (I32)
    // --> [DynRexOp1rr#21] and stop
    0x014d, 0x0021,
    // end of band.b32 (I32)
    // end of band.i32 (I32)
    // end of band.b32 (I64)
    // end of band.i32 (I64)
    // 0001bb: band_imm.i32 (I64)
    // --> [DynRexOp1r_ib#4083]
    // 0001bb: band_imm.i32 (I32)
    // --> [DynRexOp1r_ib#4083]
    0x015c, 0x4083,
    // --> [DynRexOp1r_id#4081] and stop
    // --> [DynRexOp1r_id#4081] and stop
    0x0161, 0x4081,
    // end of band_imm.i32 (I32)
    // end of band_imm.i32 (I64)
    // 0001bf: bint.i32 (I64)
    // skip 4 unless inst_predicate_6
    0x5006,
    // --> [RexOp2urm_noflags#4b6]
    0x0022, 0x04b6,
    // --> [Op2urm_noflags_abcd#4b6]
    0x0020, 0x04b6,
    // skip 4 unless inst_predicate_7
    0x5007,
    // --> [RexOp2urm_noflags#4b6]
    0x0022, 0x04b6,
    // --> [Op2urm_noflags_abcd#4b6]
    0x0020, 0x04b6,
    // stop unless inst_predicate_9
    0x1009,
    // --> [RexOp2urm_noflags#4b6]
    0x0022, 0x04b6,
    // --> [Op2urm_noflags_abcd#4b6] and stop
    0x0021, 0x04b6,
    // end of bint.i32 (I64)
    // 0001ce: bitcast.i32 (I64)
    // stop unless inst_predicate_13
    0x100d,
    // --> [RexMp2rfumr#57e]
    0x00d8, 0x057e,
    // --> [Mp2rfumr#57e] and stop
    0x00d7, 0x057e,
    // end of bitcast.i32 (I64)
    // 0001d3: bnot.i32 (I64)
    // --> [DynRexOp1ur#20f7] and stop
    // 0001d3: bnot.b32 (I64)
    // --> [DynRexOp1ur#20f7] and stop
    // 0001d3: bnot.i32 (I32)
    // --> [DynRexOp1ur#20f7] and stop
    // 0001d3: bnot.b32 (I32)
    // --> [DynRexOp1ur#20f7] and stop
    0x0165, 0x20f7,
    // end of bnot.b32 (I32)
    // end of bnot.i32 (I32)
    // end of bnot.b32 (I64)
    // end of bnot.i32 (I64)
    // 0001d5: bor.i32 (I64)
    // --> [DynRexOp1rr#09] and stop
    // 0001d5: bor.b32 (I64)
    // --> [DynRexOp1rr#09] and stop
    // 0001d5: bor.i32 (I32)
    // --> [DynRexOp1rr#09] and stop
    // 0001d5: bor.b32 (I32)
    // --> [DynRexOp1rr#09] and stop
    0x014d, 0x0009,
    // end of bor.b32 (I32)
    // end of bor.i32 (I32)
    // end of bor.b32 (I64)
    // end of bor.i32 (I64)
    // 0001d7: bor_imm.i32 (I64)
    // --> [DynRexOp1r_ib#1083]
    // 0001d7: bor_imm.i32 (I32)
    // --> [DynRexOp1r_ib#1083]
    0x015c, 0x1083,
    // --> [DynRexOp1r_id#1081] and stop
    // --> [DynRexOp1r_id#1081] and stop
    0x0161, 0x1081,
    // end of bor_imm.i32 (I32)
    // end of bor_imm.i32 (I64)
    // 0001db: brnz.i32 (I64)
    // --> [RexOp1tjccb#75]
    0x02a0, 0x0075,
    // --> [Op1tjccb#75]
    0x029e, 0x0075,
    // --> [RexOp1tjccd#85]
    0x02a4, 0x0085,
    // --> [Op1tjccd#85] and stop
    0x02a3, 0x0085,
    // end of brnz.i32 (I64)
    // 0001e3: brz.i32 (I64)
    // --> [RexOp1tjccb#74]
    0x02a0, 0x0074,
    // --> [Op1tjccb#74]
    0x029e, 0x0074,
    // --> [RexOp1tjccd#84]
    0x02a4, 0x0084,
    // --> [Op1tjccd#84] and stop
    0x02a3, 0x0084,
    // end of brz.i32 (I64)
    // 0001eb: bxor.i32 (I64)
    // --> [DynRexOp1rr#31] and stop
    // 0001eb: bxor.b32 (I64)
    // --> [DynRexOp1rr#31] and stop
    // 0001eb: bxor.i32 (I32)
    // --> [DynRexOp1rr#31] and stop
    // 0001eb: bxor.b32 (I32)
    // --> [DynRexOp1rr#31] and stop
    0x014d, 0x0031,
    // end of bxor.b32 (I32)
    // end of bxor.i32 (I32)
    // end of bxor.b32 (I64)
    // end of bxor.i32 (I64)
    // 0001ed: bxor_imm.i32 (I64)
    // --> [DynRexOp1r_ib#6083]
    // 0001ed: bxor_imm.i32 (I32)
    // --> [DynRexOp1r_ib#6083]
    0x015c, 0x6083,
    // --> [DynRexOp1r_id#6081] and stop
    // --> [DynRexOp1r_id#6081] and stop
    0x0161, 0x6081,
    // end of bxor_imm.i32 (I32)
    // end of bxor_imm.i32 (I64)
    // 0001f1: clz.i32 (I64)
    // stop unless PredicateView(23)
    0x103c,
    // --> [RexMp2urm#6bd]
    0x0186, 0x06bd,
    // --> [Mp2urm#6bd] and stop
    0x0185, 0x06bd,
    // end of clz.i32 (I64)
    // 0001f6: copy.i32 (I64)
    // --> [DynRexOp1umr#89] and stop
    // 0001f6: copy.i32 (I32)
    // --> [DynRexOp1umr#89] and stop
    0x0005, 0x0089,
    // end of copy.i32 (I32)
    // end of copy.i32 (I64)
    // 0001f8: copy_to_ssa.i32 (I64)
    // --> [RexOp1umr_reg_to_ssa#89] and stop
    // 0001f8: copy_to_ssa.b1 (I64)
    // --> [RexOp1umr_reg_to_ssa#89] and stop
    // 0001f8: copy_to_ssa.i8 (I64)
    // --> [RexOp1umr_reg_to_ssa#89] and stop
    // 0001f8: copy_to_ssa.i16 (I64)
    // --> [RexOp1umr_reg_to_ssa#89] and stop
    0x002f, 0x0089,
    // end of copy_to_ssa.i16 (I64)
    // end of copy_to_ssa.i8 (I64)
    // end of copy_to_ssa.b1 (I64)
    // end of copy_to_ssa.i32 (I64)
    // 0001fa: ctz.i32 (I64)
    // stop unless PredicateView(22)
    0x103b,
    // --> [RexMp2urm#6bc]
    0x0186, 0x06bc,
    // --> [Mp2urm#6bc] and stop
    0x0185, 0x06bc,
    // end of ctz.i32 (I64)
    // 0001ff: fill.i32 (I64)
    // --> [RexOp1fillSib32#8b]
    // 0001ff: fill.b1 (I64)
    // --> [RexOp1fillSib32#8b]
    // 0001ff: fill.i8 (I64)
    // --> [RexOp1fillSib32#8b]
    // 0001ff: fill.i16 (I64)
    // --> [RexOp1fillSib32#8b]
    0x00b2, 0x008b,
    // --> [Op1fillSib32#8b] and stop
    // --> [Op1fillSib32#8b] and stop
    // --> [Op1fillSib32#8b] and stop
    // --> [Op1fillSib32#8b] and stop
    // 000201: fill.i32 (I32)
    // --> [Op1fillSib32#8b] and stop
    // 000201: fill.r32 (I32)
    // --> [Op1fillSib32#8b] and stop
    // 000201: fill.b1 (I32)
    // --> [Op1fillSib32#8b] and stop
    // 000201: fill.i8 (I32)
    // --> [Op1fillSib32#8b] and stop
    // 000201: fill.i16 (I32)
    // --> [Op1fillSib32#8b] and stop
    0x00b1, 0x008b,
    // end of fill.i16 (I32)
    // end of fill.i8 (I32)
    // end of fill.b1 (I32)
    // end of fill.r32 (I32)
    // end of fill.i32 (I32)
    // end of fill.i16 (I64)
    // end of fill.i8 (I64)
    // end of fill.b1 (I64)
    // end of fill.i32 (I64)
    // 000203: iadd.i32 (I64)
    // --> [DynRexOp1rr#01] and stop
    // 000203: iadd.i32 (I32)
    // --> [DynRexOp1rr#01] and stop
    0x014d, 0x0001,
    // end of iadd.i32 (I32)
    // end of iadd.i32 (I64)
    // 000205: iadd_ifcarry.i32 (I64)
    // --> [DynRexOp1rio#11] and stop
    // 000205: iadd_ifcarry.i32 (I32)
    // --> [DynRexOp1rio#11] and stop
    0x0159, 0x0011,
    // end of iadd_ifcarry.i32 (I32)
    // end of iadd_ifcarry.i32 (I64)
    // 000207: iadd_ifcin.i32 (I64)
    // --> [DynRexOp1rin#11] and stop
    // 000207: iadd_ifcin.i32 (I32)
    // --> [DynRexOp1rin#11] and stop
    0x0155, 0x0011,
    // end of iadd_ifcin.i32 (I32)
    // end of iadd_ifcin.i32 (I64)
    // 000209: iadd_ifcout.i32 (I64)
    // --> [DynRexOp1rout#01] and stop
    // 000209: iadd_ifcout.i32 (I32)
    // --> [DynRexOp1rout#01] and stop
    0x0151, 0x0001,
    // end of iadd_ifcout.i32 (I32)
    // end of iadd_ifcout.i32 (I64)
    // 00020b: iadd_imm.i32 (I64)
    // --> [DynRexOp1r_ib#83]
    // 00020b: iadd_imm.i32 (I32)
    // --> [DynRexOp1r_ib#83]
    0x015c, 0x0083,
    // --> [DynRexOp1r_id#81] and stop
    // --> [DynRexOp1r_id#81] and stop
    0x0161, 0x0081,
    // end of iadd_imm.i32 (I32)
    // end of iadd_imm.i32 (I64)
    // 00020f: icmp.i32 (I64)
    // --> [DynRexOp1icscc#39] and stop
    // 00020f: icmp.i32 (I32)
    // --> [DynRexOp1icscc#39] and stop
    0x018d, 0x0039,
    // end of icmp.i32 (I32)
    // end of icmp.i32 (I64)
    // 000211: icmp_imm.i32 (I64)
    // --> [DynRexOp1icscc_ib#7083]
    // 000211: icmp_imm.i32 (I32)
    // --> [DynRexOp1icscc_ib#7083]
    0x0190, 0x7083,
    // --> [DynRexOp1icscc_id#7081] and stop
    // --> [DynRexOp1icscc_id#7081] and stop
    0x0195, 0x7081,
    // end of icmp_imm.i32 (I32)
    // end of icmp_imm.i32 (I64)
    // 000215: iconst.i32 (I64)
    // --> [RexOp1pu_id#b8]
    0x0010, 0x00b8,
    // --> [Op1pu_id#b8]
    0x000e, 0x00b8,
    // stop unless inst_predicate_1
    0x1001,
    // --> [RexOp1u_id_z#31]
    0x001c, 0x0031,
    // --> [Op1u_id_z#31] and stop
    0x001b, 0x0031,
    // end of iconst.i32 (I64)
    // 00021e: ifcmp.i32 (I64)
    // --> [DynRexOp1rcmp#39] and stop
    // 00021e: ifcmp.i32 (I32)
    // --> [DynRexOp1rcmp#39] and stop
    0x0199, 0x0039,
    // end of ifcmp.i32 (I32)
    // end of ifcmp.i32 (I64)
    // 000220: ifcmp_imm.i32 (I64)
    // --> [DynRexOp1rcmp_ib#7083]
    // 000220: ifcmp_imm.i32 (I32)
    // --> [DynRexOp1rcmp_ib#7083]
    0x019c, 0x7083,
    // --> [DynRexOp1rcmp_id#7081] and stop
    // --> [DynRexOp1rcmp_id#7081] and stop
    0x01a1, 0x7081,
    // end of ifcmp_imm.i32 (I32)
    // end of ifcmp_imm.i32 (I64)
    // 000224: imul.i32 (I64)
    // --> [DynRexOp2rrx#4af] and stop
    // 000224: imul.i32 (I32)
    // --> [DynRexOp2rrx#4af] and stop
    0x016d, 0x04af,
    // end of imul.i32 (I32)
    // end of imul.i32 (I64)
    // 000226: ireduce.i32 (I64)
    // stop unless inst_predicate_4
    0x1004,
    // --> [null#00] and stop
    0x001f, 0x0000,
    // end of ireduce.i32 (I64)
    // 000229: ishl.i32 (I64)
    // --> [RexOp1rc#40d3]
    0x0182, 0x40d3,
    // --> [Op1rc#40d3] and stop
    0x0181, 0x40d3,
    // end of ishl.i32 (I64)
    // 00022d: ishl_imm.i32 (I64)
    // --> [DynRexOp1r_ib#40c1] and stop
    // 00022d: ishl_imm.i32 (I32)
    // --> [DynRexOp1r_ib#40c1] and stop
    0x015d, 0x40c1,
    // end of ishl_imm.i32 (I32)
    // end of ishl_imm.i32 (I64)
    // 00022f: isub.i32 (I64)
    // --> [DynRexOp1rr#29] and stop
    // 00022f: isub.i32 (I32)
    // --> [DynRexOp1rr#29] and stop
    0x014d, 0x0029,
    // end of isub.i32 (I32)
    // end of isub.i32 (I64)
    // 000231: isub_ifbin.i32 (I64)
    // --> [DynRexOp1rin#19] and stop
    // 000231: isub_ifbin.i32 (I32)
    // --> [DynRexOp1rin#19] and stop
    0x0155, 0x0019,
    // end of isub_ifbin.i32 (I32)
    // end of isub_ifbin.i32 (I64)
    // 000233: isub_ifborrow.i32 (I64)
    // --> [DynRexOp1rio#19] and stop
    // 000233: isub_ifborrow.i32 (I32)
    // --> [DynRexOp1rio#19] and stop
    0x0159, 0x0019,
    // end of isub_ifborrow.i32 (I32)
    // end of isub_ifborrow.i32 (I64)
    // 000235: isub_ifbout.i32 (I64)
    // --> [DynRexOp1rout#29] and stop
    // 000235: isub_ifbout.i32 (I32)
    // --> [DynRexOp1rout#29] and stop
    0x0151, 0x0029,
    // end of isub_ifbout.i32 (I32)
    // end of isub_ifbout.i32 (I64)
    // 000237: load_complex.i32 (I64)
    // stop unless inst_predicate_11
    // 000237: load_complex.r32 (I64)
    // stop unless inst_predicate_11
    // 000237: uload32_complex (I64)
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexOp1ldWithIndex#8b]
    // --> [RexOp1ldWithIndex#8b]
    // --> [RexOp1ldWithIndex#8b]
    0x0038, 0x008b,
    // --> [Op1ldWithIndex#8b]
    // --> [Op1ldWithIndex#8b]
    // --> [Op1ldWithIndex#8b]
    0x0036, 0x008b,
    // --> [RexOp1ldWithIndexDisp8#8b]
    // --> [RexOp1ldWithIndexDisp8#8b]
    // --> [RexOp1ldWithIndexDisp8#8b]
    0x0040, 0x008b,
    // --> [Op1ldWithIndexDisp8#8b]
    // --> [Op1ldWithIndexDisp8#8b]
    // --> [Op1ldWithIndexDisp8#8b]
    0x003e, 0x008b,
    // --> [RexOp1ldWithIndexDisp32#8b]
    // --> [RexOp1ldWithIndexDisp32#8b]
    // --> [RexOp1ldWithIndexDisp32#8b]
    0x0048, 0x008b,
    // --> [Op1ldWithIndexDisp32#8b] and stop
    // --> [Op1ldWithIndexDisp32#8b] and stop
    // --> [Op1ldWithIndexDisp32#8b] and stop
    0x0047, 0x008b,
    // end of uload32_complex (I64)
    // end of load_complex.r32 (I64)
    // end of load_complex.i32 (I64)
    // 000244: popcnt.i32 (I64)
    // stop unless PredicateView(24)
    0x103d,
    // --> [RexMp2urm#6b8]
    0x0186, 0x06b8,
    // --> [Mp2urm#6b8] and stop
    0x0185, 0x06b8,
    // end of popcnt.i32 (I64)
    // 000249: regfill.i32 (I64)
    // --> [RexOp1regfill32#8b]
    // 000249: regfill.b1 (I64)
    // --> [RexOp1regfill32#8b]
    // 000249: regfill.i8 (I64)
    // --> [RexOp1regfill32#8b]
    // 000249: regfill.i16 (I64)
    // --> [RexOp1regfill32#8b]
    0x00b6, 0x008b,
    // --> [Op1regfill32#8b] and stop
    // --> [Op1regfill32#8b] and stop
    // --> [Op1regfill32#8b] and stop
    // --> [Op1regfill32#8b] and stop
    // 00024b: regfill.i32 (I32)
    // --> [Op1regfill32#8b] and stop
    // 00024b: regfill.r32 (I32)
    // --> [Op1regfill32#8b] and stop
    // 00024b: regfill.b1 (I32)
    // --> [Op1regfill32#8b] and stop
    // 00024b: regfill.i8 (I32)
    // --> [Op1regfill32#8b] and stop
    // 00024b: regfill.i16 (I32)
    // --> [Op1regfill32#8b] and stop
    0x00b5, 0x008b,
    // end of regfill.i16 (I32)
    // end of regfill.i8 (I32)
    // end of regfill.b1 (I32)
    // end of regfill.r32 (I32)
    // end of regfill.i32 (I32)
    // end of regfill.i16 (I64)
    // end of regfill.i8 (I64)
    // end of regfill.b1 (I64)
    // end of regfill.i32 (I64)
    // 00024d: regmove.i32 (I64)
    // --> [RexOp1rmov#89] and stop
    // 00024d: regmove.i16 (I64)
    // --> [RexOp1rmov#89] and stop
    // 00024d: regmove.b8 (I64)
    // --> [RexOp1rmov#89] and stop
    // 00024d: regmove.b16 (I64)
    // --> [RexOp1rmov#89] and stop
    // 00024d: regmove.b32 (I64)
    // --> [RexOp1rmov#89] and stop
    // 00024d: regmove.r32 (I64)
    // --> [RexOp1rmov#89] and stop
    0x000d, 0x0089,
    // end of regmove.r32 (I64)
    // end of regmove.b32 (I64)
    // end of regmove.b16 (I64)
    // end of regmove.b8 (I64)
    // end of regmove.i16 (I64)
    // end of regmove.i32 (I64)
    // 00024f: regspill.i32 (I64)
    // --> [RexOp1regspill32#89]
    // 00024f: regspill.b1 (I64)
    // --> [RexOp1regspill32#89]
    // 00024f: regspill.i8 (I64)
    // --> [RexOp1regspill32#89]
    // 00024f: regspill.i16 (I64)
    // --> [RexOp1regspill32#89]
    0x0096, 0x0089,
    // --> [Op1regspill32#89] and stop
    // --> [Op1regspill32#89] and stop
    // --> [Op1regspill32#89] and stop
    // --> [Op1regspill32#89] and stop
    // 000251: regspill.i32 (I32)
    // --> [Op1regspill32#89] and stop
    // 000251: regspill.r32 (I32)
    // --> [Op1regspill32#89] and stop
    // 000251: regspill.b1 (I32)
    // --> [Op1regspill32#89] and stop
    // 000251: regspill.i8 (I32)
    // --> [Op1regspill32#89] and stop
    // 000251: regspill.i16 (I32)
    // --> [Op1regspill32#89] and stop
    0x0095, 0x0089,
    // end of regspill.i16 (I32)
    // end of regspill.i8 (I32)
    // end of regspill.b1 (I32)
    // end of regspill.r32 (I32)
    // end of regspill.i32 (I32)
    // end of regspill.i16 (I64)
    // end of regspill.i8 (I64)
    // end of regspill.b1 (I64)
    // end of regspill.i32 (I64)
    // 000253: rotl.i32 (I64)
    // --> [RexOp1rc#d3]
    0x0182, 0x00d3,
    // --> [Op1rc#d3] and stop
    0x0181, 0x00d3,
    // end of rotl.i32 (I64)
    // 000257: rotl_imm.i32 (I64)
    // --> [DynRexOp1r_ib#c1] and stop
    // 000257: rotl_imm.i32 (I32)
    // --> [DynRexOp1r_ib#c1] and stop
    0x015d, 0x00c1,
    // end of rotl_imm.i32 (I32)
    // end of rotl_imm.i32 (I64)
    // 000259: rotr.i32 (I64)
    // --> [RexOp1rc#10d3]
    0x0182, 0x10d3,
    // --> [Op1rc#10d3] and stop
    0x0181, 0x10d3,
    // end of rotr.i32 (I64)
    // 00025d: rotr_imm.i32 (I64)
    // --> [DynRexOp1r_ib#10c1] and stop
    // 00025d: rotr_imm.i32 (I32)
    // --> [DynRexOp1r_ib#10c1] and stop
    0x015d, 0x10c1,
    // end of rotr_imm.i32 (I32)
    // end of rotr_imm.i32 (I64)
    // 00025f: selectif.i32 (I64)
    // --> [DynRexOp2cmov#440] and stop
    // 00025f: selectif_spectre_guard.i32 (I64)
    // --> [DynRexOp2cmov#440] and stop
    // 00025f: selectif.i32 (I32)
    // --> [DynRexOp2cmov#440] and stop
    // 00025f: selectif_spectre_guard.i32 (I32)
    // --> [DynRexOp2cmov#440] and stop
    0x01b1, 0x0440,
    // end of selectif_spectre_guard.i32 (I32)
    // end of selectif.i32 (I32)
    // end of selectif_spectre_guard.i32 (I64)
    // end of selectif.i32 (I64)
    // 000261: sextend.i32 (I64)
    // skip 4 unless inst_predicate_5
    0x5005,
    // --> [RexOp2urm_noflags#4be]
    0x0022, 0x04be,
    // --> [Op2urm_noflags_abcd#4be]
    0x0020, 0x04be,
    // stop unless inst_predicate_2
    0x1002,
    // --> [RexOp2urm_noflags#4bf]
    0x0022, 0x04bf,
    // --> [Op2urm_noflags#4bf] and stop
    0x0025, 0x04bf,
    // end of sextend.i32 (I64)
    // 00026b: sload16.i32 (I64)
    // --> [RexOp2ld#4bf]
    0x009e, 0x04bf,
    // --> [Op2ld#4bf]
    0x009c, 0x04bf,
    // --> [RexOp2ldDisp8#4bf]
    0x00a6, 0x04bf,
    // --> [Op2ldDisp8#4bf]
    0x00a4, 0x04bf,
    // --> [RexOp2ldDisp32#4bf]
    0x00ae, 0x04bf,
    // --> [Op2ldDisp32#4bf] and stop
    0x00ad, 0x04bf,
    // end of sload16.i32 (I64)
    // 000277: sload16_complex.i32 (I64)
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexOp2ldWithIndex#4bf]
    0x003c, 0x04bf,
    // --> [Op2ldWithIndex#4bf]
    0x003a, 0x04bf,
    // --> [RexOp2ldWithIndexDisp8#4bf]
    0x0044, 0x04bf,
    // --> [Op2ldWithIndexDisp8#4bf]
    0x0042, 0x04bf,
    // --> [RexOp2ldWithIndexDisp32#4bf]
    0x004c, 0x04bf,
    // --> [Op2ldWithIndexDisp32#4bf] and stop
    0x004b, 0x04bf,
    // end of sload16_complex.i32 (I64)
    // 000284: sload8.i32 (I64)
    // --> [RexOp2ld#4be]
    0x009e, 0x04be,
    // --> [Op2ld#4be]
    0x009c, 0x04be,
    // --> [RexOp2ldDisp8#4be]
    0x00a6, 0x04be,
    // --> [Op2ldDisp8#4be]
    0x00a4, 0x04be,
    // --> [RexOp2ldDisp32#4be]
    0x00ae, 0x04be,
    // --> [Op2ldDisp32#4be] and stop
    0x00ad, 0x04be,
    // end of sload8.i32 (I64)
    // 000290: sload8_complex.i32 (I64)
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexOp2ldWithIndex#4be]
    0x003c, 0x04be,
    // --> [Op2ldWithIndex#4be]
    0x003a, 0x04be,
    // --> [RexOp2ldWithIndexDisp8#4be]
    0x0044, 0x04be,
    // --> [Op2ldWithIndexDisp8#4be]
    0x0042, 0x04be,
    // --> [RexOp2ldWithIndexDisp32#4be]
    0x004c, 0x04be,
    // --> [Op2ldWithIndexDisp32#4be] and stop
    0x004b, 0x04be,
    // end of sload8_complex.i32 (I64)
    // 00029d: spill.i32 (I64)
    // --> [RexOp1spillSib32#89]
    // 00029d: spill.b1 (I64)
    // --> [RexOp1spillSib32#89]
    // 00029d: spill.i8 (I64)
    // --> [RexOp1spillSib32#89]
    // 00029d: spill.i16 (I64)
    // --> [RexOp1spillSib32#89]
    0x0092, 0x0089,
    // --> [Op1spillSib32#89] and stop
    // --> [Op1spillSib32#89] and stop
    // --> [Op1spillSib32#89] and stop
    // --> [Op1spillSib32#89] and stop
    // 00029f: spill.i32 (I32)
    // --> [Op1spillSib32#89] and stop
    // 00029f: spill.r32 (I32)
    // --> [Op1spillSib32#89] and stop
    // 00029f: spill.b1 (I32)
    // --> [Op1spillSib32#89] and stop
    // 00029f: spill.i8 (I32)
    // --> [Op1spillSib32#89] and stop
    // 00029f: spill.i16 (I32)
    // --> [Op1spillSib32#89] and stop
    0x0091, 0x0089,
    // end of spill.i16 (I32)
    // end of spill.i8 (I32)
    // end of spill.b1 (I32)
    // end of spill.r32 (I32)
    // end of spill.i32 (I32)
    // end of spill.i16 (I64)
    // end of spill.i8 (I64)
    // end of spill.b1 (I64)
    // end of spill.i32 (I64)
    // 0002a1: sshr.i32 (I64)
    // --> [RexOp1rc#70d3]
    0x0182, 0x70d3,
    // --> [Op1rc#70d3] and stop
    0x0181, 0x70d3,
    // end of sshr.i32 (I64)
    // 0002a5: sshr_imm.i32 (I64)
    // --> [DynRexOp1r_ib#70c1] and stop
    // 0002a5: sshr_imm.i32 (I32)
    // --> [DynRexOp1r_ib#70c1] and stop
    0x015d, 0x70c1,
    // end of sshr_imm.i32 (I32)
    // end of sshr_imm.i32 (I64)
    // 0002a7: store_complex.i32 (I64)
    // stop unless inst_predicate_12
    // 0002a7: store_complex.r32 (I64)
    // stop unless inst_predicate_12
    // 0002a7: istore32_complex (I64)
    // stop unless inst_predicate_12
    0x100c,
    // --> [RexOp1stWithIndex#89]
    // --> [RexOp1stWithIndex#89]
    // --> [RexOp1stWithIndex#89]
    0x0050, 0x0089,
    // --> [Op1stWithIndex#89]
    // --> [Op1stWithIndex#89]
    // --> [Op1stWithIndex#89]
    0x004e, 0x0089,
    // --> [RexOp1stWithIndexDisp8#89]
    // --> [RexOp1stWithIndexDisp8#89]
    // --> [RexOp1stWithIndexDisp8#89]
    0x0058, 0x0089,
    // --> [Op1stWithIndexDisp8#89]
    // --> [Op1stWithIndexDisp8#89]
    // --> [Op1stWithIndexDisp8#89]
    0x0056, 0x0089,
    // --> [RexOp1stWithIndexDisp32#89]
    // --> [RexOp1stWithIndexDisp32#89]
    // --> [RexOp1stWithIndexDisp32#89]
    0x0060, 0x0089,
    // --> [Op1stWithIndexDisp32#89] and stop
    // --> [Op1stWithIndexDisp32#89] and stop
    // --> [Op1stWithIndexDisp32#89] and stop
    0x005f, 0x0089,
    // end of istore32_complex (I64)
    // end of store_complex.r32 (I64)
    // end of store_complex.i32 (I64)
    // 0002b4: uextend.i32 (I64)
    // skip 4 unless inst_predicate_5
    0x5005,
    // --> [RexOp2urm_noflags#4b6]
    0x0022, 0x04b6,
    // --> [Op2urm_noflags_abcd#4b6]
    0x0020, 0x04b6,
    // stop unless inst_predicate_2
    0x1002,
    // --> [RexOp2urm_noflags#4b7]
    0x0022, 0x04b7,
    // --> [Op2urm_noflags#4b7] and stop
    0x0025, 0x04b7,
    // end of uextend.i32 (I64)
    // 0002be: uload16.i32 (I64)
    // --> [RexOp2ld#4b7]
    0x009e, 0x04b7,
    // --> [Op2ld#4b7]
    0x009c, 0x04b7,
    // --> [RexOp2ldDisp8#4b7]
    0x00a6, 0x04b7,
    // --> [Op2ldDisp8#4b7]
    0x00a4, 0x04b7,
    // --> [RexOp2ldDisp32#4b7]
    0x00ae, 0x04b7,
    // --> [Op2ldDisp32#4b7] and stop
    0x00ad, 0x04b7,
    // end of uload16.i32 (I64)
    // 0002ca: uload16_complex.i32 (I64)
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexOp2ldWithIndex#4b7]
    0x003c, 0x04b7,
    // --> [Op2ldWithIndex#4b7]
    0x003a, 0x04b7,
    // --> [RexOp2ldWithIndexDisp8#4b7]
    0x0044, 0x04b7,
    // --> [Op2ldWithIndexDisp8#4b7]
    0x0042, 0x04b7,
    // --> [RexOp2ldWithIndexDisp32#4b7]
    0x004c, 0x04b7,
    // --> [Op2ldWithIndexDisp32#4b7] and stop
    0x004b, 0x04b7,
    // end of uload16_complex.i32 (I64)
    // 0002d7: uload8.i32 (I64)
    // --> [RexOp2ld#4b6]
    0x009e, 0x04b6,
    // --> [Op2ld#4b6]
    0x009c, 0x04b6,
    // --> [RexOp2ldDisp8#4b6]
    0x00a6, 0x04b6,
    // --> [Op2ldDisp8#4b6]
    0x00a4, 0x04b6,
    // --> [RexOp2ldDisp32#4b6]
    0x00ae, 0x04b6,
    // --> [Op2ldDisp32#4b6] and stop
    0x00ad, 0x04b6,
    // end of uload8.i32 (I64)
    // 0002e3: uload8_complex.i32 (I64)
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexOp2ldWithIndex#4b6]
    0x003c, 0x04b6,
    // --> [Op2ldWithIndex#4b6]
    0x003a, 0x04b6,
    // --> [RexOp2ldWithIndexDisp8#4b6]
    0x0044, 0x04b6,
    // --> [Op2ldWithIndexDisp8#4b6]
    0x0042, 0x04b6,
    // --> [RexOp2ldWithIndexDisp32#4b6]
    0x004c, 0x04b6,
    // --> [Op2ldWithIndexDisp32#4b6] and stop
    0x004b, 0x04b6,
    // end of uload8_complex.i32 (I64)
    // 0002f0: ushr.i32 (I64)
    // --> [RexOp1rc#50d3]
    0x0182, 0x50d3,
    // --> [Op1rc#50d3] and stop
    0x0181, 0x50d3,
    // end of ushr.i32 (I64)
    // 0002f4: ushr_imm.i32 (I64)
    // --> [DynRexOp1r_ib#50c1] and stop
    // 0002f4: ushr_imm.i32 (I32)
    // --> [DynRexOp1r_ib#50c1] and stop
    0x015d, 0x50c1,
    // end of ushr_imm.i32 (I32)
    // end of ushr_imm.i32 (I64)
    // 0002f6: x86_bsf.i32 (I64)
    // --> [DynRexOp2bsf_and_bsr#4bc] and stop
    // 0002f6: x86_bsf.i32 (I32)
    // --> [DynRexOp2bsf_and_bsr#4bc] and stop
    0x0189, 0x04bc,
    // end of x86_bsf.i32 (I32)
    // end of x86_bsf.i32 (I64)
    // 0002f8: x86_bsr.i32 (I64)
    // --> [DynRexOp2bsf_and_bsr#4bd] and stop
    // 0002f8: x86_bsr.i32 (I32)
    // --> [DynRexOp2bsf_and_bsr#4bd] and stop
    0x0189, 0x04bd,
    // end of x86_bsr.i32 (I32)
    // end of x86_bsr.i32 (I64)
    // 0002fa: x86_cvtt2si.i32 (I64)
    // skip 4 unless inst_predicate_13
    0x500d,
    // --> [RexMp2rfurm#62c]
    0x0132, 0x062c,
    // --> [Mp2rfurm#62c]
    0x0130, 0x062c,
    // stop unless inst_predicate_14
    0x100e,
    // --> [RexMp2rfurm#72c]
    0x0132, 0x072c,
    // --> [Mp2rfurm#72c] and stop
    0x0131, 0x072c,
    // end of x86_cvtt2si.i32 (I64)
    // 000304: x86_sdivmodx.i32 (I64)
    // --> [DynRexOp1div#70f7] and stop
    // 000304: x86_sdivmodx.i32 (I32)
    // --> [DynRexOp1div#70f7] and stop
    0x0171, 0x70f7,
    // end of x86_sdivmodx.i32 (I32)
    // end of x86_sdivmodx.i32 (I64)
    // 000306: x86_smulx.i32 (I64)
    // --> [DynRexOp1mulx#50f7] and stop
    // 000306: x86_smulx.i32 (I32)
    // --> [DynRexOp1mulx#50f7] and stop
    0x0175, 0x50f7,
    // end of x86_smulx.i32 (I32)
    // end of x86_smulx.i32 (I64)
    // 000308: x86_udivmodx.i32 (I64)
    // --> [DynRexOp1div#60f7] and stop
    // 000308: x86_udivmodx.i32 (I32)
    // --> [DynRexOp1div#60f7] and stop
    0x0171, 0x60f7,
    // end of x86_udivmodx.i32 (I32)
    // end of x86_udivmodx.i32 (I64)
    // 00030a: x86_umulx.i32 (I64)
    // --> [DynRexOp1mulx#40f7] and stop
    // 00030a: x86_umulx.i32 (I32)
    // --> [DynRexOp1mulx#40f7] and stop
    0x0175, 0x40f7,
    // end of x86_umulx.i32 (I32)
    // end of x86_umulx.i32 (I64)
    // 00030c: is_invalid.r64 (I64)
    // --> [RexOp1is_invalid#f083] and stop
    0x02cf, 0xf083,
    // end of is_invalid.r64 (I64)
    // 00030e: is_null.r64 (I64)
    // --> [RexOp1is_zero#8085] and stop
    0x02cb, 0x8085,
    // end of is_null.r64 (I64)
    // 000310: null.r64 (I64)
    // --> [RexOp1pu_id_ref#b8]
    0x02c6, 0x00b8,
    // --> [Op1pu_id_ref#b8] and stop
    // 000312: null.r32 (I32)
    // --> [Op1pu_id_ref#b8] and stop
    0x02c5, 0x00b8,
    // end of null.r32 (I32)
    // end of null.r64 (I64)
    // 000314: band.b1 (I64)
    // --> [RexOp1rr#21]
    0x014e, 0x0021,
    // --> [Op1rr#21] and stop
    // 000316: band.b1 (I32)
    // --> [Op1rr#21] and stop
    0x016b, 0x0021,
    // end of band.b1 (I32)
    // end of band.b1 (I64)
    // 000318: bconst.b1 (I64)
    // --> [RexOp1pu_id_bool#b8]
    // 000318: bconst.b8 (I64)
    // --> [RexOp1pu_id_bool#b8]
    // 000318: bconst.b16 (I64)
    // --> [RexOp1pu_id_bool#b8]
    // 000318: bconst.b32 (I64)
    // --> [RexOp1pu_id_bool#b8]
    0x0018, 0x00b8,
    // --> [Op1pu_id_bool#b8] and stop
    // --> [Op1pu_id_bool#b8] and stop
    // --> [Op1pu_id_bool#b8] and stop
    // --> [Op1pu_id_bool#b8] and stop
    // 00031a: bconst.b1 (I32)
    // --> [Op1pu_id_bool#b8] and stop
    // 00031a: bconst.b8 (I32)
    // --> [Op1pu_id_bool#b8] and stop
    // 00031a: bconst.b16 (I32)
    // --> [Op1pu_id_bool#b8] and stop
    // 00031a: bconst.b32 (I32)
    // --> [Op1pu_id_bool#b8] and stop
    0x0017, 0x00b8,
    // end of bconst.b32 (I32)
    // end of bconst.b16 (I32)
    // end of bconst.b8 (I32)
    // end of bconst.b1 (I32)
    // end of bconst.b32 (I64)
    // end of bconst.b16 (I64)
    // end of bconst.b8 (I64)
    // end of bconst.b1 (I64)
    // 00031c: bnot.b1 (I64)
    // --> [RexOp1ur#20f7]
    0x0166, 0x20f7,
    // --> [Op1ur#20f7] and stop
    // 00031e: bnot.b1 (I32)
    // --> [Op1ur#20f7] and stop
    0x0169, 0x20f7,
    // end of bnot.b1 (I32)
    // end of bnot.b1 (I64)
    // 000320: bor.b1 (I64)
    // --> [RexOp1rr#09]
    0x014e, 0x0009,
    // --> [Op1rr#09] and stop
    // 000322: bor.b1 (I32)
    // --> [Op1rr#09] and stop
    0x016b, 0x0009,
    // end of bor.b1 (I32)
    // end of bor.b1 (I64)
    // 000324: brnz.b1 (I64)
    // --> [RexOp1t8jccb#75]
    0x02aa, 0x0075,
    // --> [Op1t8jccb_abcd#75]
    0x02a8, 0x0075,
    // --> [RexOp1t8jccd#85]
    0x02ae, 0x0085,
    // --> [Op1t8jccd_abcd#85] and stop
    0x02ad, 0x0085,
    // end of brnz.b1 (I64)
    // 00032c: brz.b1 (I64)
    // --> [RexOp1t8jccb#74]
    0x02aa, 0x0074,
    // --> [Op1t8jccb_abcd#74]
    0x02a8, 0x0074,
    // --> [RexOp1t8jccd#84]
    0x02ae, 0x0084,
    // --> [Op1t8jccd_abcd#84] and stop
    0x02ad, 0x0084,
    // end of brz.b1 (I64)
    // 000334: bxor.b1 (I64)
    // --> [RexOp1rr#31]
    0x014e, 0x0031,
    // --> [Op1rr#31] and stop
    // 000336: bxor.b1 (I32)
    // --> [Op1rr#31] and stop
    0x016b, 0x0031,
    // end of bxor.b1 (I32)
    // end of bxor.b1 (I64)
    // 000338: regmove.b1 (I64)
    // --> [RexOp1rmov#89]
    0x000c, 0x0089,
    // --> [Op1rmov#89] and stop
    // 00033a: regmove.i32 (I32)
    // --> [Op1rmov#89] and stop
    // 00033a: regmove.r32 (I32)
    // --> [Op1rmov#89] and stop
    // 00033a: regmove.b1 (I32)
    // --> [Op1rmov#89] and stop
    // 00033a: regmove.i16 (I32)
    // --> [Op1rmov#89] and stop
    // 00033a: regmove.b8 (I32)
    // --> [Op1rmov#89] and stop
    // 00033a: regmove.b16 (I32)
    // --> [Op1rmov#89] and stop
    // 00033a: regmove.b32 (I32)
    // --> [Op1rmov#89] and stop
    0x000b, 0x0089,
    // end of regmove.b32 (I32)
    // end of regmove.b16 (I32)
    // end of regmove.b8 (I32)
    // end of regmove.i16 (I32)
    // end of regmove.b1 (I32)
    // end of regmove.r32 (I32)
    // end of regmove.i32 (I32)
    // end of regmove.b1 (I64)
    // 00033c: bint.i8 (I64)
    // skip 4 unless inst_predicate_6
    0x5006,
    // --> [RexOp2urm_noflags#4b6]
    0x0022, 0x04b6,
    // --> [Op2urm_noflags_abcd#4b6]
    0x0020, 0x04b6,
    // stop unless inst_predicate_7
    0x1007,
    // --> [RexOp2urm_noflags#4b6]
    0x0022, 0x04b6,
    // --> [Op2urm_noflags_abcd#4b6] and stop
    0x0021, 0x04b6,
    // end of bint.i8 (I64)
    // 000346: iconst.i8 (I64)
    // stop unless inst_predicate_1
    0x1001,
    // --> [RexOp1u_id_z#30]
    0x001c, 0x0030,
    // --> [Op1u_id_z#30] and stop
    0x001b, 0x0030,
    // end of iconst.i8 (I64)
    // 00034b: ireduce.i8 (I64)
    // skip 2 unless inst_predicate_2
    0x3002,
    // --> [null#00]
    0x001e, 0x0000,
    // skip 2 unless inst_predicate_3
    // 00034e: ireduce.i16 (I64)
    // skip 2 unless inst_predicate_3
    0x3003,
    // --> [null#00]
    // --> [null#00]
    0x001e, 0x0000,
    // stop unless inst_predicate_4
    // stop unless inst_predicate_4
    0x1004,
    // --> [null#00] and stop
    // --> [null#00] and stop
    0x001f, 0x0000,
    // end of ireduce.i16 (I64)
    // end of ireduce.i8 (I64)
    // 000354: regmove.i8 (I64)
    // --> [RexOp1rmov#89]
    0x000c, 0x0089,
    // --> [RexOp1rmov#89]
    0x000c, 0x0089,
    // --> [Op1rmov#89] and stop
    0x000b, 0x0089,
    // end of regmove.i8 (I64)
    // 00035a: bint.i16 (I64)
    // skip 4 unless inst_predicate_6
    0x5006,
    // --> [RexOp2urm_noflags#4b6]
    0x0022, 0x04b6,
    // --> [Op2urm_noflags_abcd#4b6]
    0x0020, 0x04b6,
    // skip 4 unless inst_predicate_7
    0x5007,
    // --> [RexOp2urm_noflags#4b6]
    0x0022, 0x04b6,
    // --> [Op2urm_noflags_abcd#4b6]
    0x0020, 0x04b6,
    // stop unless inst_predicate_8
    0x1008,
    // --> [RexOp2urm_noflags#4b6]
    0x0022, 0x04b6,
    // --> [Op2urm_noflags_abcd#4b6] and stop
    0x0021, 0x04b6,
    // end of bint.i16 (I64)
    // 000369: bconst.b64 (I64)
    // --> [RexOp1pu_id_bool#b8] and stop
    0x0019, 0x00b8,
    // end of bconst.b64 (I64)
    // 00036b: adjust_sp_down_imm (I64)
    // --> [RexOp1adjustsp_ib#d083]
    0x00ce, 0xd083,
    // --> [RexOp1adjustsp_id#d081] and stop
    0x00d1, 0xd081,
    // end of adjust_sp_down_imm (I64)
    // 00036f: adjust_sp_up_imm (I64)
    // --> [RexOp1adjustsp_ib#8083]
    0x00ce, 0x8083,
    // --> [RexOp1adjustsp_id#8081] and stop
    0x00d1, 0x8081,
    // end of adjust_sp_up_imm (I64)
    // 000373: brff (I64)
    // --> [RexOp1brfb#70]
    0x0298, 0x0070,
    // --> [Op1brfb#70]
    0x0296, 0x0070,
    // --> [RexOp2brfd#480]
    0x029c, 0x0480,
    // --> [Op2brfd#480] and stop
    0x029b, 0x0480,
    // end of brff (I64)
    // 00037b: brif (I64)
    // --> [RexOp1brib#70]
    0x0290, 0x0070,
    // --> [Op1brib#70]
    0x028e, 0x0070,
    // --> [RexOp2brid#480]
    0x0294, 0x0480,
    // --> [Op2brid#480] and stop
    0x0293, 0x0480,
    // end of brif (I64)
    // 000383: call (I64)
    // skip 2 unless inst_predicate_36
    0x3024,
    // --> [Op1call_id#e8]
    0x0280, 0x00e8,
    // stop unless PredicateView(14)
    0x1033,
    // --> [Op1call_plt_id#e8] and stop
    0x0283, 0x00e8,
    // end of call (I64)
    // 000389: copy_special (I64)
    // --> [RexOp1copysp#8089] and stop
    0x0029, 0x8089,
    // end of copy_special (I64)
    // 00038b: debugtrap (I64)
    // --> [debugtrap#00] and stop
    // 00038b: debugtrap (I32)
    // --> [debugtrap#00] and stop
    0x02bf, 0x0000,
    // end of debugtrap (I32)
    // end of debugtrap (I64)
    // 00038d: dummy_sarg_t (I64)
    // --> [dummy_sarg_t#00] and stop
    // 00038d: dummy_sarg_t (I32)
    // --> [dummy_sarg_t#00] and stop
    0x0035, 0x0000,
    // end of dummy_sarg_t (I32)
    // end of dummy_sarg_t (I64)
    // 00038f: f32const (I64)
    // stop unless inst_predicate_15
    0x100f,
    // --> [RexOp2f32imm_z#457]
    0x0126, 0x0457,
    // --> [Op2f32imm_z#457] and stop
    0x0123, 0x0457,
    // end of f32const (I64)
    // 000394: f64const (I64)
    // stop unless inst_predicate_16
    0x1010,
    // --> [RexMp2f64imm_z#557]
    0x0128, 0x0557,
    // --> [Mp2f64imm_z#557] and stop
    0x0125, 0x0557,
    // end of f64const (I64)
    // 000399: jump (I64)
    // --> [Op1jmpb#eb]
    // 000399: jump (I32)
    // --> [Op1jmpb#eb]
    0x028a, 0x00eb,
    // --> [Op1jmpd#e9] and stop
    // --> [Op1jmpd#e9] and stop
    0x028d, 0x00e9,
    // end of jump (I32)
    // end of jump (I64)
    // 00039d: resumable_trap (I64)
    // --> [Op2trap#40b] and stop
    // 00039d: trap (I64)
    // --> [Op2trap#40b] and stop
    // 00039d: resumable_trap (I32)
    // --> [Op2trap#40b] and stop
    // 00039d: trap (I32)
    // --> [Op2trap#40b] and stop
    0x02bd, 0x040b,
    // end of trap (I32)
    // end of resumable_trap (I32)
    // end of trap (I64)
    // end of resumable_trap (I64)
    // 00039f: return (I64)
    // --> [Op1ret#c3] and stop
    // 00039f: return (I32)
    // --> [Op1ret#c3] and stop
    0x0289, 0x00c3,
    // end of return (I32)
    // end of return (I64)
    // 0003a1: safepoint (I64)
    // --> [safepoint#00] and stop
    // 0003a1: safepoint (I32)
    // --> [safepoint#00] and stop
    0x02d1, 0x0000,
    // end of safepoint (I32)
    // end of safepoint (I64)
    // 0003a3: sload16x4_complex (I64)
    // stop unless PredicateView(26)
    0x103f,
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexMp3fldWithIndex#923]
    0x023a, 0x0923,
    // --> [Mp3fldWithIndex#923]
    0x0238, 0x0923,
    // --> [RexMp3fldWithIndexDisp8#923]
    0x023e, 0x0923,
    // --> [Mp3fldWithIndexDisp8#923]
    0x023c, 0x0923,
    // --> [RexMp3fldWithIndexDisp32#923]
    0x0242, 0x0923,
    // --> [Mp3fldWithIndexDisp32#923] and stop
    0x0241, 0x0923,
    // end of sload16x4_complex (I64)
    // 0003b1: sload32_complex (I64)
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexOp1ldWithIndex#8063]
    0x0038, 0x8063,
    // --> [RexOp1ldWithIndexDisp8#8063]
    0x0040, 0x8063,
    // --> [RexOp1ldWithIndexDisp32#8063] and stop
    0x0049, 0x8063,
    // end of sload32_complex (I64)
    // 0003b8: sload32x2_complex (I64)
    // stop unless PredicateView(26)
    0x103f,
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexMp3fldWithIndex#925]
    0x023a, 0x0925,
    // --> [Mp3fldWithIndex#925]
    0x0238, 0x0925,
    // --> [RexMp3fldWithIndexDisp8#925]
    0x023e, 0x0925,
    // --> [Mp3fldWithIndexDisp8#925]
    0x023c, 0x0925,
    // --> [RexMp3fldWithIndexDisp32#925]
    0x0242, 0x0925,
    // --> [Mp3fldWithIndexDisp32#925] and stop
    0x0241, 0x0925,
    // end of sload32x2_complex (I64)
    // 0003c6: sload8x8_complex (I64)
    // stop unless PredicateView(26)
    0x103f,
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexMp3fldWithIndex#920]
    0x023a, 0x0920,
    // --> [Mp3fldWithIndex#920]
    0x0238, 0x0920,
    // --> [RexMp3fldWithIndexDisp8#920]
    0x023e, 0x0920,
    // --> [Mp3fldWithIndexDisp8#920]
    0x023c, 0x0920,
    // --> [RexMp3fldWithIndexDisp32#920]
    0x0242, 0x0920,
    // --> [Mp3fldWithIndexDisp32#920] and stop
    0x0241, 0x0920,
    // end of sload8x8_complex (I64)
    // 0003d4: trapff (I64)
    // --> [trapff#00] and stop
    // 0003d4: trapff (I32)
    // --> [trapff#00] and stop
    0x02c3, 0x0000,
    // end of trapff (I32)
    // end of trapff (I64)
    // 0003d6: trapif (I64)
    // --> [trapif#00] and stop
    // 0003d6: trapif (I32)
    // --> [trapif#00] and stop
    0x02c1, 0x0000,
    // end of trapif (I32)
    // end of trapif (I64)
    // 0003d8: trueff (I64)
    // --> [RexOp2setf#490]
    0x01ae, 0x0490,
    // --> [Op2setf_abcd#490] and stop
    // 0003da: trueff (I32)
    // --> [Op2setf_abcd#490] and stop
    0x01ad, 0x0490,
    // end of trueff (I32)
    // end of trueff (I64)
    // 0003dc: trueif (I64)
    // --> [RexOp2seti#490]
    0x01aa, 0x0490,
    // --> [Op2seti_abcd#490] and stop
    // 0003de: trueif (I32)
    // --> [Op2seti_abcd#490] and stop
    0x01a9, 0x0490,
    // end of trueif (I32)
    // end of trueif (I64)
    // 0003e0: uload16x4_complex (I64)
    // stop unless PredicateView(26)
    0x103f,
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexMp3fldWithIndex#933]
    0x023a, 0x0933,
    // --> [Mp3fldWithIndex#933]
    0x0238, 0x0933,
    // --> [RexMp3fldWithIndexDisp8#933]
    0x023e, 0x0933,
    // --> [Mp3fldWithIndexDisp8#933]
    0x023c, 0x0933,
    // --> [RexMp3fldWithIndexDisp32#933]
    0x0242, 0x0933,
    // --> [Mp3fldWithIndexDisp32#933] and stop
    0x0241, 0x0933,
    // end of uload16x4_complex (I64)
    // 0003ee: uload32x2_complex (I64)
    // stop unless PredicateView(26)
    0x103f,
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexMp3fldWithIndex#935]
    0x023a, 0x0935,
    // --> [Mp3fldWithIndex#935]
    0x0238, 0x0935,
    // --> [RexMp3fldWithIndexDisp8#935]
    0x023e, 0x0935,
    // --> [Mp3fldWithIndexDisp8#935]
    0x023c, 0x0935,
    // --> [RexMp3fldWithIndexDisp32#935]
    0x0242, 0x0935,
    // --> [Mp3fldWithIndexDisp32#935] and stop
    0x0241, 0x0935,
    // end of uload32x2_complex (I64)
    // 0003fc: uload8x8_complex (I64)
    // stop unless PredicateView(26)
    0x103f,
    // stop unless inst_predicate_11
    0x100b,
    // --> [RexMp3fldWithIndex#930]
    0x023a, 0x0930,
    // --> [Mp3fldWithIndex#930]
    0x0238, 0x0930,
    // --> [RexMp3fldWithIndexDisp8#930]
    0x023e, 0x0930,
    // --> [Mp3fldWithIndexDisp8#930]
    0x023c, 0x0930,
    // --> [RexMp3fldWithIndexDisp32#930]
    0x0242, 0x0930,
    // --> [Mp3fldWithIndexDisp32#930] and stop
    0x0241, 0x0930,
    // end of uload8x8_complex (I64)
    // 00040a: x86_elf_tls_get_addr (I64)
    // --> [elf_tls_get_addr#00] and stop
    0x02d3, 0x0000,
    // end of x86_elf_tls_get_addr (I64)
    // 00040c: x86_macho_tls_get_addr (I64)
    // --> [macho_tls_get_addr#00] and stop
    0x02d5, 0x0000,
    // end of x86_macho_tls_get_addr (I64)
    // 00040e: x86_pmullq (I64)
    // stop unless PredicateView(18)
    // 00040e: x86_pmullq (I32)
    // stop unless PredicateView(18)
    0x1037,
    // --> [EvexMp3evex_reg_vvvv_rm_128#8940] and stop
    // --> [EvexMp3evex_reg_vvvv_rm_128#8940] and stop
    0x0245, 0x8940,
    // end of x86_pmullq (I32)
    // end of x86_pmullq (I64)
    // 000411: x86_pmuludq (I64)
    // --> [DynRexMp2fa#5f4] and stop
    0x01d1, 0x05f4,
    // end of x86_pmuludq (I64)
    // 000413: x86_vcvtudq2ps (I64)
    // stop unless PredicateView(20)
    // 000413: x86_vcvtudq2ps (I32)
    // stop unless PredicateView(20)
    0x1039,
    // --> [EvexMp2evex_reg_rm_128#77a] and stop
    // --> [EvexMp2evex_reg_rm_128#77a] and stop
    0x01df, 0x077a,
    // end of x86_vcvtudq2ps (I32)
    // end of x86_vcvtudq2ps (I64)
    // 000416: band.f64 (I64)
    // --> [RexOp2fa#454]
    // 000416: band.f32 (I64)
    // --> [RexOp2fa#454]
    0x017a, 0x0454,
    // --> [Op2fa#454] and stop
    // --> [Op2fa#454] and stop
    // 000418: band.f64 (I32)
    // --> [Op2fa#454] and stop
    // 000418: band.f32 (I32)
    // --> [Op2fa#454] and stop
    0x0179, 0x0454,
    // end of band.f32 (I32)
    // end of band.f64 (I32)
    // end of band.f32 (I64)
    // end of band.f64 (I64)
    // 00041a: band_not.f64 (I64)
    // --> [RexOp2fax#455]
    // 00041a: band_not.f32 (I64)
    // --> [RexOp2fax#455]
    0x017e, 0x0455,
    // --> [Op2fax#455] and stop
    // --> [Op2fax#455] and stop
    // 00041c: band_not.f64 (I32)
    // --> [Op2fax#455] and stop
    // 00041c: band_not.f32 (I32)
    // --> [Op2fax#455] and stop
    0x017d, 0x0455,
    // end of band_not.f32 (I32)
    // end of band_not.f64 (I32)
    // end of band_not.f32 (I64)
    // end of band_not.f64 (I64)
    // 00041e: bitcast.f64 (I64)
    // stop unless inst_predicate_4
    0x1004,
    // --> [RexMp2frurm#856e] and stop
    // 00041f: scalar_to_vector.b64x2 (I64)
    // --> [RexMp2frurm#856e] and stop
    // 00041f: scalar_to_vector.i64x2 (I64)
    // --> [RexMp2frurm#856e] and stop
    0x00d5, 0x856e,
    // end of scalar_to_vector.i64x2 (I64)
    // end of scalar_to_vector.b64x2 (I64)
    // end of bitcast.f64 (I64)
    // 000421: bor.f64 (I64)
    // --> [RexOp2fa#456]
    // 000421: bor.f32 (I64)
    // --> [RexOp2fa#456]
    0x017a, 0x0456,
    // --> [Op2fa#456] and stop
    // --> [Op2fa#456] and stop
    // 000423: bor.f64 (I32)
    // --> [Op2fa#456] and stop
    // 000423: bor.f32 (I32)
    // --> [Op2fa#456] and stop
    0x0179, 0x0456,
    // end of bor.f32 (I32)
    // end of bor.f64 (I32)
    // end of bor.f32 (I64)
    // end of bor.f64 (I64)
    // 000425: bxor.f64 (I64)
    // --> [RexOp2fa#457]
    // 000425: bxor.f32 (I64)
    // --> [RexOp2fa#457]
    0x017a, 0x0457,
    // --> [Op2fa#457] and stop
    // --> [Op2fa#457] and stop
    // 000427: bxor.f64 (I32)
    // --> [Op2fa#457] and stop
    // 000427: bxor.f32 (I32)
    // --> [Op2fa#457] and stop
    0x0179, 0x0457,
    // end of bxor.f32 (I32)
    // end of bxor.f64 (I32)
    // end of bxor.f32 (I64)
    // end of bxor.f64 (I64)
    // 000429: ceil.f64 (I64)
    // stop unless PredicateView(25)
    // 000429: floor.f64 (I64)
    // stop unless PredicateView(25)
    // 000429: nearest.f64 (I64)
    // stop unless PredicateView(25)
    // 000429: trunc.f64 (I64)
    // stop unless PredicateView(25)
    0x103e,
    // --> [RexMp3furmi_rnd#d0b]
    // --> [RexMp3furmi_rnd#d0b]
    // --> [RexMp3furmi_rnd#d0b]
    // --> [RexMp3furmi_rnd#d0b]
    0x0136, 0x0d0b,
    // --> [Mp3furmi_rnd#d0b] and stop
    // --> [Mp3furmi_rnd#d0b] and stop
    // --> [Mp3furmi_rnd#d0b] and stop
    // --> [Mp3furmi_rnd#d0b] and stop
    0x0135, 0x0d0b,
    // end of trunc.f64 (I64)
    // end of nearest.f64 (I64)
    // end of floor.f64 (I64)
    // end of ceil.f64 (I64)
    // 00042e: copy.f64 (I64)
    // --> [RexOp2furm#428]
    // 00042e: copy.f32 (I64)
    // --> [RexOp2furm#428]
    // 00042e: copy.b8x16 (I64)
    // --> [RexOp2furm#428]
    // 00042e: copy.b16x8 (I64)
    // --> [RexOp2furm#428]
    // 00042e: copy.b32x4 (I64)
    // --> [RexOp2furm#428]
    // 00042e: copy.b64x2 (I64)
    // --> [RexOp2furm#428]
    // 00042e: copy.i8x16 (I64)
    // --> [RexOp2furm#428]
    // 00042e: copy.i16x8 (I64)
    // --> [RexOp2furm#428]
    // 00042e: copy.i32x4 (I64)
    // --> [RexOp2furm#428]
    // 00042e: copy.i64x2 (I64)
    // --> [RexOp2furm#428]
    // 00042e: copy.f32x4 (I64)
    // --> [RexOp2furm#428]
    // 00042e: copy.f64x2 (I64)
    // --> [RexOp2furm#428]
    0x00dc, 0x0428,
    // --> [Op2furm#428] and stop
    // --> [Op2furm#428] and stop
    // --> [Op2furm#428] and stop
    // --> [Op2furm#428] and stop
    // --> [Op2furm#428] and stop
    // --> [Op2furm#428] and stop
    // --> [Op2furm#428] and stop
    // --> [Op2furm#428] and stop
    // --> [Op2furm#428] and stop
    // --> [Op2furm#428] and stop
    // --> [Op2furm#428] and stop
    // --> [Op2furm#428] and stop
    // 000430: copy.f64 (I32)
    // --> [Op2furm#428] and stop
    // 000430: copy.f32 (I32)
    // --> [Op2furm#428] and stop
    // 000430: copy.b8x16 (I32)
    // --> [Op2furm#428] and stop
    // 000430: copy.b16x8 (I32)
    // --> [Op2furm#428] and stop
    // 000430: copy.b32x4 (I32)
    // --> [Op2furm#428] and stop
    // 000430: copy.b64x2 (I32)
    // --> [Op2furm#428] and stop
    // 000430: copy.i8x16 (I32)
    // --> [Op2furm#428] and stop
    // 000430: copy.i16x8 (I32)
    // --> [Op2furm#428] and stop
    // 000430: copy.i32x4 (I32)
    // --> [Op2furm#428] and stop
    // 000430: copy.i64x2 (I32)
    // --> [Op2furm#428] and stop
    // 000430: copy.f32x4 (I32)
    // --> [Op2furm#428] and stop
    // 000430: copy.f64x2 (I32)
    // --> [Op2furm#428] and stop
    0x00db, 0x0428,
    // end of copy.f64x2 (I32)
    // end of copy.f32x4 (I32)
    // end of copy.i64x2 (I32)
    // end of copy.i32x4 (I32)
    // end of copy.i16x8 (I32)
    // end of copy.i8x16 (I32)
    // end of copy.b64x2 (I32)
    // end of copy.b32x4 (I32)
    // end of copy.b16x8 (I32)
    // end of copy.b8x16 (I32)
    // end of copy.f32 (I32)
    // end of copy.f64 (I32)
    // end of copy.f64x2 (I64)
    // end of copy.f32x4 (I64)
    // end of copy.i64x2 (I64)
    // end of copy.i32x4 (I64)
    // end of copy.i16x8 (I64)
    // end of copy.i8x16 (I64)
    // end of copy.b64x2 (I64)
    // end of copy.b32x4 (I64)
    // end of copy.b16x8 (I64)
    // end of copy.b8x16 (I64)
    // end of copy.f32 (I64)
    // end of copy.f64 (I64)
    // 000432: copy_to_ssa.f64 (I64)
    // --> [RexMp2furm_reg_to_ssa#710] and stop
    0x0033, 0x0710,
    // end of copy_to_ssa.f64 (I64)
    // 000434: fadd.f64 (I64)
    // --> [RexMp2fa#758]
    0x013a, 0x0758,
    // --> [Mp2fa#758] and stop
    // 000436: fadd.f64 (I32)
    // --> [Mp2fa#758] and stop
    0x0139, 0x0758,
    // end of fadd.f64 (I32)
    // end of fadd.f64 (I64)
    // 000438: fcmp.f64 (I64)
    // --> [RexMp2fcscc#52e]
    0x0142, 0x052e,
    // --> [Mp2fcscc#52e] and stop
    // 00043a: fcmp.f64 (I32)
    // --> [Mp2fcscc#52e] and stop
    0x0141, 0x052e,
    // end of fcmp.f64 (I32)
    // end of fcmp.f64 (I64)
    // 00043c: fcvt_from_sint.f64 (I64)
    // skip 2 unless inst_predicate_3
    0x3003,
    // --> [DynRexMp2frurm#72a]
    0x012a, 0x072a,
    // stop unless inst_predicate_4
    0x1004,
    // --> [RexMp2frurm#872a] and stop
    0x00d5, 0x872a,
    // end of fcvt_from_sint.f64 (I64)
    // 000442: fdiv.f64 (I64)
    // --> [RexMp2fa#75e]
    0x013a, 0x075e,
    // --> [Mp2fa#75e] and stop
    // 000444: fdiv.f64 (I32)
    // --> [Mp2fa#75e] and stop
    0x0139, 0x075e,
    // end of fdiv.f64 (I32)
    // end of fdiv.f64 (I64)
    // 000446: ffcmp.f64 (I64)
    // --> [RexMp2fcmp#52e]
    0x014a, 0x052e,
    // --> [Mp2fcmp#52e] and stop
    // 000448: ffcmp.f64 (I32)
    // --> [Mp2fcmp#52e] and stop
    0x0149, 0x052e,
    // end of ffcmp.f64 (I32)
    // end of ffcmp.f64 (I64)
    // 00044a: fill.f64 (I64)
    // --> [RexMp2ffillSib32#710]
    0x0114, 0x0710,
    // --> [Mp2ffillSib32#710] and stop
    // 00044c: fill.f64 (I32)
    // --> [Mp2ffillSib32#710] and stop
    0x0113, 0x0710,
    // end of fill.f64 (I32)
    // end of fill.f64 (I64)
    // 00044e: fill_nop.f64 (I64)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.f32 (I64)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.b8x16 (I64)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.b16x8 (I64)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.b32x4 (I64)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.b64x2 (I64)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.i8x16 (I64)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.i16x8 (I64)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.i32x4 (I64)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.i64x2 (I64)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.f32x4 (I64)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.f64x2 (I64)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.f64 (I32)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.f32 (I32)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.b8x16 (I32)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.b16x8 (I32)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.b32x4 (I32)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.b64x2 (I32)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.i8x16 (I32)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.i16x8 (I32)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.i32x4 (I32)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.i64x2 (I32)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.f32x4 (I32)
    // --> [ffillnull#00] and stop
    // 00044e: fill_nop.f64x2 (I32)
    // --> [ffillnull#00] and stop
    0x00bb, 0x0000,
    // end of fill_nop.f64x2 (I32)
    // end of fill_nop.f32x4 (I32)
    // end of fill_nop.i64x2 (I32)
    // end of fill_nop.i32x4 (I32)
    // end of fill_nop.i16x8 (I32)
    // end of fill_nop.i8x16 (I32)
    // end of fill_nop.b64x2 (I32)
    // end of fill_nop.b32x4 (I32)
    // end of fill_nop.b16x8 (I32)
    // end of fill_nop.b8x16 (I32)
    // end of fill_nop.f32 (I32)
    // end of fill_nop.f64 (I32)
    // end of fill_nop.f64x2 (I64)
    // end of fill_nop.f32x4 (I64)
    // end of fill_nop.i64x2 (I64)
    // end of fill_nop.i32x4 (I64)
    // end of fill_nop.i16x8 (I64)
    // end of fill_nop.i8x16 (I64)
    // end of fill_nop.b64x2 (I64)
    // end of fill_nop.b32x4 (I64)
    // end of fill_nop.b16x8 (I64)
    // end of fill_nop.b8x16 (I64)
    // end of fill_nop.f32 (I64)
    // end of fill_nop.f64 (I64)
    // 000450: fmul.f64 (I64)
    // --> [RexMp2fa#759]
    0x013a, 0x0759,
    // --> [Mp2fa#759] and stop
    // 000452: fmul.f64 (I32)
    // --> [Mp2fa#759] and stop
    0x0139, 0x0759,
    // end of fmul.f64 (I32)
    // end of fmul.f64 (I64)
    // 000454: fpromote.f64 (I64)
    // stop unless inst_predicate_13
    0x100d,
    // --> [RexMp2furm#65a]
    0x012e, 0x065a,
    // --> [Mp2furm#65a] and stop
    0x012d, 0x065a,
    // end of fpromote.f64 (I64)
    // 000459: fsub.f64 (I64)
    // --> [RexMp2fa#75c]
    0x013a, 0x075c,
    // --> [Mp2fa#75c] and stop
    // 00045b: fsub.f64 (I32)
    // --> [Mp2fa#75c] and stop
    0x0139, 0x075c,
    // end of fsub.f64 (I32)
    // end of fsub.f64 (I64)
    // 00045d: load.f64 (I64)
    // --> [RexMp2fld#710]
    0x00e4, 0x0710,
    // --> [Mp2fld#710]
    0x00e2, 0x0710,
    // --> [RexMp2fldDisp8#710]
    0x00e8, 0x0710,
    // --> [Mp2fldDisp8#710]
    0x00e6, 0x0710,
    // --> [RexMp2fldDisp32#710]
    0x00ec, 0x0710,
    // --> [Mp2fldDisp32#710] and stop
    0x00eb, 0x0710,
    // end of load.f64 (I64)
    // 000469: load_complex.f64 (I64)
    // --> [RexMp2fldWithIndex#710]
    0x00f0, 0x0710,
    // --> [Mp2fldWithIndex#710]
    0x00ee, 0x0710,
    // --> [RexMp2fldWithIndexDisp8#710]
    0x00f4, 0x0710,
    // --> [Mp2fldWithIndexDisp8#710]
    0x00f2, 0x0710,
    // --> [RexMp2fldWithIndexDisp32#710]
    0x00f8, 0x0710,
    // --> [Mp2fldWithIndexDisp32#710] and stop
    0x00f7, 0x0710,
    // end of load_complex.f64 (I64)
    // 000475: raw_bitcast.f64 (I64)
    // skip 2 unless inst_predicate_20
    // 000475: raw_bitcast.f32 (I64)
    // skip 2 unless inst_predicate_20
    // 000475: raw_bitcast.f64 (I32)
    // skip 2 unless inst_predicate_20
    // 000475: raw_bitcast.f32 (I32)
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    0x3019,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_26
    // skip 2 unless inst_predicate_26
    // skip 2 unless inst_predicate_26
    // skip 2 unless inst_predicate_26
    0x301a,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    0x301b,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_28
    // skip 2 unless inst_predicate_28
    // skip 2 unless inst_predicate_28
    // skip 2 unless inst_predicate_28
    0x301c,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // stop unless inst_predicate_29
    // stop unless inst_predicate_29
    // stop unless inst_predicate_29
    // stop unless inst_predicate_29
    0x101d,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    // 000491: scalar_to_vector.f32x4 (I64)
    // --> [null_fpr#00] and stop
    // 000491: scalar_to_vector.f64x2 (I64)
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    // 000491: scalar_to_vector.f32x4 (I32)
    // --> [null_fpr#00] and stop
    // 000491: scalar_to_vector.f64x2 (I32)
    // --> [null_fpr#00] and stop
    0x01c5, 0x0000,
    // end of scalar_to_vector.f64x2 (I32)
    // end of scalar_to_vector.f32x4 (I32)
    // end of raw_bitcast.f32 (I32)
    // end of raw_bitcast.f64 (I32)
    // end of scalar_to_vector.f64x2 (I64)
    // end of scalar_to_vector.f32x4 (I64)
    // end of raw_bitcast.f32 (I64)
    // end of raw_bitcast.f64 (I64)
    // 000493: regfill.f64 (I64)
    // --> [RexMp2fregfill32#710]
    0x0118, 0x0710,
    // --> [Mp2fregfill32#710] and stop
    // 000495: regfill.f64 (I32)
    // --> [Mp2fregfill32#710] and stop
    0x0117, 0x0710,
    // end of regfill.f64 (I32)
    // end of regfill.f64 (I64)
    // 000497: regmove.f64 (I64)
    // --> [RexOp2frmov#428] and stop
    // 000497: regmove.f32 (I64)
    // --> [RexOp2frmov#428] and stop
    0x00e1, 0x0428,
    // end of regmove.f32 (I64)
    // end of regmove.f64 (I64)
    // 000499: regspill.f64 (I64)
    // --> [RexMp2fregspill32#711]
    0x0120, 0x0711,
    // --> [Mp2fregspill32#711] and stop
    // 00049b: regspill.f64 (I32)
    // --> [Mp2fregspill32#711] and stop
    0x011f, 0x0711,
    // end of regspill.f64 (I32)
    // end of regspill.f64 (I64)
    // 00049d: spill.f64 (I64)
    // --> [RexMp2fspillSib32#711]
    0x011c, 0x0711,
    // --> [Mp2fspillSib32#711] and stop
    // 00049f: spill.f64 (I32)
    // --> [Mp2fspillSib32#711] and stop
    0x011b, 0x0711,
    // end of spill.f64 (I32)
    // end of spill.f64 (I64)
    // 0004a1: sqrt.f64 (I64)
    // --> [RexMp2furm#751]
    0x012e, 0x0751,
    // --> [Mp2furm#751] and stop
    // 0004a3: sqrt.f64 (I32)
    // --> [Mp2furm#751] and stop
    0x012d, 0x0751,
    // end of sqrt.f64 (I32)
    // end of sqrt.f64 (I64)
    // 0004a5: store.f64 (I64)
    // --> [RexMp2fst#711]
    0x00fc, 0x0711,
    // --> [Mp2fst#711]
    0x00fa, 0x0711,
    // --> [RexMp2fstDisp8#711]
    0x0100, 0x0711,
    // --> [Mp2fstDisp8#711]
    0x00fe, 0x0711,
    // --> [RexMp2fstDisp32#711]
    0x0104, 0x0711,
    // --> [Mp2fstDisp32#711] and stop
    0x0103, 0x0711,
    // end of store.f64 (I64)
    // 0004b1: store_complex.f64 (I64)
    // --> [RexMp2fstWithIndex#711]
    0x0108, 0x0711,
    // --> [Mp2fstWithIndex#711]
    0x0106, 0x0711,
    // --> [RexMp2fstWithIndexDisp8#711]
    0x010c, 0x0711,
    // --> [Mp2fstWithIndexDisp8#711]
    0x010a, 0x0711,
    // --> [RexMp2fstWithIndexDisp32#711]
    0x0110, 0x0711,
    // --> [Mp2fstWithIndexDisp32#711] and stop
    0x010f, 0x0711,
    // end of store_complex.f64 (I64)
    // 0004bd: x86_fmax.f64 (I64)
    // --> [RexMp2fa#75f]
    0x013a, 0x075f,
    // --> [Mp2fa#75f] and stop
    // 0004bf: x86_fmax.f64 (I32)
    // --> [Mp2fa#75f] and stop
    0x0139, 0x075f,
    // end of x86_fmax.f64 (I32)
    // end of x86_fmax.f64 (I64)
    // 0004c1: x86_fmin.f64 (I64)
    // --> [RexMp2fa#75d]
    0x013a, 0x075d,
    // --> [Mp2fa#75d] and stop
    // 0004c3: x86_fmin.f64 (I32)
    // --> [Mp2fa#75d] and stop
    0x0139, 0x075d,
    // end of x86_fmin.f64 (I32)
    // end of x86_fmin.f64 (I64)
    // 0004c5: bitcast.f32 (I64)
    // stop unless inst_predicate_3
    0x1003,
    // --> [RexMp2frurm#56e]
    0x00d4, 0x056e,
    // --> [Mp2frurm#56e] and stop
    // 0004c8: scalar_to_vector.b8x16 (I32)
    // --> [Mp2frurm#56e] and stop
    // 0004c8: scalar_to_vector.b16x8 (I32)
    // --> [Mp2frurm#56e] and stop
    // 0004c8: scalar_to_vector.b32x4 (I32)
    // --> [Mp2frurm#56e] and stop
    // 0004c8: scalar_to_vector.i8x16 (I32)
    // --> [Mp2frurm#56e] and stop
    // 0004c8: scalar_to_vector.i16x8 (I32)
    // --> [Mp2frurm#56e] and stop
    // 0004c8: scalar_to_vector.i32x4 (I32)
    // --> [Mp2frurm#56e] and stop
    0x00d3, 0x056e,
    // end of scalar_to_vector.i32x4 (I32)
    // end of scalar_to_vector.i16x8 (I32)
    // end of scalar_to_vector.i8x16 (I32)
    // end of scalar_to_vector.b32x4 (I32)
    // end of scalar_to_vector.b16x8 (I32)
    // end of scalar_to_vector.b8x16 (I32)
    // end of bitcast.f32 (I64)
    // 0004ca: ceil.f32 (I64)
    // stop unless PredicateView(25)
    // 0004ca: floor.f32 (I64)
    // stop unless PredicateView(25)
    // 0004ca: nearest.f32 (I64)
    // stop unless PredicateView(25)
    // 0004ca: trunc.f32 (I64)
    // stop unless PredicateView(25)
    0x103e,
    // --> [RexMp3furmi_rnd#d0a]
    // --> [RexMp3furmi_rnd#d0a]
    // --> [RexMp3furmi_rnd#d0a]
    // --> [RexMp3furmi_rnd#d0a]
    0x0136, 0x0d0a,
    // --> [Mp3furmi_rnd#d0a] and stop
    // --> [Mp3furmi_rnd#d0a] and stop
    // --> [Mp3furmi_rnd#d0a] and stop
    // --> [Mp3furmi_rnd#d0a] and stop
    0x0135, 0x0d0a,
    // end of trunc.f32 (I64)
    // end of nearest.f32 (I64)
    // end of floor.f32 (I64)
    // end of ceil.f32 (I64)
    // 0004cf: copy_to_ssa.f32 (I64)
    // --> [RexMp2furm_reg_to_ssa#610] and stop
    0x0033, 0x0610,
    // end of copy_to_ssa.f32 (I64)
    // 0004d1: fadd.f32 (I64)
    // --> [RexMp2fa#658]
    0x013a, 0x0658,
    // --> [Mp2fa#658] and stop
    // 0004d3: fadd.f32 (I32)
    // --> [Mp2fa#658] and stop
    0x0139, 0x0658,
    // end of fadd.f32 (I32)
    // end of fadd.f32 (I64)
    // 0004d5: fcmp.f32 (I64)
    // --> [RexOp2fcscc#42e]
    0x013e, 0x042e,
    // --> [Op2fcscc#42e] and stop
    // 0004d7: fcmp.f32 (I32)
    // --> [Op2fcscc#42e] and stop
    0x013d, 0x042e,
    // end of fcmp.f32 (I32)
    // end of fcmp.f32 (I64)
    // 0004d9: fcvt_from_sint.f32 (I64)
    // skip 2 unless inst_predicate_3
    0x3003,
    // --> [DynRexMp2frurm#62a]
    0x012a, 0x062a,
    // stop unless inst_predicate_4
    0x1004,
    // --> [RexMp2frurm#862a] and stop
    0x00d5, 0x862a,
    // end of fcvt_from_sint.f32 (I64)
    // 0004df: fdemote.f32 (I64)
    // stop unless inst_predicate_14
    0x100e,
    // --> [RexMp2furm#75a]
    0x012e, 0x075a,
    // --> [Mp2furm#75a] and stop
    0x012d, 0x075a,
    // end of fdemote.f32 (I64)
    // 0004e4: fdiv.f32 (I64)
    // --> [RexMp2fa#65e]
    0x013a, 0x065e,
    // --> [Mp2fa#65e] and stop
    // 0004e6: fdiv.f32 (I32)
    // --> [Mp2fa#65e] and stop
    0x0139, 0x065e,
    // end of fdiv.f32 (I32)
    // end of fdiv.f32 (I64)
    // 0004e8: ffcmp.f32 (I64)
    // --> [RexOp2fcmp#42e]
    0x0146, 0x042e,
    // --> [Op2fcmp#42e] and stop
    // 0004ea: ffcmp.f32 (I32)
    // --> [Op2fcmp#42e] and stop
    0x0145, 0x042e,
    // end of ffcmp.f32 (I32)
    // end of ffcmp.f32 (I64)
    // 0004ec: fill.f32 (I64)
    // --> [RexMp2ffillSib32#610]
    0x0114, 0x0610,
    // --> [Mp2ffillSib32#610] and stop
    // 0004ee: fill.f32 (I32)
    // --> [Mp2ffillSib32#610] and stop
    0x0113, 0x0610,
    // end of fill.f32 (I32)
    // end of fill.f32 (I64)
    // 0004f0: fmul.f32 (I64)
    // --> [RexMp2fa#659]
    0x013a, 0x0659,
    // --> [Mp2fa#659] and stop
    // 0004f2: fmul.f32 (I32)
    // --> [Mp2fa#659] and stop
    0x0139, 0x0659,
    // end of fmul.f32 (I32)
    // end of fmul.f32 (I64)
    // 0004f4: fsub.f32 (I64)
    // --> [RexMp2fa#65c]
    0x013a, 0x065c,
    // --> [Mp2fa#65c] and stop
    // 0004f6: fsub.f32 (I32)
    // --> [Mp2fa#65c] and stop
    0x0139, 0x065c,
    // end of fsub.f32 (I32)
    // end of fsub.f32 (I64)
    // 0004f8: load.f32 (I64)
    // --> [RexMp2fld#610]
    0x00e4, 0x0610,
    // --> [Mp2fld#610]
    0x00e2, 0x0610,
    // --> [RexMp2fldDisp8#610]
    0x00e8, 0x0610,
    // --> [Mp2fldDisp8#610]
    0x00e6, 0x0610,
    // --> [RexMp2fldDisp32#610]
    0x00ec, 0x0610,
    // --> [Mp2fldDisp32#610] and stop
    0x00eb, 0x0610,
    // end of load.f32 (I64)
    // 000504: load_complex.f32 (I64)
    // --> [RexMp2fldWithIndex#610]
    0x00f0, 0x0610,
    // --> [Mp2fldWithIndex#610]
    0x00ee, 0x0610,
    // --> [RexMp2fldWithIndexDisp8#610]
    0x00f4, 0x0610,
    // --> [Mp2fldWithIndexDisp8#610]
    0x00f2, 0x0610,
    // --> [RexMp2fldWithIndexDisp32#610]
    0x00f8, 0x0610,
    // --> [Mp2fldWithIndexDisp32#610] and stop
    0x00f7, 0x0610,
    // end of load_complex.f32 (I64)
    // 000510: regfill.f32 (I64)
    // --> [RexMp2fregfill32#610]
    0x0118, 0x0610,
    // --> [Mp2fregfill32#610] and stop
    // 000512: regfill.f32 (I32)
    // --> [Mp2fregfill32#610] and stop
    0x0117, 0x0610,
    // end of regfill.f32 (I32)
    // end of regfill.f32 (I64)
    // 000514: regspill.f32 (I64)
    // --> [RexMp2fregspill32#611]
    0x0120, 0x0611,
    // --> [Mp2fregspill32#611] and stop
    // 000516: regspill.f32 (I32)
    // --> [Mp2fregspill32#611] and stop
    0x011f, 0x0611,
    // end of regspill.f32 (I32)
    // end of regspill.f32 (I64)
    // 000518: spill.f32 (I64)
    // --> [RexMp2fspillSib32#611]
    0x011c, 0x0611,
    // --> [Mp2fspillSib32#611] and stop
    // 00051a: spill.f32 (I32)
    // --> [Mp2fspillSib32#611] and stop
    0x011b, 0x0611,
    // end of spill.f32 (I32)
    // end of spill.f32 (I64)
    // 00051c: sqrt.f32 (I64)
    // --> [RexMp2furm#651]
    0x012e, 0x0651,
    // --> [Mp2furm#651] and stop
    // 00051e: sqrt.f32 (I32)
    // --> [Mp2furm#651] and stop
    0x012d, 0x0651,
    // end of sqrt.f32 (I32)
    // end of sqrt.f32 (I64)
    // 000520: store.f32 (I64)
    // --> [RexMp2fst#611]
    0x00fc, 0x0611,
    // --> [Mp2fst#611]
    0x00fa, 0x0611,
    // --> [RexMp2fstDisp8#611]
    0x0100, 0x0611,
    // --> [Mp2fstDisp8#611]
    0x00fe, 0x0611,
    // --> [RexMp2fstDisp32#611]
    0x0104, 0x0611,
    // --> [Mp2fstDisp32#611] and stop
    0x0103, 0x0611,
    // end of store.f32 (I64)
    // 00052c: store_complex.f32 (I64)
    // --> [RexMp2fstWithIndex#611]
    0x0108, 0x0611,
    // --> [Mp2fstWithIndex#611]
    0x0106, 0x0611,
    // --> [RexMp2fstWithIndexDisp8#611]
    0x010c, 0x0611,
    // --> [Mp2fstWithIndexDisp8#611]
    0x010a, 0x0611,
    // --> [RexMp2fstWithIndexDisp32#611]
    0x0110, 0x0611,
    // --> [Mp2fstWithIndexDisp32#611] and stop
    0x010f, 0x0611,
    // end of store_complex.f32 (I64)
    // 000538: x86_fmax.f32 (I64)
    // --> [RexMp2fa#65f]
    0x013a, 0x065f,
    // --> [Mp2fa#65f] and stop
    // 00053a: x86_fmax.f32 (I32)
    // --> [Mp2fa#65f] and stop
    0x0139, 0x065f,
    // end of x86_fmax.f32 (I32)
    // end of x86_fmax.f32 (I64)
    // 00053c: x86_fmin.f32 (I64)
    // --> [RexMp2fa#65d]
    0x013a, 0x065d,
    // --> [Mp2fa#65d] and stop
    // 00053e: x86_fmin.f32 (I32)
    // --> [Mp2fa#65d] and stop
    0x0139, 0x065d,
    // end of x86_fmin.f32 (I32)
    // end of x86_fmin.f32 (I64)
    // 000540: band.b8x16 (I64)
    // --> [DynRexMp2fa#5db] and stop
    // 000540: band.b16x8 (I64)
    // --> [DynRexMp2fa#5db] and stop
    // 000540: band.b32x4 (I64)
    // --> [DynRexMp2fa#5db] and stop
    // 000540: band.b64x2 (I64)
    // --> [DynRexMp2fa#5db] and stop
    // 000540: band.i8x16 (I64)
    // --> [DynRexMp2fa#5db] and stop
    // 000540: band.i16x8 (I64)
    // --> [DynRexMp2fa#5db] and stop
    // 000540: band.i32x4 (I64)
    // --> [DynRexMp2fa#5db] and stop
    // 000540: band.i64x2 (I64)
    // --> [DynRexMp2fa#5db] and stop
    // 000540: band.f32x4 (I64)
    // --> [DynRexMp2fa#5db] and stop
    // 000540: band.f64x2 (I64)
    // --> [DynRexMp2fa#5db] and stop
    0x01d1, 0x05db,
    // end of band.f64x2 (I64)
    // end of band.f32x4 (I64)
    // end of band.i64x2 (I64)
    // end of band.i32x4 (I64)
    // end of band.i16x8 (I64)
    // end of band.i8x16 (I64)
    // end of band.b64x2 (I64)
    // end of band.b32x4 (I64)
    // end of band.b16x8 (I64)
    // end of band.b8x16 (I64)
    // 000542: band_not.b8x16 (I64)
    // --> [DynRexMp2fax#5df] and stop
    // 000542: band_not.b16x8 (I64)
    // --> [DynRexMp2fax#5df] and stop
    // 000542: band_not.b32x4 (I64)
    // --> [DynRexMp2fax#5df] and stop
    // 000542: band_not.b64x2 (I64)
    // --> [DynRexMp2fax#5df] and stop
    // 000542: band_not.i8x16 (I64)
    // --> [DynRexMp2fax#5df] and stop
    // 000542: band_not.i16x8 (I64)
    // --> [DynRexMp2fax#5df] and stop
    // 000542: band_not.i32x4 (I64)
    // --> [DynRexMp2fax#5df] and stop
    // 000542: band_not.i64x2 (I64)
    // --> [DynRexMp2fax#5df] and stop
    // 000542: band_not.f32x4 (I64)
    // --> [DynRexMp2fax#5df] and stop
    // 000542: band_not.f64x2 (I64)
    // --> [DynRexMp2fax#5df] and stop
    0x0249, 0x05df,
    // end of band_not.f64x2 (I64)
    // end of band_not.f32x4 (I64)
    // end of band_not.i64x2 (I64)
    // end of band_not.i32x4 (I64)
    // end of band_not.i16x8 (I64)
    // end of band_not.i8x16 (I64)
    // end of band_not.b64x2 (I64)
    // end of band_not.b32x4 (I64)
    // end of band_not.b16x8 (I64)
    // end of band_not.b8x16 (I64)
    // 000544: bor.b8x16 (I64)
    // --> [DynRexMp2fa#5eb] and stop
    // 000544: bor.b16x8 (I64)
    // --> [DynRexMp2fa#5eb] and stop
    // 000544: bor.b32x4 (I64)
    // --> [DynRexMp2fa#5eb] and stop
    // 000544: bor.b64x2 (I64)
    // --> [DynRexMp2fa#5eb] and stop
    // 000544: bor.i8x16 (I64)
    // --> [DynRexMp2fa#5eb] and stop
    // 000544: bor.i16x8 (I64)
    // --> [DynRexMp2fa#5eb] and stop
    // 000544: bor.i32x4 (I64)
    // --> [DynRexMp2fa#5eb] and stop
    // 000544: bor.i64x2 (I64)
    // --> [DynRexMp2fa#5eb] and stop
    // 000544: bor.f32x4 (I64)
    // --> [DynRexMp2fa#5eb] and stop
    // 000544: bor.f64x2 (I64)
    // --> [DynRexMp2fa#5eb] and stop
    0x01d1, 0x05eb,
    // end of bor.f64x2 (I64)
    // end of bor.f32x4 (I64)
    // end of bor.i64x2 (I64)
    // end of bor.i32x4 (I64)
    // end of bor.i16x8 (I64)
    // end of bor.i8x16 (I64)
    // end of bor.b64x2 (I64)
    // end of bor.b32x4 (I64)
    // end of bor.b16x8 (I64)
    // end of bor.b8x16 (I64)
    // 000546: bxor.b8x16 (I64)
    // --> [DynRexMp2fa#5ef] and stop
    // 000546: bxor.b16x8 (I64)
    // --> [DynRexMp2fa#5ef] and stop
    // 000546: bxor.b32x4 (I64)
    // --> [DynRexMp2fa#5ef] and stop
    // 000546: bxor.b64x2 (I64)
    // --> [DynRexMp2fa#5ef] and stop
    // 000546: bxor.i8x16 (I64)
    // --> [DynRexMp2fa#5ef] and stop
    // 000546: bxor.i16x8 (I64)
    // --> [DynRexMp2fa#5ef] and stop
    // 000546: bxor.i32x4 (I64)
    // --> [DynRexMp2fa#5ef] and stop
    // 000546: bxor.i64x2 (I64)
    // --> [DynRexMp2fa#5ef] and stop
    // 000546: bxor.f32x4 (I64)
    // --> [DynRexMp2fa#5ef] and stop
    // 000546: bxor.f64x2 (I64)
    // --> [DynRexMp2fa#5ef] and stop
    0x01d1, 0x05ef,
    // end of bxor.f64x2 (I64)
    // end of bxor.f32x4 (I64)
    // end of bxor.i64x2 (I64)
    // end of bxor.i32x4 (I64)
    // end of bxor.i16x8 (I64)
    // end of bxor.i8x16 (I64)
    // end of bxor.b64x2 (I64)
    // end of bxor.b32x4 (I64)
    // end of bxor.b16x8 (I64)
    // end of bxor.b8x16 (I64)
    // 000548: copy_to_ssa.b8x16 (I64)
    // --> [RexOp2furm_reg_to_ssa#428]
    // 000548: copy_to_ssa.b16x8 (I64)
    // --> [RexOp2furm_reg_to_ssa#428]
    // 000548: copy_to_ssa.b32x4 (I64)
    // --> [RexOp2furm_reg_to_ssa#428]
    // 000548: copy_to_ssa.b64x2 (I64)
    // --> [RexOp2furm_reg_to_ssa#428]
    // 000548: copy_to_ssa.i8x16 (I64)
    // --> [RexOp2furm_reg_to_ssa#428]
    // 000548: copy_to_ssa.i16x8 (I64)
    // --> [RexOp2furm_reg_to_ssa#428]
    // 000548: copy_to_ssa.i32x4 (I64)
    // --> [RexOp2furm_reg_to_ssa#428]
    // 000548: copy_to_ssa.i64x2 (I64)
    // --> [RexOp2furm_reg_to_ssa#428]
    // 000548: copy_to_ssa.f32x4 (I64)
    // --> [RexOp2furm_reg_to_ssa#428]
    // 000548: copy_to_ssa.f64x2 (I64)
    // --> [RexOp2furm_reg_to_ssa#428]
    0x022a, 0x0428,
    // --> [Op2furm_reg_to_ssa#428] and stop
    // --> [Op2furm_reg_to_ssa#428] and stop
    // --> [Op2furm_reg_to_ssa#428] and stop
    // --> [Op2furm_reg_to_ssa#428] and stop
    // --> [Op2furm_reg_to_ssa#428] and stop
    // --> [Op2furm_reg_to_ssa#428] and stop
    // --> [Op2furm_reg_to_ssa#428] and stop
    // --> [Op2furm_reg_to_ssa#428] and stop
    // --> [Op2furm_reg_to_ssa#428] and stop
    // --> [Op2furm_reg_to_ssa#428] and stop
    // 00054a: copy_to_ssa.b8x16 (I32)
    // --> [Op2furm_reg_to_ssa#428] and stop
    // 00054a: copy_to_ssa.b16x8 (I32)
    // --> [Op2furm_reg_to_ssa#428] and stop
    // 00054a: copy_to_ssa.b32x4 (I32)
    // --> [Op2furm_reg_to_ssa#428] and stop
    // 00054a: copy_to_ssa.b64x2 (I32)
    // --> [Op2furm_reg_to_ssa#428] and stop
    // 00054a: copy_to_ssa.i8x16 (I32)
    // --> [Op2furm_reg_to_ssa#428] and stop
    // 00054a: copy_to_ssa.i16x8 (I32)
    // --> [Op2furm_reg_to_ssa#428] and stop
    // 00054a: copy_to_ssa.i32x4 (I32)
    // --> [Op2furm_reg_to_ssa#428] and stop
    // 00054a: copy_to_ssa.i64x2 (I32)
    // --> [Op2furm_reg_to_ssa#428] and stop
    // 00054a: copy_to_ssa.f32x4 (I32)
    // --> [Op2furm_reg_to_ssa#428] and stop
    // 00054a: copy_to_ssa.f64x2 (I32)
    // --> [Op2furm_reg_to_ssa#428] and stop
    0x0229, 0x0428,
    // end of copy_to_ssa.f64x2 (I32)
    // end of copy_to_ssa.f32x4 (I32)
    // end of copy_to_ssa.i64x2 (I32)
    // end of copy_to_ssa.i32x4 (I32)
    // end of copy_to_ssa.i16x8 (I32)
    // end of copy_to_ssa.i8x16 (I32)
    // end of copy_to_ssa.b64x2 (I32)
    // end of copy_to_ssa.b32x4 (I32)
    // end of copy_to_ssa.b16x8 (I32)
    // end of copy_to_ssa.b8x16 (I32)
    // end of copy_to_ssa.f64x2 (I64)
    // end of copy_to_ssa.f32x4 (I64)
    // end of copy_to_ssa.i64x2 (I64)
    // end of copy_to_ssa.i32x4 (I64)
    // end of copy_to_ssa.i16x8 (I64)
    // end of copy_to_ssa.i8x16 (I64)
    // end of copy_to_ssa.b64x2 (I64)
    // end of copy_to_ssa.b32x4 (I64)
    // end of copy_to_ssa.b16x8 (I64)
    // end of copy_to_ssa.b8x16 (I64)
    // 00054c: fill.b8x16 (I64)
    // --> [RexOp2ffillSib32#410]
    // 00054c: fill.b16x8 (I64)
    // --> [RexOp2ffillSib32#410]
    // 00054c: fill.b32x4 (I64)
    // --> [RexOp2ffillSib32#410]
    // 00054c: fill.b64x2 (I64)
    // --> [RexOp2ffillSib32#410]
    // 00054c: fill.i8x16 (I64)
    // --> [RexOp2ffillSib32#410]
    // 00054c: fill.i16x8 (I64)
    // --> [RexOp2ffillSib32#410]
    // 00054c: fill.i32x4 (I64)
    // --> [RexOp2ffillSib32#410]
    // 00054c: fill.i64x2 (I64)
    // --> [RexOp2ffillSib32#410]
    // 00054c: fill.f32x4 (I64)
    // --> [RexOp2ffillSib32#410]
    // 00054c: fill.f64x2 (I64)
    // --> [RexOp2ffillSib32#410]
    0x0222, 0x0410,
    // --> [Op2ffillSib32#410] and stop
    // --> [Op2ffillSib32#410] and stop
    // --> [Op2ffillSib32#410] and stop
    // --> [Op2ffillSib32#410] and stop
    // --> [Op2ffillSib32#410] and stop
    // --> [Op2ffillSib32#410] and stop
    // --> [Op2ffillSib32#410] and stop
    // --> [Op2ffillSib32#410] and stop
    // --> [Op2ffillSib32#410] and stop
    // --> [Op2ffillSib32#410] and stop
    // 00054e: fill.b8x16 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 00054e: fill.b16x8 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 00054e: fill.b32x4 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 00054e: fill.b64x2 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 00054e: fill.i8x16 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 00054e: fill.i16x8 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 00054e: fill.i32x4 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 00054e: fill.i64x2 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 00054e: fill.f32x4 (I32)
    // --> [Op2ffillSib32#410] and stop
    // 00054e: fill.f64x2 (I32)
    // --> [Op2ffillSib32#410] and stop
    0x0221, 0x0410,
    // end of fill.f64x2 (I32)
    // end of fill.f32x4 (I32)
    // end of fill.i64x2 (I32)
    // end of fill.i32x4 (I32)
    // end of fill.i16x8 (I32)
    // end of fill.i8x16 (I32)
    // end of fill.b64x2 (I32)
    // end of fill.b32x4 (I32)
    // end of fill.b16x8 (I32)
    // end of fill.b8x16 (I32)
    // end of fill.f64x2 (I64)
    // end of fill.f32x4 (I64)
    // end of fill.i64x2 (I64)
    // end of fill.i32x4 (I64)
    // end of fill.i16x8 (I64)
    // end of fill.i8x16 (I64)
    // end of fill.b64x2 (I64)
    // end of fill.b32x4 (I64)
    // end of fill.b16x8 (I64)
    // end of fill.b8x16 (I64)
    // 000550: load.b8x16 (I64)
    // --> [DynRexOp2fld#410]
    // 000550: load.b16x8 (I64)
    // --> [DynRexOp2fld#410]
    // 000550: load.b32x4 (I64)
    // --> [DynRexOp2fld#410]
    // 000550: load.b64x2 (I64)
    // --> [DynRexOp2fld#410]
    // 000550: load.i8x16 (I64)
    // --> [DynRexOp2fld#410]
    // 000550: load.i16x8 (I64)
    // --> [DynRexOp2fld#410]
    // 000550: load.i32x4 (I64)
    // --> [DynRexOp2fld#410]
    // 000550: load.i64x2 (I64)
    // --> [DynRexOp2fld#410]
    // 000550: load.f32x4 (I64)
    // --> [DynRexOp2fld#410]
    // 000550: load.f64x2 (I64)
    // --> [DynRexOp2fld#410]
    0x0202, 0x0410,
    // --> [DynRexOp2fldDisp8#410]
    // --> [DynRexOp2fldDisp8#410]
    // --> [DynRexOp2fldDisp8#410]
    // --> [DynRexOp2fldDisp8#410]
    // --> [DynRexOp2fldDisp8#410]
    // --> [DynRexOp2fldDisp8#410]
    // --> [DynRexOp2fldDisp8#410]
    // --> [DynRexOp2fldDisp8#410]
    // --> [DynRexOp2fldDisp8#410]
    // --> [DynRexOp2fldDisp8#410]
    0x0206, 0x0410,
    // --> [DynRexOp2fldDisp32#410] and stop
    // --> [DynRexOp2fldDisp32#410] and stop
    // --> [DynRexOp2fldDisp32#410] and stop
    // --> [DynRexOp2fldDisp32#410] and stop
    // --> [DynRexOp2fldDisp32#410] and stop
    // --> [DynRexOp2fldDisp32#410] and stop
    // --> [DynRexOp2fldDisp32#410] and stop
    // --> [DynRexOp2fldDisp32#410] and stop
    // --> [DynRexOp2fldDisp32#410] and stop
    // --> [DynRexOp2fldDisp32#410] and stop
    0x020b, 0x0410,
    // end of load.f64x2 (I64)
    // end of load.f32x4 (I64)
    // end of load.i64x2 (I64)
    // end of load.i32x4 (I64)
    // end of load.i16x8 (I64)
    // end of load.i8x16 (I64)
    // end of load.b64x2 (I64)
    // end of load.b32x4 (I64)
    // end of load.b16x8 (I64)
    // end of load.b8x16 (I64)
    // 000556: load_complex.b8x16 (I64)
    // --> [RexOp2fldWithIndex#410]
    // 000556: load_complex.b16x8 (I64)
    // --> [RexOp2fldWithIndex#410]
    // 000556: load_complex.b32x4 (I64)
    // --> [RexOp2fldWithIndex#410]
    // 000556: load_complex.b64x2 (I64)
    // --> [RexOp2fldWithIndex#410]
    // 000556: load_complex.i8x16 (I64)
    // --> [RexOp2fldWithIndex#410]
    // 000556: load_complex.i16x8 (I64)
    // --> [RexOp2fldWithIndex#410]
    // 000556: load_complex.i32x4 (I64)
    // --> [RexOp2fldWithIndex#410]
    // 000556: load_complex.i64x2 (I64)
    // --> [RexOp2fldWithIndex#410]
    // 000556: load_complex.f32x4 (I64)
    // --> [RexOp2fldWithIndex#410]
    // 000556: load_complex.f64x2 (I64)
    // --> [RexOp2fldWithIndex#410]
    0x020e, 0x0410,
    // --> [Op2fldWithIndex#410]
    // --> [Op2fldWithIndex#410]
    // --> [Op2fldWithIndex#410]
    // --> [Op2fldWithIndex#410]
    // --> [Op2fldWithIndex#410]
    // --> [Op2fldWithIndex#410]
    // --> [Op2fldWithIndex#410]
    // --> [Op2fldWithIndex#410]
    // --> [Op2fldWithIndex#410]
    // --> [Op2fldWithIndex#410]
    0x020c, 0x0410,
    // --> [RexOp2fldWithIndexDisp8#410]
    // --> [RexOp2fldWithIndexDisp8#410]
    // --> [RexOp2fldWithIndexDisp8#410]
    // --> [RexOp2fldWithIndexDisp8#410]
    // --> [RexOp2fldWithIndexDisp8#410]
    // --> [RexOp2fldWithIndexDisp8#410]
    // --> [RexOp2fldWithIndexDisp8#410]
    // --> [RexOp2fldWithIndexDisp8#410]
    // --> [RexOp2fldWithIndexDisp8#410]
    // --> [RexOp2fldWithIndexDisp8#410]
    0x0212, 0x0410,
    // --> [Op2fldWithIndexDisp8#410]
    // --> [Op2fldWithIndexDisp8#410]
    // --> [Op2fldWithIndexDisp8#410]
    // --> [Op2fldWithIndexDisp8#410]
    // --> [Op2fldWithIndexDisp8#410]
    // --> [Op2fldWithIndexDisp8#410]
    // --> [Op2fldWithIndexDisp8#410]
    // --> [Op2fldWithIndexDisp8#410]
    // --> [Op2fldWithIndexDisp8#410]
    // --> [Op2fldWithIndexDisp8#410]
    0x0210, 0x0410,
    // --> [RexOp2fldWithIndexDisp32#410]
    // --> [RexOp2fldWithIndexDisp32#410]
    // --> [RexOp2fldWithIndexDisp32#410]
    // --> [RexOp2fldWithIndexDisp32#410]
    // --> [RexOp2fldWithIndexDisp32#410]
    // --> [RexOp2fldWithIndexDisp32#410]
    // --> [RexOp2fldWithIndexDisp32#410]
    // --> [RexOp2fldWithIndexDisp32#410]
    // --> [RexOp2fldWithIndexDisp32#410]
    // --> [RexOp2fldWithIndexDisp32#410]
    0x0216, 0x0410,
    // --> [Op2fldWithIndexDisp32#410] and stop
    // --> [Op2fldWithIndexDisp32#410] and stop
    // --> [Op2fldWithIndexDisp32#410] and stop
    // --> [Op2fldWithIndexDisp32#410] and stop
    // --> [Op2fldWithIndexDisp32#410] and stop
    // --> [Op2fldWithIndexDisp32#410] and stop
    // --> [Op2fldWithIndexDisp32#410] and stop
    // --> [Op2fldWithIndexDisp32#410] and stop
    // --> [Op2fldWithIndexDisp32#410] and stop
    // --> [Op2fldWithIndexDisp32#410] and stop
    0x0215, 0x0410,
    // end of load_complex.f64x2 (I64)
    // end of load_complex.f32x4 (I64)
    // end of load_complex.i64x2 (I64)
    // end of load_complex.i32x4 (I64)
    // end of load_complex.i16x8 (I64)
    // end of load_complex.i8x16 (I64)
    // end of load_complex.b64x2 (I64)
    // end of load_complex.b32x4 (I64)
    // end of load_complex.b16x8 (I64)
    // end of load_complex.b8x16 (I64)
    // 000562: raw_bitcast.b8x16 (I64)
    // skip 2 unless inst_predicate_21
    // 000562: raw_bitcast.b8x16 (I32)
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    0x3019,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_26
    // skip 2 unless inst_predicate_26
    0x301a,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    0x301b,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_28
    // skip 2 unless inst_predicate_28
    0x301c,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_29
    // skip 2 unless inst_predicate_29
    0x301d,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_13
    // skip 2 unless inst_predicate_13
    0x300d,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // stop unless inst_predicate_14
    // stop unless inst_predicate_14
    0x100e,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x01c5, 0x0000,
    // end of raw_bitcast.b8x16 (I32)
    // end of raw_bitcast.b8x16 (I64)
    // 000583: regfill.b8x16 (I64)
    // --> [RexOp2fregfill32#410]
    // 000583: regfill.b16x8 (I64)
    // --> [RexOp2fregfill32#410]
    // 000583: regfill.b32x4 (I64)
    // --> [RexOp2fregfill32#410]
    // 000583: regfill.b64x2 (I64)
    // --> [RexOp2fregfill32#410]
    // 000583: regfill.i8x16 (I64)
    // --> [RexOp2fregfill32#410]
    // 000583: regfill.i16x8 (I64)
    // --> [RexOp2fregfill32#410]
    // 000583: regfill.i32x4 (I64)
    // --> [RexOp2fregfill32#410]
    // 000583: regfill.i64x2 (I64)
    // --> [RexOp2fregfill32#410]
    // 000583: regfill.f32x4 (I64)
    // --> [RexOp2fregfill32#410]
    // 000583: regfill.f64x2 (I64)
    // --> [RexOp2fregfill32#410]
    0x0226, 0x0410,
    // --> [Op2fregfill32#410] and stop
    // --> [Op2fregfill32#410] and stop
    // --> [Op2fregfill32#410] and stop
    // --> [Op2fregfill32#410] and stop
    // --> [Op2fregfill32#410] and stop
    // --> [Op2fregfill32#410] and stop
    // --> [Op2fregfill32#410] and stop
    // --> [Op2fregfill32#410] and stop
    // --> [Op2fregfill32#410] and stop
    // --> [Op2fregfill32#410] and stop
    // 000585: regfill.b8x16 (I32)
    // --> [Op2fregfill32#410] and stop
    // 000585: regfill.b16x8 (I32)
    // --> [Op2fregfill32#410] and stop
    // 000585: regfill.b32x4 (I32)
    // --> [Op2fregfill32#410] and stop
    // 000585: regfill.b64x2 (I32)
    // --> [Op2fregfill32#410] and stop
    // 000585: regfill.i8x16 (I32)
    // --> [Op2fregfill32#410] and stop
    // 000585: regfill.i16x8 (I32)
    // --> [Op2fregfill32#410] and stop
    // 000585: regfill.i32x4 (I32)
    // --> [Op2fregfill32#410] and stop
    // 000585: regfill.i64x2 (I32)
    // --> [Op2fregfill32#410] and stop
    // 000585: regfill.f32x4 (I32)
    // --> [Op2fregfill32#410] and stop
    // 000585: regfill.f64x2 (I32)
    // --> [Op2fregfill32#410] and stop
    0x0225, 0x0410,
    // end of regfill.f64x2 (I32)
    // end of regfill.f32x4 (I32)
    // end of regfill.i64x2 (I32)
    // end of regfill.i32x4 (I32)
    // end of regfill.i16x8 (I32)
    // end of regfill.i8x16 (I32)
    // end of regfill.b64x2 (I32)
    // end of regfill.b32x4 (I32)
    // end of regfill.b16x8 (I32)
    // end of regfill.b8x16 (I32)
    // end of regfill.f64x2 (I64)
    // end of regfill.f32x4 (I64)
    // end of regfill.i64x2 (I64)
    // end of regfill.i32x4 (I64)
    // end of regfill.i16x8 (I64)
    // end of regfill.i8x16 (I64)
    // end of regfill.b64x2 (I64)
    // end of regfill.b32x4 (I64)
    // end of regfill.b16x8 (I64)
    // end of regfill.b8x16 (I64)
    // 000587: regmove.b8x16 (I64)
    // --> [RexOp2frmov#428]
    // 000587: regmove.b16x8 (I64)
    // --> [RexOp2frmov#428]
    // 000587: regmove.b32x4 (I64)
    // --> [RexOp2frmov#428]
    // 000587: regmove.b64x2 (I64)
    // --> [RexOp2frmov#428]
    // 000587: regmove.i8x16 (I64)
    // --> [RexOp2frmov#428]
    // 000587: regmove.i16x8 (I64)
    // --> [RexOp2frmov#428]
    // 000587: regmove.i32x4 (I64)
    // --> [RexOp2frmov#428]
    // 000587: regmove.i64x2 (I64)
    // --> [RexOp2frmov#428]
    // 000587: regmove.f32x4 (I64)
    // --> [RexOp2frmov#428]
    // 000587: regmove.f64x2 (I64)
    // --> [RexOp2frmov#428]
    0x00e0, 0x0428,
    // --> [Op2frmov#428] and stop
    // --> [Op2frmov#428] and stop
    // --> [Op2frmov#428] and stop
    // --> [Op2frmov#428] and stop
    // --> [Op2frmov#428] and stop
    // --> [Op2frmov#428] and stop
    // --> [Op2frmov#428] and stop
    // --> [Op2frmov#428] and stop
    // --> [Op2frmov#428] and stop
    // --> [Op2frmov#428] and stop
    // 000589: regmove.f64 (I32)
    // --> [Op2frmov#428] and stop
    // 000589: regmove.f32 (I32)
    // --> [Op2frmov#428] and stop
    // 000589: regmove.b8x16 (I32)
    // --> [Op2frmov#428] and stop
    // 000589: regmove.b16x8 (I32)
    // --> [Op2frmov#428] and stop
    // 000589: regmove.b32x4 (I32)
    // --> [Op2frmov#428] and stop
    // 000589: regmove.b64x2 (I32)
    // --> [Op2frmov#428] and stop
    // 000589: regmove.i8x16 (I32)
    // --> [Op2frmov#428] and stop
    // 000589: regmove.i16x8 (I32)
    // --> [Op2frmov#428] and stop
    // 000589: regmove.i32x4 (I32)
    // --> [Op2frmov#428] and stop
    // 000589: regmove.i64x2 (I32)
    // --> [Op2frmov#428] and stop
    // 000589: regmove.f32x4 (I32)
    // --> [Op2frmov#428] and stop
    // 000589: regmove.f64x2 (I32)
    // --> [Op2frmov#428] and stop
    0x00df, 0x0428,
    // end of regmove.f64x2 (I32)
    // end of regmove.f32x4 (I32)
    // end of regmove.i64x2 (I32)
    // end of regmove.i32x4 (I32)
    // end of regmove.i16x8 (I32)
    // end of regmove.i8x16 (I32)
    // end of regmove.b64x2 (I32)
    // end of regmove.b32x4 (I32)
    // end of regmove.b16x8 (I32)
    // end of regmove.b8x16 (I32)
    // end of regmove.f32 (I32)
    // end of regmove.f64 (I32)
    // end of regmove.f64x2 (I64)
    // end of regmove.f32x4 (I64)
    // end of regmove.i64x2 (I64)
    // end of regmove.i32x4 (I64)
    // end of regmove.i16x8 (I64)
    // end of regmove.i8x16 (I64)
    // end of regmove.b64x2 (I64)
    // end of regmove.b32x4 (I64)
    // end of regmove.b16x8 (I64)
    // end of regmove.b8x16 (I64)
    // 00058b: regspill.b8x16 (I64)
    // --> [RexOp2fregspill32#411]
    // 00058b: regspill.b16x8 (I64)
    // --> [RexOp2fregspill32#411]
    // 00058b: regspill.b32x4 (I64)
    // --> [RexOp2fregspill32#411]
    // 00058b: regspill.b64x2 (I64)
    // --> [RexOp2fregspill32#411]
    // 00058b: regspill.i8x16 (I64)
    // --> [RexOp2fregspill32#411]
    // 00058b: regspill.i16x8 (I64)
    // --> [RexOp2fregspill32#411]
    // 00058b: regspill.i32x4 (I64)
    // --> [RexOp2fregspill32#411]
    // 00058b: regspill.i64x2 (I64)
    // --> [RexOp2fregspill32#411]
    // 00058b: regspill.f32x4 (I64)
    // --> [RexOp2fregspill32#411]
    // 00058b: regspill.f64x2 (I64)
    // --> [RexOp2fregspill32#411]
    0x021e, 0x0411,
    // --> [Op2fregspill32#411] and stop
    // --> [Op2fregspill32#411] and stop
    // --> [Op2fregspill32#411] and stop
    // --> [Op2fregspill32#411] and stop
    // --> [Op2fregspill32#411] and stop
    // --> [Op2fregspill32#411] and stop
    // --> [Op2fregspill32#411] and stop
    // --> [Op2fregspill32#411] and stop
    // --> [Op2fregspill32#411] and stop
    // --> [Op2fregspill32#411] and stop
    // 00058d: regspill.b8x16 (I32)
    // --> [Op2fregspill32#411] and stop
    // 00058d: regspill.b16x8 (I32)
    // --> [Op2fregspill32#411] and stop
    // 00058d: regspill.b32x4 (I32)
    // --> [Op2fregspill32#411] and stop
    // 00058d: regspill.b64x2 (I32)
    // --> [Op2fregspill32#411] and stop
    // 00058d: regspill.i8x16 (I32)
    // --> [Op2fregspill32#411] and stop
    // 00058d: regspill.i16x8 (I32)
    // --> [Op2fregspill32#411] and stop
    // 00058d: regspill.i32x4 (I32)
    // --> [Op2fregspill32#411] and stop
    // 00058d: regspill.i64x2 (I32)
    // --> [Op2fregspill32#411] and stop
    // 00058d: regspill.f32x4 (I32)
    // --> [Op2fregspill32#411] and stop
    // 00058d: regspill.f64x2 (I32)
    // --> [Op2fregspill32#411] and stop
    0x021d, 0x0411,
    // end of regspill.f64x2 (I32)
    // end of regspill.f32x4 (I32)
    // end of regspill.i64x2 (I32)
    // end of regspill.i32x4 (I32)
    // end of regspill.i16x8 (I32)
    // end of regspill.i8x16 (I32)
    // end of regspill.b64x2 (I32)
    // end of regspill.b32x4 (I32)
    // end of regspill.b16x8 (I32)
    // end of regspill.b8x16 (I32)
    // end of regspill.f64x2 (I64)
    // end of regspill.f32x4 (I64)
    // end of regspill.i64x2 (I64)
    // end of regspill.i32x4 (I64)
    // end of regspill.i16x8 (I64)
    // end of regspill.i8x16 (I64)
    // end of regspill.b64x2 (I64)
    // end of regspill.b32x4 (I64)
    // end of regspill.b16x8 (I64)
    // end of regspill.b8x16 (I64)
    // 00058f: scalar_to_vector.b8x16 (I64)
    // --> [DynRexMp2frurm#56e] and stop
    // 00058f: scalar_to_vector.b16x8 (I64)
    // --> [DynRexMp2frurm#56e] and stop
    // 00058f: scalar_to_vector.b32x4 (I64)
    // --> [DynRexMp2frurm#56e] and stop
    // 00058f: scalar_to_vector.i8x16 (I64)
    // --> [DynRexMp2frurm#56e] and stop
    // 00058f: scalar_to_vector.i16x8 (I64)
    // --> [DynRexMp2frurm#56e] and stop
    // 00058f: scalar_to_vector.i32x4 (I64)
    // --> [DynRexMp2frurm#56e] and stop
    0x012b, 0x056e,
    // end of scalar_to_vector.i32x4 (I64)
    // end of scalar_to_vector.i16x8 (I64)
    // end of scalar_to_vector.i8x16 (I64)
    // end of scalar_to_vector.b32x4 (I64)
    // end of scalar_to_vector.b16x8 (I64)
    // end of scalar_to_vector.b8x16 (I64)
    // 000591: spill.b8x16 (I64)
    // --> [RexOp2fspillSib32#411]
    // 000591: spill.b16x8 (I64)
    // --> [RexOp2fspillSib32#411]
    // 000591: spill.b32x4 (I64)
    // --> [RexOp2fspillSib32#411]
    // 000591: spill.b64x2 (I64)
    // --> [RexOp2fspillSib32#411]
    // 000591: spill.i8x16 (I64)
    // --> [RexOp2fspillSib32#411]
    // 000591: spill.i16x8 (I64)
    // --> [RexOp2fspillSib32#411]
    // 000591: spill.i32x4 (I64)
    // --> [RexOp2fspillSib32#411]
    // 000591: spill.i64x2 (I64)
    // --> [RexOp2fspillSib32#411]
    // 000591: spill.f32x4 (I64)
    // --> [RexOp2fspillSib32#411]
    // 000591: spill.f64x2 (I64)
    // --> [RexOp2fspillSib32#411]
    0x021a, 0x0411,
    // --> [Op2fspillSib32#411] and stop
    // --> [Op2fspillSib32#411] and stop
    // --> [Op2fspillSib32#411] and stop
    // --> [Op2fspillSib32#411] and stop
    // --> [Op2fspillSib32#411] and stop
    // --> [Op2fspillSib32#411] and stop
    // --> [Op2fspillSib32#411] and stop
    // --> [Op2fspillSib32#411] and stop
    // --> [Op2fspillSib32#411] and stop
    // --> [Op2fspillSib32#411] and stop
    // 000593: spill.b8x16 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 000593: spill.b16x8 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 000593: spill.b32x4 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 000593: spill.b64x2 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 000593: spill.i8x16 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 000593: spill.i16x8 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 000593: spill.i32x4 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 000593: spill.i64x2 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 000593: spill.f32x4 (I32)
    // --> [Op2fspillSib32#411] and stop
    // 000593: spill.f64x2 (I32)
    // --> [Op2fspillSib32#411] and stop
    0x0219, 0x0411,
    // end of spill.f64x2 (I32)
    // end of spill.f32x4 (I32)
    // end of spill.i64x2 (I32)
    // end of spill.i32x4 (I32)
    // end of spill.i16x8 (I32)
    // end of spill.i8x16 (I32)
    // end of spill.b64x2 (I32)
    // end of spill.b32x4 (I32)
    // end of spill.b16x8 (I32)
    // end of spill.b8x16 (I32)
    // end of spill.f64x2 (I64)
    // end of spill.f32x4 (I64)
    // end of spill.i64x2 (I64)
    // end of spill.i32x4 (I64)
    // end of spill.i16x8 (I64)
    // end of spill.i8x16 (I64)
    // end of spill.b64x2 (I64)
    // end of spill.b32x4 (I64)
    // end of spill.b16x8 (I64)
    // end of spill.b8x16 (I64)
    // 000595: store.b8x16 (I64)
    // --> [DynRexOp2fst#411]
    // 000595: store.b16x8 (I64)
    // --> [DynRexOp2fst#411]
    // 000595: store.b32x4 (I64)
    // --> [DynRexOp2fst#411]
    // 000595: store.b64x2 (I64)
    // --> [DynRexOp2fst#411]
    // 000595: store.i8x16 (I64)
    // --> [DynRexOp2fst#411]
    // 000595: store.i16x8 (I64)
    // --> [DynRexOp2fst#411]
    // 000595: store.i32x4 (I64)
    // --> [DynRexOp2fst#411]
    // 000595: store.i64x2 (I64)
    // --> [DynRexOp2fst#411]
    // 000595: store.f32x4 (I64)
    // --> [DynRexOp2fst#411]
    // 000595: store.f64x2 (I64)
    // --> [DynRexOp2fst#411]
    0x01ea, 0x0411,
    // --> [DynRexOp2fstDisp8#411]
    // --> [DynRexOp2fstDisp8#411]
    // --> [DynRexOp2fstDisp8#411]
    // --> [DynRexOp2fstDisp8#411]
    // --> [DynRexOp2fstDisp8#411]
    // --> [DynRexOp2fstDisp8#411]
    // --> [DynRexOp2fstDisp8#411]
    // --> [DynRexOp2fstDisp8#411]
    // --> [DynRexOp2fstDisp8#411]
    // --> [DynRexOp2fstDisp8#411]
    0x01ee, 0x0411,
    // --> [DynRexOp2fstDisp32#411] and stop
    // --> [DynRexOp2fstDisp32#411] and stop
    // --> [DynRexOp2fstDisp32#411] and stop
    // --> [DynRexOp2fstDisp32#411] and stop
    // --> [DynRexOp2fstDisp32#411] and stop
    // --> [DynRexOp2fstDisp32#411] and stop
    // --> [DynRexOp2fstDisp32#411] and stop
    // --> [DynRexOp2fstDisp32#411] and stop
    // --> [DynRexOp2fstDisp32#411] and stop
    // --> [DynRexOp2fstDisp32#411] and stop
    0x01f3, 0x0411,
    // end of store.f64x2 (I64)
    // end of store.f32x4 (I64)
    // end of store.i64x2 (I64)
    // end of store.i32x4 (I64)
    // end of store.i16x8 (I64)
    // end of store.i8x16 (I64)
    // end of store.b64x2 (I64)
    // end of store.b32x4 (I64)
    // end of store.b16x8 (I64)
    // end of store.b8x16 (I64)
    // 00059b: store_complex.b8x16 (I64)
    // --> [RexOp2fstWithIndex#411]
    // 00059b: store_complex.b16x8 (I64)
    // --> [RexOp2fstWithIndex#411]
    // 00059b: store_complex.b32x4 (I64)
    // --> [RexOp2fstWithIndex#411]
    // 00059b: store_complex.b64x2 (I64)
    // --> [RexOp2fstWithIndex#411]
    // 00059b: store_complex.i8x16 (I64)
    // --> [RexOp2fstWithIndex#411]
    // 00059b: store_complex.i16x8 (I64)
    // --> [RexOp2fstWithIndex#411]
    // 00059b: store_complex.i32x4 (I64)
    // --> [RexOp2fstWithIndex#411]
    // 00059b: store_complex.i64x2 (I64)
    // --> [RexOp2fstWithIndex#411]
    // 00059b: store_complex.f32x4 (I64)
    // --> [RexOp2fstWithIndex#411]
    // 00059b: store_complex.f64x2 (I64)
    // --> [RexOp2fstWithIndex#411]
    0x01f6, 0x0411,
    // --> [Op2fstWithIndex#411]
    // --> [Op2fstWithIndex#411]
    // --> [Op2fstWithIndex#411]
    // --> [Op2fstWithIndex#411]
    // --> [Op2fstWithIndex#411]
    // --> [Op2fstWithIndex#411]
    // --> [Op2fstWithIndex#411]
    // --> [Op2fstWithIndex#411]
    // --> [Op2fstWithIndex#411]
    // --> [Op2fstWithIndex#411]
    0x01f4, 0x0411,
    // --> [RexOp2fstWithIndexDisp8#411]
    // --> [RexOp2fstWithIndexDisp8#411]
    // --> [RexOp2fstWithIndexDisp8#411]
    // --> [RexOp2fstWithIndexDisp8#411]
    // --> [RexOp2fstWithIndexDisp8#411]
    // --> [RexOp2fstWithIndexDisp8#411]
    // --> [RexOp2fstWithIndexDisp8#411]
    // --> [RexOp2fstWithIndexDisp8#411]
    // --> [RexOp2fstWithIndexDisp8#411]
    // --> [RexOp2fstWithIndexDisp8#411]
    0x01fa, 0x0411,
    // --> [Op2fstWithIndexDisp8#411]
    // --> [Op2fstWithIndexDisp8#411]
    // --> [Op2fstWithIndexDisp8#411]
    // --> [Op2fstWithIndexDisp8#411]
    // --> [Op2fstWithIndexDisp8#411]
    // --> [Op2fstWithIndexDisp8#411]
    // --> [Op2fstWithIndexDisp8#411]
    // --> [Op2fstWithIndexDisp8#411]
    // --> [Op2fstWithIndexDisp8#411]
    // --> [Op2fstWithIndexDisp8#411]
    0x01f8, 0x0411,
    // --> [RexOp2fstWithIndexDisp32#411]
    // --> [RexOp2fstWithIndexDisp32#411]
    // --> [RexOp2fstWithIndexDisp32#411]
    // --> [RexOp2fstWithIndexDisp32#411]
    // --> [RexOp2fstWithIndexDisp32#411]
    // --> [RexOp2fstWithIndexDisp32#411]
    // --> [RexOp2fstWithIndexDisp32#411]
    // --> [RexOp2fstWithIndexDisp32#411]
    // --> [RexOp2fstWithIndexDisp32#411]
    // --> [RexOp2fstWithIndexDisp32#411]
    0x01fe, 0x0411,
    // --> [Op2fstWithIndexDisp32#411] and stop
    // --> [Op2fstWithIndexDisp32#411] and stop
    // --> [Op2fstWithIndexDisp32#411] and stop
    // --> [Op2fstWithIndexDisp32#411] and stop
    // --> [Op2fstWithIndexDisp32#411] and stop
    // --> [Op2fstWithIndexDisp32#411] and stop
    // --> [Op2fstWithIndexDisp32#411] and stop
    // --> [Op2fstWithIndexDisp32#411] and stop
    // --> [Op2fstWithIndexDisp32#411] and stop
    // --> [Op2fstWithIndexDisp32#411] and stop
    0x01fd, 0x0411,
    // end of store_complex.f64x2 (I64)
    // end of store_complex.f32x4 (I64)
    // end of store_complex.i64x2 (I64)
    // end of store_complex.i32x4 (I64)
    // end of store_complex.i16x8 (I64)
    // end of store_complex.i8x16 (I64)
    // end of store_complex.b64x2 (I64)
    // end of store_complex.b32x4 (I64)
    // end of store_complex.b16x8 (I64)
    // end of store_complex.b8x16 (I64)
    // 0005a7: vconst.b8x16 (I64)
    // skip 2 unless inst_predicate_30
    // 0005a7: vconst.b16x8 (I64)
    // skip 2 unless inst_predicate_30
    // 0005a7: vconst.b32x4 (I64)
    // skip 2 unless inst_predicate_30
    // 0005a7: vconst.b64x2 (I64)
    // skip 2 unless inst_predicate_30
    // 0005a7: vconst.i8x16 (I64)
    // skip 2 unless inst_predicate_30
    // 0005a7: vconst.i16x8 (I64)
    // skip 2 unless inst_predicate_30
    // 0005a7: vconst.i32x4 (I64)
    // skip 2 unless inst_predicate_30
    // 0005a7: vconst.i64x2 (I64)
    // skip 2 unless inst_predicate_30
    // 0005a7: vconst.f32x4 (I64)
    // skip 2 unless inst_predicate_30
    // 0005a7: vconst.f64x2 (I64)
    // skip 2 unless inst_predicate_30
    0x301e,
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    0x01e2, 0x05ef,
    // skip 2 unless inst_predicate_31
    // skip 2 unless inst_predicate_31
    // skip 2 unless inst_predicate_31
    // skip 2 unless inst_predicate_31
    // skip 2 unless inst_predicate_31
    // skip 2 unless inst_predicate_31
    // skip 2 unless inst_predicate_31
    // skip 2 unless inst_predicate_31
    // skip 2 unless inst_predicate_31
    // skip 2 unless inst_predicate_31
    0x301f,
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    0x01e2, 0x0574,
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    // --> [DynRexOp2vconst#410] and stop
    0x01e7, 0x0410,
    // end of vconst.f64x2 (I64)
    // end of vconst.f32x4 (I64)
    // end of vconst.i64x2 (I64)
    // end of vconst.i32x4 (I64)
    // end of vconst.i16x8 (I64)
    // end of vconst.i8x16 (I64)
    // end of vconst.b64x2 (I64)
    // end of vconst.b32x4 (I64)
    // end of vconst.b16x8 (I64)
    // end of vconst.b8x16 (I64)
    // 0005af: vselect.b8x16 (I64)
    // stop unless PredicateView(26)
    // 0005af: vselect.b16x8 (I64)
    // stop unless PredicateView(26)
    // 0005af: vselect.i8x16 (I64)
    // stop unless PredicateView(26)
    // 0005af: vselect.i16x8 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3blend#910] and stop
    // --> [DynRexMp3blend#910] and stop
    // --> [DynRexMp3blend#910] and stop
    // --> [DynRexMp3blend#910] and stop
    0x01bf, 0x0910,
    // end of vselect.i16x8 (I64)
    // end of vselect.i8x16 (I64)
    // end of vselect.b16x8 (I64)
    // end of vselect.b8x16 (I64)
    // 0005b2: x86_pextr.b8x16 (I64)
    // stop unless PredicateView(26)
    // 0005b2: x86_pextr.i8x16 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3r_ib_unsigned_gpr#d14] and stop
    // --> [DynRexMp3r_ib_unsigned_gpr#d14] and stop
    0x01d7, 0x0d14,
    // end of x86_pextr.i8x16 (I64)
    // end of x86_pextr.b8x16 (I64)
    // 0005b5: x86_pinsr.b8x16 (I64)
    // stop unless PredicateView(26)
    // 0005b5: x86_pinsr.i8x16 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3r_ib_unsigned_r#d20] and stop
    // --> [DynRexMp3r_ib_unsigned_r#d20] and stop
    0x01c9, 0x0d20,
    // end of x86_pinsr.i8x16 (I64)
    // end of x86_pinsr.b8x16 (I64)
    // 0005b8: x86_pshufb.b8x16 (I64)
    // stop unless PredicateView(30)
    // 0005b8: x86_pshufb.b16x8 (I64)
    // stop unless PredicateView(30)
    // 0005b8: x86_pshufb.b32x4 (I64)
    // stop unless PredicateView(30)
    // 0005b8: x86_pshufb.b64x2 (I64)
    // stop unless PredicateView(30)
    // 0005b8: x86_pshufb.i8x16 (I64)
    // stop unless PredicateView(30)
    // 0005b8: x86_pshufb.i16x8 (I64)
    // stop unless PredicateView(30)
    // 0005b8: x86_pshufb.i32x4 (I64)
    // stop unless PredicateView(30)
    // 0005b8: x86_pshufb.i64x2 (I64)
    // stop unless PredicateView(30)
    // 0005b8: x86_pshufb.f32x4 (I64)
    // stop unless PredicateView(30)
    // 0005b8: x86_pshufb.f64x2 (I64)
    // stop unless PredicateView(30)
    0x1043,
    // --> [DynRexMp3fa#900] and stop
    // --> [DynRexMp3fa#900] and stop
    // --> [DynRexMp3fa#900] and stop
    // --> [DynRexMp3fa#900] and stop
    // --> [DynRexMp3fa#900] and stop
    // --> [DynRexMp3fa#900] and stop
    // --> [DynRexMp3fa#900] and stop
    // --> [DynRexMp3fa#900] and stop
    // --> [DynRexMp3fa#900] and stop
    // --> [DynRexMp3fa#900] and stop
    0x01b7, 0x0900,
    // end of x86_pshufb.f64x2 (I64)
    // end of x86_pshufb.f32x4 (I64)
    // end of x86_pshufb.i64x2 (I64)
    // end of x86_pshufb.i32x4 (I64)
    // end of x86_pshufb.i16x8 (I64)
    // end of x86_pshufb.i8x16 (I64)
    // end of x86_pshufb.b64x2 (I64)
    // end of x86_pshufb.b32x4 (I64)
    // end of x86_pshufb.b16x8 (I64)
    // end of x86_pshufb.b8x16 (I64)
    // 0005bb: x86_ptest.b8x16 (I64)
    // stop unless PredicateView(26)
    // 0005bb: x86_ptest.b16x8 (I64)
    // stop unless PredicateView(26)
    // 0005bb: x86_ptest.b32x4 (I64)
    // stop unless PredicateView(26)
    // 0005bb: x86_ptest.b64x2 (I64)
    // stop unless PredicateView(26)
    // 0005bb: x86_ptest.i8x16 (I64)
    // stop unless PredicateView(26)
    // 0005bb: x86_ptest.i16x8 (I64)
    // stop unless PredicateView(26)
    // 0005bb: x86_ptest.i32x4 (I64)
    // stop unless PredicateView(26)
    // 0005bb: x86_ptest.i64x2 (I64)
    // stop unless PredicateView(26)
    // 0005bb: x86_ptest.f32x4 (I64)
    // stop unless PredicateView(26)
    // 0005bb: x86_ptest.f64x2 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3fcmp#917] and stop
    // --> [DynRexMp3fcmp#917] and stop
    // --> [DynRexMp3fcmp#917] and stop
    // --> [DynRexMp3fcmp#917] and stop
    // --> [DynRexMp3fcmp#917] and stop
    // --> [DynRexMp3fcmp#917] and stop
    // --> [DynRexMp3fcmp#917] and stop
    // --> [DynRexMp3fcmp#917] and stop
    // --> [DynRexMp3fcmp#917] and stop
    // --> [DynRexMp3fcmp#917] and stop
    0x024d, 0x0917,
    // end of x86_ptest.f64x2 (I64)
    // end of x86_ptest.f32x4 (I64)
    // end of x86_ptest.i64x2 (I64)
    // end of x86_ptest.i32x4 (I64)
    // end of x86_ptest.i16x8 (I64)
    // end of x86_ptest.i8x16 (I64)
    // end of x86_ptest.b64x2 (I64)
    // end of x86_ptest.b32x4 (I64)
    // end of x86_ptest.b16x8 (I64)
    // end of x86_ptest.b8x16 (I64)
    // 0005be: x86_punpckh.b8x16 (I64)
    // --> [DynRexMp2fa#568] and stop
    // 0005be: x86_punpckh.i8x16 (I64)
    // --> [DynRexMp2fa#568] and stop
    0x01d1, 0x0568,
    // end of x86_punpckh.i8x16 (I64)
    // end of x86_punpckh.b8x16 (I64)
    // 0005c0: x86_punpckl.b8x16 (I64)
    // --> [DynRexMp2fa#560] and stop
    // 0005c0: x86_punpckl.i8x16 (I64)
    // --> [DynRexMp2fa#560] and stop
    0x01d1, 0x0560,
    // end of x86_punpckl.i8x16 (I64)
    // end of x86_punpckl.b8x16 (I64)
    // 0005c2: raw_bitcast.b16x8 (I64)
    // skip 2 unless inst_predicate_20
    // 0005c2: raw_bitcast.b16x8 (I32)
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    0x3019,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_26
    // skip 2 unless inst_predicate_26
    0x301a,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    0x301b,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_28
    // skip 2 unless inst_predicate_28
    0x301c,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_29
    // skip 2 unless inst_predicate_29
    0x301d,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_13
    // skip 2 unless inst_predicate_13
    0x300d,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // stop unless inst_predicate_14
    // stop unless inst_predicate_14
    0x100e,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x01c5, 0x0000,
    // end of raw_bitcast.b16x8 (I32)
    // end of raw_bitcast.b16x8 (I64)
    // 0005e3: x86_pblendw.b16x8 (I64)
    // stop unless PredicateView(26)
    // 0005e3: x86_pblendw.i16x8 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3fa_ib#d0e] and stop
    // --> [DynRexMp3fa_ib#d0e] and stop
    0x01c3, 0x0d0e,
    // end of x86_pblendw.i16x8 (I64)
    // end of x86_pblendw.b16x8 (I64)
    // 0005e6: x86_pextr.b16x8 (I64)
    // stop unless PredicateView(26)
    // 0005e6: x86_pextr.i16x8 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3r_ib_unsigned_gpr#d15] and stop
    // --> [DynRexMp3r_ib_unsigned_gpr#d15] and stop
    0x01d7, 0x0d15,
    // end of x86_pextr.i16x8 (I64)
    // end of x86_pextr.b16x8 (I64)
    // 0005e9: x86_pinsr.b16x8 (I64)
    // --> [DynRexMp2r_ib_unsigned_r#5c4] and stop
    // 0005e9: x86_pinsr.i16x8 (I64)
    // --> [DynRexMp2r_ib_unsigned_r#5c4] and stop
    0x01cd, 0x05c4,
    // end of x86_pinsr.i16x8 (I64)
    // end of x86_pinsr.b16x8 (I64)
    // 0005eb: x86_punpckh.b16x8 (I64)
    // --> [DynRexMp2fa#569] and stop
    // 0005eb: x86_punpckh.i16x8 (I64)
    // --> [DynRexMp2fa#569] and stop
    0x01d1, 0x0569,
    // end of x86_punpckh.i16x8 (I64)
    // end of x86_punpckh.b16x8 (I64)
    // 0005ed: x86_punpckl.b16x8 (I64)
    // --> [DynRexMp2fa#561] and stop
    // 0005ed: x86_punpckl.i16x8 (I64)
    // --> [DynRexMp2fa#561] and stop
    0x01d1, 0x0561,
    // end of x86_punpckl.i16x8 (I64)
    // end of x86_punpckl.b16x8 (I64)
    // 0005ef: raw_bitcast.b32x4 (I64)
    // skip 2 unless inst_predicate_20
    // 0005ef: raw_bitcast.b32x4 (I32)
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    0x3019,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_26
    // skip 2 unless inst_predicate_26
    0x301a,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    0x301b,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_28
    // skip 2 unless inst_predicate_28
    0x301c,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_29
    // skip 2 unless inst_predicate_29
    0x301d,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_13
    // skip 2 unless inst_predicate_13
    0x300d,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // stop unless inst_predicate_14
    // stop unless inst_predicate_14
    0x100e,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x01c5, 0x0000,
    // end of raw_bitcast.b32x4 (I32)
    // end of raw_bitcast.b32x4 (I64)
    // 000610: vselect.b32x4 (I64)
    // stop unless PredicateView(26)
    // 000610: vselect.i32x4 (I64)
    // stop unless PredicateView(26)
    // 000610: vselect.f32x4 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3blend#914] and stop
    // --> [DynRexMp3blend#914] and stop
    // --> [DynRexMp3blend#914] and stop
    0x01bf, 0x0914,
    // end of vselect.f32x4 (I64)
    // end of vselect.i32x4 (I64)
    // end of vselect.b32x4 (I64)
    // 000613: x86_pextr.b32x4 (I64)
    // stop unless PredicateView(26)
    // 000613: x86_pextr.i32x4 (I64)
    // stop unless PredicateView(26)
    // 000613: x86_pextr.f32x4 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3r_ib_unsigned_gpr#d16] and stop
    // --> [DynRexMp3r_ib_unsigned_gpr#d16] and stop
    // --> [DynRexMp3r_ib_unsigned_gpr#d16] and stop
    0x01d7, 0x0d16,
    // end of x86_pextr.f32x4 (I64)
    // end of x86_pextr.i32x4 (I64)
    // end of x86_pextr.b32x4 (I64)
    // 000616: x86_pinsr.b32x4 (I64)
    // stop unless PredicateView(26)
    // 000616: x86_pinsr.i32x4 (I64)
    // stop unless PredicateView(26)
    // 000616: x86_pinsr.f32x4 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3r_ib_unsigned_r#d22] and stop
    // --> [DynRexMp3r_ib_unsigned_r#d22] and stop
    // --> [DynRexMp3r_ib_unsigned_r#d22] and stop
    0x01c9, 0x0d22,
    // end of x86_pinsr.f32x4 (I64)
    // end of x86_pinsr.i32x4 (I64)
    // end of x86_pinsr.b32x4 (I64)
    // 000619: x86_pshufd.b32x4 (I64)
    // --> [DynRexMp2r_ib_unsigned_fpr#570] and stop
    // 000619: x86_pshufd.i32x4 (I64)
    // --> [DynRexMp2r_ib_unsigned_fpr#570] and stop
    // 000619: x86_pshufd.f32x4 (I64)
    // --> [DynRexMp2r_ib_unsigned_fpr#570] and stop
    0x01bb, 0x0570,
    // end of x86_pshufd.f32x4 (I64)
    // end of x86_pshufd.i32x4 (I64)
    // end of x86_pshufd.b32x4 (I64)
    // 00061b: x86_punpckh.b32x4 (I64)
    // --> [DynRexMp2fa#56a] and stop
    // 00061b: x86_punpckh.i32x4 (I64)
    // --> [DynRexMp2fa#56a] and stop
    // 00061b: x86_punpckh.f32x4 (I64)
    // --> [DynRexMp2fa#56a] and stop
    0x01d1, 0x056a,
    // end of x86_punpckh.f32x4 (I64)
    // end of x86_punpckh.i32x4 (I64)
    // end of x86_punpckh.b32x4 (I64)
    // 00061d: x86_punpckl.b32x4 (I64)
    // --> [DynRexMp2fa#562] and stop
    // 00061d: x86_punpckl.i32x4 (I64)
    // --> [DynRexMp2fa#562] and stop
    // 00061d: x86_punpckl.f32x4 (I64)
    // --> [DynRexMp2fa#562] and stop
    0x01d1, 0x0562,
    // end of x86_punpckl.f32x4 (I64)
    // end of x86_punpckl.i32x4 (I64)
    // end of x86_punpckl.b32x4 (I64)
    // 00061f: raw_bitcast.b64x2 (I64)
    // skip 2 unless inst_predicate_20
    // 00061f: raw_bitcast.b64x2 (I32)
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    0x3019,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_26
    // skip 2 unless inst_predicate_26
    0x301a,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    0x301b,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_28
    // skip 2 unless inst_predicate_28
    0x301c,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_29
    // skip 2 unless inst_predicate_29
    0x301d,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_13
    // skip 2 unless inst_predicate_13
    0x300d,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // stop unless inst_predicate_14
    // stop unless inst_predicate_14
    0x100e,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x01c5, 0x0000,
    // end of raw_bitcast.b64x2 (I32)
    // end of raw_bitcast.b64x2 (I64)
    // 000640: vselect.b64x2 (I64)
    // stop unless PredicateView(26)
    // 000640: vselect.i64x2 (I64)
    // stop unless PredicateView(26)
    // 000640: vselect.f64x2 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3blend#915] and stop
    // --> [DynRexMp3blend#915] and stop
    // --> [DynRexMp3blend#915] and stop
    0x01bf, 0x0915,
    // end of vselect.f64x2 (I64)
    // end of vselect.i64x2 (I64)
    // end of vselect.b64x2 (I64)
    // 000643: x86_pextr.b64x2 (I64)
    // stop unless PredicateView(26)
    // 000643: x86_pextr.i64x2 (I64)
    // stop unless PredicateView(26)
    // 000643: x86_pextr.f64x2 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [RexMp3r_ib_unsigned_gpr#8d16] and stop
    // --> [RexMp3r_ib_unsigned_gpr#8d16] and stop
    // --> [RexMp3r_ib_unsigned_gpr#8d16] and stop
    0x01d9, 0x8d16,
    // end of x86_pextr.f64x2 (I64)
    // end of x86_pextr.i64x2 (I64)
    // end of x86_pextr.b64x2 (I64)
    // 000646: x86_pinsr.b64x2 (I64)
    // stop unless PredicateView(26)
    // 000646: x86_pinsr.i64x2 (I64)
    // stop unless PredicateView(26)
    // 000646: x86_pinsr.f64x2 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [RexMp3r_ib_unsigned_r#8d22] and stop
    // --> [RexMp3r_ib_unsigned_r#8d22] and stop
    // --> [RexMp3r_ib_unsigned_r#8d22] and stop
    0x01cf, 0x8d22,
    // end of x86_pinsr.f64x2 (I64)
    // end of x86_pinsr.i64x2 (I64)
    // end of x86_pinsr.b64x2 (I64)
    // 000649: x86_punpckh.b64x2 (I64)
    // --> [DynRexMp2fa#56d] and stop
    // 000649: x86_punpckh.i64x2 (I64)
    // --> [DynRexMp2fa#56d] and stop
    // 000649: x86_punpckh.f64x2 (I64)
    // --> [DynRexMp2fa#56d] and stop
    0x01d1, 0x056d,
    // end of x86_punpckh.f64x2 (I64)
    // end of x86_punpckh.i64x2 (I64)
    // end of x86_punpckh.b64x2 (I64)
    // 00064b: x86_punpckl.b64x2 (I64)
    // --> [DynRexMp2fa#56c] and stop
    // 00064b: x86_punpckl.i64x2 (I64)
    // --> [DynRexMp2fa#56c] and stop
    // 00064b: x86_punpckl.f64x2 (I64)
    // --> [DynRexMp2fa#56c] and stop
    0x01d1, 0x056c,
    // end of x86_punpckl.f64x2 (I64)
    // end of x86_punpckl.i64x2 (I64)
    // end of x86_punpckl.b64x2 (I64)
    // 00064d: avg_round.i8x16 (I64)
    // --> [DynRexMp2fa#5e0] and stop
    0x01d1, 0x05e0,
    // end of avg_round.i8x16 (I64)
    // 00064f: iabs.i8x16 (I64)
    // stop unless PredicateView(30)
    0x1043,
    // --> [DynRexMp3furm#91c] and stop
    0x01dd, 0x091c,
    // end of iabs.i8x16 (I64)
    // 000652: iadd.i8x16 (I64)
    // --> [DynRexMp2fa#5fc] and stop
    0x01d1, 0x05fc,
    // end of iadd.i8x16 (I64)
    // 000654: icmp.i8x16 (I64)
    // skip 2 unless inst_predicate_32
    0x3020,
    // --> [DynRexMp2icscc_fpr#574]
    0x0254, 0x0574,
    // stop unless inst_predicate_33
    0x1021,
    // --> [DynRexMp2icscc_fpr#564] and stop
    0x0255, 0x0564,
    // end of icmp.i8x16 (I64)
    // 00065a: isub.i8x16 (I64)
    // --> [DynRexMp2fa#5f8] and stop
    0x01d1, 0x05f8,
    // end of isub.i8x16 (I64)
    // 00065c: raw_bitcast.i8x16 (I64)
    // skip 2 unless inst_predicate_20
    // 00065c: raw_bitcast.i8x16 (I32)
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    0x3019,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_26
    // skip 2 unless inst_predicate_26
    0x301a,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    0x301b,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_28
    // skip 2 unless inst_predicate_28
    0x301c,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_29
    // skip 2 unless inst_predicate_29
    0x301d,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_13
    // skip 2 unless inst_predicate_13
    0x300d,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // stop unless inst_predicate_14
    // stop unless inst_predicate_14
    0x100e,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x01c5, 0x0000,
    // end of raw_bitcast.i8x16 (I32)
    // end of raw_bitcast.i8x16 (I64)
    // 00067d: sadd_sat.i8x16 (I64)
    // --> [DynRexMp2fa#5ec] and stop
    0x01d1, 0x05ec,
    // end of sadd_sat.i8x16 (I64)
    // 00067f: ssub_sat.i8x16 (I64)
    // --> [DynRexMp2fa#5e8] and stop
    0x01d1, 0x05e8,
    // end of ssub_sat.i8x16 (I64)
    // 000681: swiden_low.i8x16 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3furm#920] and stop
    0x01dd, 0x0920,
    // end of swiden_low.i8x16 (I64)
    // 000684: uadd_sat.i8x16 (I64)
    // --> [DynRexMp2fa#5dc] and stop
    0x01d1, 0x05dc,
    // end of uadd_sat.i8x16 (I64)
    // 000686: usub_sat.i8x16 (I64)
    // --> [DynRexMp2fa#5d8] and stop
    0x01d1, 0x05d8,
    // end of usub_sat.i8x16 (I64)
    // 000688: uwiden_low.i8x16 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3furm#930] and stop
    0x01dd, 0x0930,
    // end of uwiden_low.i8x16 (I64)
    // 00068b: x86_palignr.i8x16 (I64)
    // stop unless PredicateView(30)
    // 00068b: x86_palignr.i16x8 (I64)
    // stop unless PredicateView(30)
    // 00068b: x86_palignr.i32x4 (I64)
    // stop unless PredicateView(30)
    // 00068b: x86_palignr.i64x2 (I64)
    // stop unless PredicateView(30)
    0x1043,
    // --> [DynRexMp3fa_ib#d0f] and stop
    // --> [DynRexMp3fa_ib#d0f] and stop
    // --> [DynRexMp3fa_ib#d0f] and stop
    // --> [DynRexMp3fa_ib#d0f] and stop
    0x01c3, 0x0d0f,
    // end of x86_palignr.i64x2 (I64)
    // end of x86_palignr.i32x4 (I64)
    // end of x86_palignr.i16x8 (I64)
    // end of x86_palignr.i8x16 (I64)
    // 00068e: x86_pmaxs.i8x16 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3fa#93c] and stop
    0x01b7, 0x093c,
    // end of x86_pmaxs.i8x16 (I64)
    // 000691: x86_pmaxu.i8x16 (I64)
    // --> [DynRexMp2fa#5de] and stop
    0x01d1, 0x05de,
    // end of x86_pmaxu.i8x16 (I64)
    // 000693: x86_pmins.i8x16 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3fa#938] and stop
    0x01b7, 0x0938,
    // end of x86_pmins.i8x16 (I64)
    // 000696: x86_pminu.i8x16 (I64)
    // --> [DynRexMp2fa#5da] and stop
    0x01d1, 0x05da,
    // end of x86_pminu.i8x16 (I64)
    // 000698: avg_round.i16x8 (I64)
    // --> [DynRexMp2fa#5e3] and stop
    0x01d1, 0x05e3,
    // end of avg_round.i16x8 (I64)
    // 00069a: iabs.i16x8 (I64)
    // stop unless PredicateView(30)
    0x1043,
    // --> [DynRexMp3furm#91d] and stop
    0x01dd, 0x091d,
    // end of iabs.i16x8 (I64)
    // 00069d: iadd.i16x8 (I64)
    // --> [DynRexMp2fa#5fd] and stop
    0x01d1, 0x05fd,
    // end of iadd.i16x8 (I64)
    // 00069f: icmp.i16x8 (I64)
    // skip 2 unless inst_predicate_32
    0x3020,
    // --> [DynRexMp2icscc_fpr#575]
    0x0254, 0x0575,
    // stop unless inst_predicate_33
    0x1021,
    // --> [DynRexMp2icscc_fpr#565] and stop
    0x0255, 0x0565,
    // end of icmp.i16x8 (I64)
    // 0006a5: imul.i16x8 (I64)
    // --> [DynRexMp2fa#5d5] and stop
    0x01d1, 0x05d5,
    // end of imul.i16x8 (I64)
    // 0006a7: ishl_imm.i16x8 (I64)
    // --> [DynRexMp2f_ib#6571] and stop
    0x0251, 0x6571,
    // end of ishl_imm.i16x8 (I64)
    // 0006a9: isub.i16x8 (I64)
    // --> [DynRexMp2fa#5f9] and stop
    0x01d1, 0x05f9,
    // end of isub.i16x8 (I64)
    // 0006ab: raw_bitcast.i16x8 (I64)
    // skip 2 unless inst_predicate_20
    // 0006ab: raw_bitcast.i16x8 (I32)
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_26
    // skip 2 unless inst_predicate_26
    0x301a,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    0x301b,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_28
    // skip 2 unless inst_predicate_28
    0x301c,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_29
    // skip 2 unless inst_predicate_29
    0x301d,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_13
    // skip 2 unless inst_predicate_13
    0x300d,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // stop unless inst_predicate_14
    // stop unless inst_predicate_14
    0x100e,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x01c5, 0x0000,
    // end of raw_bitcast.i16x8 (I32)
    // end of raw_bitcast.i16x8 (I64)
    // 0006cc: sadd_sat.i16x8 (I64)
    // --> [DynRexMp2fa#5ed] and stop
    0x01d1, 0x05ed,
    // end of sadd_sat.i16x8 (I64)
    // 0006ce: snarrow.i16x8 (I64)
    // --> [DynRexMp2fa#563] and stop
    0x01d1, 0x0563,
    // end of snarrow.i16x8 (I64)
    // 0006d0: sshr_imm.i16x8 (I64)
    // --> [DynRexMp2f_ib#4571] and stop
    0x0251, 0x4571,
    // end of sshr_imm.i16x8 (I64)
    // 0006d2: ssub_sat.i16x8 (I64)
    // --> [DynRexMp2fa#5e9] and stop
    0x01d1, 0x05e9,
    // end of ssub_sat.i16x8 (I64)
    // 0006d4: swiden_low.i16x8 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3furm#923] and stop
    0x01dd, 0x0923,
    // end of swiden_low.i16x8 (I64)
    // 0006d7: uadd_sat.i16x8 (I64)
    // --> [DynRexMp2fa#5dd] and stop
    0x01d1, 0x05dd,
    // end of uadd_sat.i16x8 (I64)
    // 0006d9: unarrow.i16x8 (I64)
    // --> [DynRexMp2fa#567] and stop
    0x01d1, 0x0567,
    // end of unarrow.i16x8 (I64)
    // 0006db: ushr_imm.i16x8 (I64)
    // --> [DynRexMp2f_ib#2571] and stop
    0x0251, 0x2571,
    // end of ushr_imm.i16x8 (I64)
    // 0006dd: usub_sat.i16x8 (I64)
    // --> [DynRexMp2fa#5d9] and stop
    0x01d1, 0x05d9,
    // end of usub_sat.i16x8 (I64)
    // 0006df: uwiden_low.i16x8 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3furm#933] and stop
    0x01dd, 0x0933,
    // end of uwiden_low.i16x8 (I64)
    // 0006e2: x86_pmaxs.i16x8 (I64)
    // --> [DynRexMp2fa#5ee] and stop
    0x01d1, 0x05ee,
    // end of x86_pmaxs.i16x8 (I64)
    // 0006e4: x86_pmaxu.i16x8 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3fa#93e] and stop
    0x01b7, 0x093e,
    // end of x86_pmaxu.i16x8 (I64)
    // 0006e7: x86_pmins.i16x8 (I64)
    // --> [DynRexMp2fa#5ea] and stop
    0x01d1, 0x05ea,
    // end of x86_pmins.i16x8 (I64)
    // 0006e9: x86_pminu.i16x8 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3fa#93a] and stop
    0x01b7, 0x093a,
    // end of x86_pminu.i16x8 (I64)
    // 0006ec: x86_psll.i16x8 (I64)
    // --> [DynRexMp2fa#5f1] and stop
    0x01d1, 0x05f1,
    // end of x86_psll.i16x8 (I64)
    // 0006ee: x86_psra.i16x8 (I64)
    // --> [DynRexMp2fa#5e1] and stop
    0x01d1, 0x05e1,
    // end of x86_psra.i16x8 (I64)
    // 0006f0: x86_psrl.i16x8 (I64)
    // --> [DynRexMp2fa#5d1] and stop
    0x01d1, 0x05d1,
    // end of x86_psrl.i16x8 (I64)
    // 0006f2: iabs.i32x4 (I64)
    // stop unless PredicateView(30)
    0x1043,
    // --> [DynRexMp3furm#91e] and stop
    0x01dd, 0x091e,
    // end of iabs.i32x4 (I64)
    // 0006f5: iadd.i32x4 (I64)
    // --> [DynRexMp2fa#5fe] and stop
    0x01d1, 0x05fe,
    // end of iadd.i32x4 (I64)
    // 0006f7: icmp.i32x4 (I64)
    // skip 2 unless inst_predicate_32
    0x3020,
    // --> [DynRexMp2icscc_fpr#576]
    0x0254, 0x0576,
    // stop unless inst_predicate_33
    0x1021,
    // --> [DynRexMp2icscc_fpr#566] and stop
    0x0255, 0x0566,
    // end of icmp.i32x4 (I64)
    // 0006fd: imul.i32x4 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3fa#940] and stop
    0x01b7, 0x0940,
    // end of imul.i32x4 (I64)
    // 000700: ishl_imm.i32x4 (I64)
    // --> [DynRexMp2f_ib#6572] and stop
    0x0251, 0x6572,
    // end of ishl_imm.i32x4 (I64)
    // 000702: isub.i32x4 (I64)
    // --> [DynRexMp2fa#5fa] and stop
    0x01d1, 0x05fa,
    // end of isub.i32x4 (I64)
    // 000704: raw_bitcast.i32x4 (I64)
    // skip 2 unless inst_predicate_20
    // 000704: raw_bitcast.i32x4 (I32)
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    0x3019,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    0x301b,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_28
    // skip 2 unless inst_predicate_28
    0x301c,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_29
    // skip 2 unless inst_predicate_29
    0x301d,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_13
    // skip 2 unless inst_predicate_13
    0x300d,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // stop unless inst_predicate_14
    // stop unless inst_predicate_14
    0x100e,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x01c5, 0x0000,
    // end of raw_bitcast.i32x4 (I32)
    // end of raw_bitcast.i32x4 (I64)
    // 000725: snarrow.i32x4 (I64)
    // --> [DynRexMp2fa#56b] and stop
    0x01d1, 0x056b,
    // end of snarrow.i32x4 (I64)
    // 000727: sshr_imm.i32x4 (I64)
    // --> [DynRexMp2f_ib#4572] and stop
    0x0251, 0x4572,
    // end of sshr_imm.i32x4 (I64)
    // 000729: unarrow.i32x4 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3fa#92b] and stop
    0x01b7, 0x092b,
    // end of unarrow.i32x4 (I64)
    // 00072c: ushr_imm.i32x4 (I64)
    // --> [DynRexMp2f_ib#2572] and stop
    0x0251, 0x2572,
    // end of ushr_imm.i32x4 (I64)
    // 00072e: x86_cvtt2si.i32x4 (I64)
    // stop unless inst_predicate_28
    0x101c,
    // --> [DynRexMp2furm#65b] and stop
    0x01e1, 0x065b,
    // end of x86_cvtt2si.i32x4 (I64)
    // 000731: x86_pmaxs.i32x4 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3fa#93d] and stop
    0x01b7, 0x093d,
    // end of x86_pmaxs.i32x4 (I64)
    // 000734: x86_pmaxu.i32x4 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3fa#93f] and stop
    0x01b7, 0x093f,
    // end of x86_pmaxu.i32x4 (I64)
    // 000737: x86_pmins.i32x4 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3fa#939] and stop
    0x01b7, 0x0939,
    // end of x86_pmins.i32x4 (I64)
    // 00073a: x86_pminu.i32x4 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3fa#93b] and stop
    0x01b7, 0x093b,
    // end of x86_pminu.i32x4 (I64)
    // 00073d: x86_psll.i32x4 (I64)
    // --> [DynRexMp2fa#5f2] and stop
    0x01d1, 0x05f2,
    // end of x86_psll.i32x4 (I64)
    // 00073f: x86_psra.i32x4 (I64)
    // --> [DynRexMp2fa#5e2] and stop
    0x01d1, 0x05e2,
    // end of x86_psra.i32x4 (I64)
    // 000741: x86_psrl.i32x4 (I64)
    // --> [DynRexMp2fa#5d2] and stop
    0x01d1, 0x05d2,
    // end of x86_psrl.i32x4 (I64)
    // 000743: bitcast.i64x2 (I64)
    // skip 2 unless inst_predicate_3
    0x3003,
    // --> [DynRexMp2frurm#56e]
    0x012a, 0x056e,
    // stop unless inst_predicate_4
    0x1004,
    // --> [RexMp2frurm#856e] and stop
    0x00d5, 0x856e,
    // end of bitcast.i64x2 (I64)
    // 000749: iadd.i64x2 (I64)
    // --> [DynRexMp2fa#5d4] and stop
    0x01d1, 0x05d4,
    // end of iadd.i64x2 (I64)
    // 00074b: icmp.i64x2 (I64)
    // skip 3 unless PredicateView(26)
    0x403f,
    // skip 2 unless inst_predicate_32
    0x3020,
    // --> [DynRexMp3icscc_fpr#929]
    0x0258, 0x0929,
    // stop unless PredicateView(28)
    0x1041,
    // stop unless inst_predicate_33
    0x1021,
    // --> [DynRexMp3icscc_fpr#937] and stop
    0x0259, 0x0937,
    // end of icmp.i64x2 (I64)
    // 000753: ishl_imm.i64x2 (I64)
    // --> [DynRexMp2f_ib#6573] and stop
    0x0251, 0x6573,
    // end of ishl_imm.i64x2 (I64)
    // 000755: isub.i64x2 (I64)
    // --> [DynRexMp2fa#5fb] and stop
    0x01d1, 0x05fb,
    // end of isub.i64x2 (I64)
    // 000757: raw_bitcast.i64x2 (I64)
    // skip 2 unless inst_predicate_20
    // 000757: raw_bitcast.i64x2 (I32)
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    0x3019,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_26
    // skip 2 unless inst_predicate_26
    0x301a,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_28
    // skip 2 unless inst_predicate_28
    0x301c,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_29
    // skip 2 unless inst_predicate_29
    0x301d,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_13
    // skip 2 unless inst_predicate_13
    0x300d,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // stop unless inst_predicate_14
    // stop unless inst_predicate_14
    0x100e,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x01c5, 0x0000,
    // end of raw_bitcast.i64x2 (I32)
    // end of raw_bitcast.i64x2 (I64)
    // 000778: ushr_imm.i64x2 (I64)
    // --> [DynRexMp2f_ib#2573] and stop
    0x0251, 0x2573,
    // end of ushr_imm.i64x2 (I64)
    // 00077a: x86_psll.i64x2 (I64)
    // --> [DynRexMp2fa#5f3] and stop
    0x01d1, 0x05f3,
    // end of x86_psll.i64x2 (I64)
    // 00077c: x86_psrl.i64x2 (I64)
    // --> [DynRexMp2fa#5d3] and stop
    0x01d1, 0x05d3,
    // end of x86_psrl.i64x2 (I64)
    // 00077e: fadd.f32x4 (I64)
    // --> [DynRexOp2fa#458] and stop
    0x01d3, 0x0458,
    // end of fadd.f32x4 (I64)
    // 000780: fcmp.f32x4 (I64)
    // --> [DynRexOp2pfcmp#4c2] and stop
    0x025d, 0x04c2,
    // end of fcmp.f32x4 (I64)
    // 000782: fcvt_from_sint.f32x4 (I64)
    // stop unless inst_predicate_26
    0x101a,
    // --> [RexOp2furm#45b]
    0x00dc, 0x045b,
    // --> [Op2furm#45b] and stop
    0x00db, 0x045b,
    // end of fcvt_from_sint.f32x4 (I64)
    // 000787: fdiv.f32x4 (I64)
    // --> [DynRexOp2fa#45e] and stop
    0x01d3, 0x045e,
    // end of fdiv.f32x4 (I64)
    // 000789: fmul.f32x4 (I64)
    // --> [DynRexOp2fa#459] and stop
    0x01d3, 0x0459,
    // end of fmul.f32x4 (I64)
    // 00078b: fsub.f32x4 (I64)
    // --> [DynRexOp2fa#45c] and stop
    0x01d3, 0x045c,
    // end of fsub.f32x4 (I64)
    // 00078d: raw_bitcast.f32x4 (I64)
    // skip 2 unless inst_predicate_20
    // 00078d: raw_bitcast.f32x4 (I32)
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    0x3019,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_26
    // skip 2 unless inst_predicate_26
    0x301a,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    0x301b,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_29
    // skip 2 unless inst_predicate_29
    0x301d,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_13
    // skip 2 unless inst_predicate_13
    0x300d,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // stop unless inst_predicate_14
    // stop unless inst_predicate_14
    0x100e,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x01c5, 0x0000,
    // end of raw_bitcast.f32x4 (I32)
    // end of raw_bitcast.f32x4 (I64)
    // 0007ae: sqrt.f32x4 (I64)
    // --> [DynRexOp2furm#451] and stop
    0x0263, 0x0451,
    // end of sqrt.f32x4 (I64)
    // 0007b0: x86_fmax.f32x4 (I64)
    // --> [DynRexOp2fa#45f] and stop
    0x01d3, 0x045f,
    // end of x86_fmax.f32x4 (I64)
    // 0007b2: x86_fmin.f32x4 (I64)
    // --> [DynRexOp2fa#45d] and stop
    0x01d3, 0x045d,
    // end of x86_fmin.f32x4 (I64)
    // 0007b4: x86_insertps.f32x4 (I64)
    // stop unless PredicateView(26)
    0x103f,
    // --> [DynRexMp3fa_ib#d21] and stop
    0x01c3, 0x0d21,
    // end of x86_insertps.f32x4 (I64)
    // 0007b7: fadd.f64x2 (I64)
    // --> [DynRexMp2fa#558] and stop
    0x01d1, 0x0558,
    // end of fadd.f64x2 (I64)
    // 0007b9: fcmp.f64x2 (I64)
    // --> [DynRexMp2pfcmp#5c2] and stop
    0x0261, 0x05c2,
    // end of fcmp.f64x2 (I64)
    // 0007bb: fdiv.f64x2 (I64)
    // --> [DynRexMp2fa#55e] and stop
    0x01d1, 0x055e,
    // end of fdiv.f64x2 (I64)
    // 0007bd: fmul.f64x2 (I64)
    // --> [DynRexMp2fa#559] and stop
    0x01d1, 0x0559,
    // end of fmul.f64x2 (I64)
    // 0007bf: fsub.f64x2 (I64)
    // --> [DynRexMp2fa#55c] and stop
    0x01d1, 0x055c,
    // end of fsub.f64x2 (I64)
    // 0007c1: raw_bitcast.f64x2 (I64)
    // skip 2 unless inst_predicate_20
    // 0007c1: raw_bitcast.f64x2 (I32)
    // skip 2 unless inst_predicate_20
    0x3014,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_21
    // skip 2 unless inst_predicate_21
    0x3015,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_22
    // skip 2 unless inst_predicate_22
    0x3016,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_23
    // skip 2 unless inst_predicate_23
    0x3017,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_24
    // skip 2 unless inst_predicate_24
    0x3018,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_25
    // skip 2 unless inst_predicate_25
    0x3019,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_26
    // skip 2 unless inst_predicate_26
    0x301a,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_27
    // skip 2 unless inst_predicate_27
    0x301b,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_28
    // skip 2 unless inst_predicate_28
    0x301c,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // skip 2 unless inst_predicate_13
    // skip 2 unless inst_predicate_13
    0x300d,
    // --> [null_fpr#00]
    // --> [null_fpr#00]
    0x01c4, 0x0000,
    // stop unless inst_predicate_14
    // stop unless inst_predicate_14
    0x100e,
    // --> [null_fpr#00] and stop
    // --> [null_fpr#00] and stop
    0x01c5, 0x0000,
    // end of raw_bitcast.f64x2 (I32)
    // end of raw_bitcast.f64x2 (I64)
    // 0007e2: sqrt.f64x2 (I64)
    // --> [DynRexMp2furm#551] and stop
    0x01e1, 0x0551,
    // end of sqrt.f64x2 (I64)
    // 0007e4: x86_fmax.f64x2 (I64)
    // --> [DynRexMp2fa#55f] and stop
    0x01d1, 0x055f,
    // end of x86_fmax.f64x2 (I64)
    // 0007e6: x86_fmin.f64x2 (I64)
    // --> [DynRexMp2fa#55d] and stop
    0x01d1, 0x055d,
    // end of x86_fmin.f64x2 (I64)
    // 0007e8: x86_movlhps.f64x2 (I64)
    // --> [DynRexOp2fa#416] and stop
    0x01d3, 0x0416,
    // end of x86_movlhps.f64x2 (I64)
    // 0007ea: x86_movsd.f64x2 (I64)
    // --> [DynRexMp2fa#710] and stop
    0x01d1, 0x0710,
    // end of x86_movsd.f64x2 (I64)
    // 0007ec: adjust_sp_down.i32 (I32)
    // --> [Op1adjustsp#29] and stop
    0x00c7, 0x0029,
    // end of adjust_sp_down.i32 (I32)
    // 0007ee: bint.i32 (I32)
    // skip 2 unless inst_predicate_6
    0x3006,
    // --> [Op2urm_noflags_abcd#4b6]
    0x0020, 0x04b6,
    // skip 2 unless inst_predicate_7
    0x3007,
    // --> [Op2urm_noflags_abcd#4b6]
    0x0020, 0x04b6,
    // stop unless inst_predicate_9
    0x1009,
    // --> [Op2urm_noflags_abcd#4b6] and stop
    0x0021, 0x04b6,
    // end of bint.i32 (I32)
    // 0007f7: bitcast.i32 (I32)
    // stop unless inst_predicate_13
    0x100d,
    // --> [Mp2rfumr#57e] and stop
    0x00d7, 0x057e,
    // end of bitcast.i32 (I32)
    // 0007fa: brnz.i32 (I32)
    // --> [Op1tjccb#75]
    0x029e, 0x0075,
    // --> [Op1tjccd#85] and stop
    0x02a3, 0x0085,
    // end of brnz.i32 (I32)
    // 0007fe: brz.i32 (I32)
    // --> [Op1tjccb#74]
    0x029e, 0x0074,
    // --> [Op1tjccd#84] and stop
    0x02a3, 0x0084,
    // end of brz.i32 (I32)
    // 000802: clz.i32 (I32)
    // stop unless PredicateView(23)
    0x103c,
    // --> [Mp2urm#6bd] and stop
    0x0185, 0x06bd,
    // end of clz.i32 (I32)
    // 000805: const_addr.i32 (I32)
    // --> [Op1const_addr#8d] and stop
    0x027f, 0x008d,
    // end of const_addr.i32 (I32)
    // 000807: copy_to_ssa.i32 (I32)
    // --> [Op1umr_reg_to_ssa#89] and stop
    // 000807: copy_to_ssa.r32 (I32)
    // --> [Op1umr_reg_to_ssa#89] and stop
    // 000807: copy_to_ssa.b1 (I32)
    // --> [Op1umr_reg_to_ssa#89] and stop
    // 000807: copy_to_ssa.i8 (I32)
    // --> [Op1umr_reg_to_ssa#89] and stop
    // 000807: copy_to_ssa.i16 (I32)
    // --> [Op1umr_reg_to_ssa#89] and stop
    0x002d, 0x0089,
    // end of copy_to_ssa.i16 (I32)
    // end of copy_to_ssa.i8 (I32)
    // end of copy_to_ssa.b1 (I32)
    // end of copy_to_ssa.r32 (I32)
    // end of copy_to_ssa.i32 (I32)
    // 000809: ctz.i32 (I32)
    // stop unless PredicateView(22)
    0x103b,
    // --> [Mp2urm#6bc] and stop
    0x0185, 0x06bc,
    // end of ctz.i32 (I32)
    // 00080c: func_addr.i32 (I32)
    // skip 2 unless PredicateView(15)
    0x3034,
    // --> [Op1fnaddr4#b8]
    0x0264, 0x00b8,
    // stop unless PredicateView(13)
    0x1032,
    // --> [Op1allones_fnaddr4#b8] and stop
    0x0269, 0x00b8,
    // end of func_addr.i32 (I32)
    // 000812: iconst.i32 (I32)
    // --> [Op1pu_id#b8]
    0x000e, 0x00b8,
    // stop unless inst_predicate_1
    // 000814: iconst.i16 (I32)
    // stop unless inst_predicate_1
    0x1001,
    // --> [Op1u_id_z#31] and stop
    // --> [Op1u_id_z#31] and stop
    0x001b, 0x0031,
    // end of iconst.i16 (I32)
    // end of iconst.i32 (I32)
    // 000817: ifcmp_sp.i32 (I32)
    // --> [Op1rcmp_sp#39] and stop
    0x01a5, 0x0039,
    // end of ifcmp_sp.i32 (I32)
    // 000819: ishl.i32 (I32)
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [Op1rc#40d3]
    0x0180, 0x40d3,
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [Op1rc#40d3]
    0x0180, 0x40d3,
    // stop unless inst_predicate_19
    0x1013,
    // --> [Op1rc#40d3] and stop
    0x0181, 0x40d3,
    // end of ishl.i32 (I32)
    // 000822: istore16.i32 (I32)
    // --> [Mp1st#189]
    0x0076, 0x0189,
    // --> [Mp1stDisp8#189]
    0x007e, 0x0189,
    // --> [Mp1stDisp32#189] and stop
    0x0087, 0x0189,
    // end of istore16.i32 (I32)
    // 000828: istore16_complex.i32 (I32)
    // stop unless inst_predicate_12
    0x100c,
    // --> [Mp1stWithIndex#189]
    0x0052, 0x0189,
    // --> [Mp1stWithIndexDisp8#189]
    0x005a, 0x0189,
    // --> [Mp1stWithIndexDisp32#189] and stop
    0x0063, 0x0189,
    // end of istore16_complex.i32 (I32)
    // 00082f: istore8.i32 (I32)
    // --> [Op1st_abcd#88]
    0x008a, 0x0088,
    // --> [Op1stDisp8_abcd#88]
    0x008c, 0x0088,
    // --> [Op1stDisp32_abcd#88] and stop
    0x008f, 0x0088,
    // end of istore8.i32 (I32)
    // 000835: istore8_complex.i32 (I32)
    // stop unless inst_predicate_12
    0x100c,
    // --> [Op1stWithIndex_abcd#88]
    0x0066, 0x0088,
    // --> [Op1stWithIndexDisp8_abcd#88]
    0x006a, 0x0088,
    // --> [Op1stWithIndexDisp32_abcd#88] and stop
    0x006f, 0x0088,
    // end of istore8_complex.i32 (I32)
    // 00083c: jump_table_base.i32 (I32)
    // --> [Op1jt_base#8d] and stop
    0x02b7, 0x008d,
    // end of jump_table_base.i32 (I32)
    // 00083e: jump_table_entry.i32 (I32)
    // --> [Op1jt_entry#8b] and stop
    0x02b3, 0x008b,
    // end of jump_table_entry.i32 (I32)
    // 000840: load.i32 (I32)
    // --> [Op1ld#8b]
    // 000840: load.r32 (I32)
    // --> [Op1ld#8b]
    0x0098, 0x008b,
    // --> [Op1ldDisp8#8b]
    // --> [Op1ldDisp8#8b]
    0x00a0, 0x008b,
    // --> [Op1ldDisp32#8b] and stop
    // --> [Op1ldDisp32#8b] and stop
    0x00a9, 0x008b,
    // end of load.r32 (I32)
    // end of load.i32 (I32)
    // 000846: load_complex.i32 (I32)
    // stop unless inst_predicate_11
    // 000846: load_complex.r32 (I32)
    // stop unless inst_predicate_11
    0x100b,
    // --> [Op1ldWithIndex#8b]
    // --> [Op1ldWithIndex#8b]
    0x0036, 0x008b,
    // --> [Op1ldWithIndexDisp8#8b]
    // --> [Op1ldWithIndexDisp8#8b]
    0x003e, 0x008b,
    // --> [Op1ldWithIndexDisp32#8b] and stop
    // --> [Op1ldWithIndexDisp32#8b] and stop
    0x0047, 0x008b,
    // end of load_complex.r32 (I32)
    // end of load_complex.i32 (I32)
    // 00084d: popcnt.i32 (I32)
    // stop unless PredicateView(24)
    0x103d,
    // --> [Mp2urm#6b8] and stop
    0x0185, 0x06b8,
    // end of popcnt.i32 (I32)
    // 000850: rotl.i32 (I32)
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [Op1rc#d3]
    0x0180, 0x00d3,
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [Op1rc#d3]
    0x0180, 0x00d3,
    // stop unless inst_predicate_19
    0x1013,
    // --> [Op1rc#d3] and stop
    0x0181, 0x00d3,
    // end of rotl.i32 (I32)
    // 000859: rotr.i32 (I32)
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [Op1rc#10d3]
    0x0180, 0x10d3,
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [Op1rc#10d3]
    0x0180, 0x10d3,
    // stop unless inst_predicate_19
    0x1013,
    // --> [Op1rc#10d3] and stop
    0x0181, 0x10d3,
    // end of rotr.i32 (I32)
    // 000862: sextend.i32 (I32)
    // skip 2 unless inst_predicate_5
    0x3005,
    // --> [Op2urm_noflags_abcd#4be]
    0x0020, 0x04be,
    // stop unless inst_predicate_2
    0x1002,
    // --> [Op2urm_noflags#4bf] and stop
    0x0025, 0x04bf,
    // end of sextend.i32 (I32)
    // 000868: sload16.i32 (I32)
    // --> [Op2ld#4bf]
    0x009c, 0x04bf,
    // --> [Op2ldDisp8#4bf]
    0x00a4, 0x04bf,
    // --> [Op2ldDisp32#4bf] and stop
    0x00ad, 0x04bf,
    // end of sload16.i32 (I32)
    // 00086e: sload16_complex.i32 (I32)
    // stop unless inst_predicate_11
    0x100b,
    // --> [Op2ldWithIndex#4bf]
    0x003a, 0x04bf,
    // --> [Op2ldWithIndexDisp8#4bf]
    0x0042, 0x04bf,
    // --> [Op2ldWithIndexDisp32#4bf] and stop
    0x004b, 0x04bf,
    // end of sload16_complex.i32 (I32)
    // 000875: sload16x4.i32 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3fld#923]
    0x022c, 0x0923,
    // --> [Mp3fldDisp8#923]
    0x0230, 0x0923,
    // --> [Mp3fldDisp32#923] and stop
    0x0235, 0x0923,
    // end of sload16x4.i32 (I32)
    // 00087c: sload32x2.i32 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3fld#925]
    0x022c, 0x0925,
    // --> [Mp3fldDisp8#925]
    0x0230, 0x0925,
    // --> [Mp3fldDisp32#925] and stop
    0x0235, 0x0925,
    // end of sload32x2.i32 (I32)
    // 000883: sload8.i32 (I32)
    // --> [Op2ld#4be]
    0x009c, 0x04be,
    // --> [Op2ldDisp8#4be]
    0x00a4, 0x04be,
    // --> [Op2ldDisp32#4be] and stop
    0x00ad, 0x04be,
    // end of sload8.i32 (I32)
    // 000889: sload8_complex.i32 (I32)
    // stop unless inst_predicate_11
    0x100b,
    // --> [Op2ldWithIndex#4be]
    0x003a, 0x04be,
    // --> [Op2ldWithIndexDisp8#4be]
    0x0042, 0x04be,
    // --> [Op2ldWithIndexDisp32#4be] and stop
    0x004b, 0x04be,
    // end of sload8_complex.i32 (I32)
    // 000890: sload8x8.i32 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3fld#920]
    0x022c, 0x0920,
    // --> [Mp3fldDisp8#920]
    0x0230, 0x0920,
    // --> [Mp3fldDisp32#920] and stop
    0x0235, 0x0920,
    // end of sload8x8.i32 (I32)
    // 000897: sshr.i32 (I32)
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [Op1rc#70d3]
    0x0180, 0x70d3,
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [Op1rc#70d3]
    0x0180, 0x70d3,
    // stop unless inst_predicate_19
    0x1013,
    // --> [Op1rc#70d3] and stop
    0x0181, 0x70d3,
    // end of sshr.i32 (I32)
    // 0008a0: stack_addr.i32 (I32)
    // --> [Op1spaddr_id#8d] and stop
    0x027b, 0x008d,
    // end of stack_addr.i32 (I32)
    // 0008a2: store.i32 (I32)
    // --> [Op1st#89]
    // 0008a2: store.r32 (I32)
    // --> [Op1st#89]
    0x0072, 0x0089,
    // --> [Op1stDisp8#89]
    // --> [Op1stDisp8#89]
    0x007a, 0x0089,
    // --> [Op1stDisp32#89] and stop
    // --> [Op1stDisp32#89] and stop
    0x0083, 0x0089,
    // end of store.r32 (I32)
    // end of store.i32 (I32)
    // 0008a8: store_complex.i32 (I32)
    // stop unless inst_predicate_12
    // 0008a8: store_complex.r32 (I32)
    // stop unless inst_predicate_12
    0x100c,
    // --> [Op1stWithIndex#89]
    // --> [Op1stWithIndex#89]
    0x004e, 0x0089,
    // --> [Op1stWithIndexDisp8#89]
    // --> [Op1stWithIndexDisp8#89]
    0x0056, 0x0089,
    // --> [Op1stWithIndexDisp32#89] and stop
    // --> [Op1stWithIndexDisp32#89] and stop
    0x005f, 0x0089,
    // end of store_complex.r32 (I32)
    // end of store_complex.i32 (I32)
    // 0008af: symbol_value.i32 (I32)
    // stop unless PredicateView(16)
    0x1035,
    // --> [Op1gvaddr4#b8] and stop
    0x0271, 0x00b8,
    // end of symbol_value.i32 (I32)
    // 0008b2: uextend.i32 (I32)
    // skip 2 unless inst_predicate_5
    0x3005,
    // --> [Op2urm_noflags_abcd#4b6]
    0x0020, 0x04b6,
    // stop unless inst_predicate_2
    0x1002,
    // --> [Op2urm_noflags#4b7] and stop
    0x0025, 0x04b7,
    // end of uextend.i32 (I32)
    // 0008b8: uload16.i32 (I32)
    // --> [Op2ld#4b7]
    0x009c, 0x04b7,
    // --> [Op2ldDisp8#4b7]
    0x00a4, 0x04b7,
    // --> [Op2ldDisp32#4b7] and stop
    0x00ad, 0x04b7,
    // end of uload16.i32 (I32)
    // 0008be: uload16_complex.i32 (I32)
    // stop unless inst_predicate_11
    0x100b,
    // --> [Op2ldWithIndex#4b7]
    0x003a, 0x04b7,
    // --> [Op2ldWithIndexDisp8#4b7]
    0x0042, 0x04b7,
    // --> [Op2ldWithIndexDisp32#4b7] and stop
    0x004b, 0x04b7,
    // end of uload16_complex.i32 (I32)
    // 0008c5: uload16x4.i32 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3fld#933]
    0x022c, 0x0933,
    // --> [Mp3fldDisp8#933]
    0x0230, 0x0933,
    // --> [Mp3fldDisp32#933] and stop
    0x0235, 0x0933,
    // end of uload16x4.i32 (I32)
    // 0008cc: uload32x2.i32 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3fld#935]
    0x022c, 0x0935,
    // --> [Mp3fldDisp8#935]
    0x0230, 0x0935,
    // --> [Mp3fldDisp32#935] and stop
    0x0235, 0x0935,
    // end of uload32x2.i32 (I32)
    // 0008d3: uload8.i32 (I32)
    // --> [Op2ld#4b6]
    0x009c, 0x04b6,
    // --> [Op2ldDisp8#4b6]
    0x00a4, 0x04b6,
    // --> [Op2ldDisp32#4b6] and stop
    0x00ad, 0x04b6,
    // end of uload8.i32 (I32)
    // 0008d9: uload8_complex.i32 (I32)
    // stop unless inst_predicate_11
    0x100b,
    // --> [Op2ldWithIndex#4b6]
    0x003a, 0x04b6,
    // --> [Op2ldWithIndexDisp8#4b6]
    0x0042, 0x04b6,
    // --> [Op2ldWithIndexDisp32#4b6] and stop
    0x004b, 0x04b6,
    // end of uload8_complex.i32 (I32)
    // 0008e0: uload8x8.i32 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3fld#930]
    0x022c, 0x0930,
    // --> [Mp3fldDisp8#930]
    0x0230, 0x0930,
    // --> [Mp3fldDisp32#930] and stop
    0x0235, 0x0930,
    // end of uload8x8.i32 (I32)
    // 0008e7: ushr.i32 (I32)
    // skip 2 unless inst_predicate_17
    0x3011,
    // --> [Op1rc#50d3]
    0x0180, 0x50d3,
    // skip 2 unless inst_predicate_18
    0x3012,
    // --> [Op1rc#50d3]
    0x0180, 0x50d3,
    // stop unless inst_predicate_19
    0x1013,
    // --> [Op1rc#50d3] and stop
    0x0181, 0x50d3,
    // end of ushr.i32 (I32)
    // 0008f0: x86_cvtt2si.i32 (I32)
    // skip 2 unless inst_predicate_13
    0x300d,
    // --> [Mp2rfurm#62c]
    0x0130, 0x062c,
    // stop unless inst_predicate_14
    0x100e,
    // --> [Mp2rfurm#72c] and stop
    0x0131, 0x072c,
    // end of x86_cvtt2si.i32 (I32)
    // 0008f6: is_invalid.r32 (I32)
    // --> [Op1is_invalid#7083] and stop
    0x02cd, 0x7083,
    // end of is_invalid.r32 (I32)
    // 0008f8: is_null.r32 (I32)
    // --> [Op1is_zero#85] and stop
    0x02c9, 0x0085,
    // end of is_null.r32 (I32)
    // 0008fa: brnz.b1 (I32)
    // --> [Op1t8jccd_long#85]
    0x02a6, 0x0085,
    // --> [Op1t8jccb_abcd#75]
    0x02a8, 0x0075,
    // --> [Op1t8jccd_abcd#85] and stop
    0x02ad, 0x0085,
    // end of brnz.b1 (I32)
    // 000900: brz.b1 (I32)
    // --> [Op1t8jccd_long#84]
    0x02a6, 0x0084,
    // --> [Op1t8jccb_abcd#74]
    0x02a8, 0x0074,
    // --> [Op1t8jccd_abcd#84] and stop
    0x02ad, 0x0084,
    // end of brz.b1 (I32)
    // 000906: bint.i8 (I32)
    // skip 2 unless inst_predicate_6
    0x3006,
    // --> [Op2urm_noflags_abcd#4b6]
    0x0020, 0x04b6,
    // stop unless inst_predicate_7
    0x1007,
    // --> [Op2urm_noflags_abcd#4b6] and stop
    0x0021, 0x04b6,
    // end of bint.i8 (I32)
    // 00090c: iconst.i8 (I32)
    // stop unless inst_predicate_1
    0x1001,
    // --> [Op1u_id_z#30] and stop
    0x001b, 0x0030,
    // end of iconst.i8 (I32)
    // 00090f: ireduce.i8 (I32)
    // skip 2 unless inst_predicate_2
    0x3002,
    // --> [null#00]
    0x001e, 0x0000,
    // stop unless inst_predicate_3
    // 000912: ireduce.i16 (I32)
    // stop unless inst_predicate_3
    0x1003,
    // --> [null#00] and stop
    // --> [null#00] and stop
    0x001f, 0x0000,
    // end of ireduce.i16 (I32)
    // end of ireduce.i8 (I32)
    // 000915: regmove.i8 (I32)
    // --> [Op1rmov#89]
    0x000a, 0x0089,
    // --> [Op1rmov#89] and stop
    0x000b, 0x0089,
    // end of regmove.i8 (I32)
    // 000919: bint.i16 (I32)
    // skip 2 unless inst_predicate_6
    0x3006,
    // --> [Op2urm_noflags_abcd#4b6]
    0x0020, 0x04b6,
    // skip 2 unless inst_predicate_7
    0x3007,
    // --> [Op2urm_noflags_abcd#4b6]
    0x0020, 0x04b6,
    // stop unless inst_predicate_8
    0x1008,
    // --> [Op2urm_noflags_abcd#4b6] and stop
    0x0021, 0x04b6,
    // end of bint.i16 (I32)
    // 000922: bint.i64 (I32)
    // stop unless inst_predicate_10
    0x100a,
    // --> [Op2urm_noflags_abcd#4b6] and stop
    0x0021, 0x04b6,
    // end of bint.i64 (I32)
    // 000925: adjust_sp_down_imm (I32)
    // --> [Op1adjustsp_ib#5083]
    0x00ca, 0x5083,
    // --> [Op1adjustsp_id#5081] and stop
    0x00cd, 0x5081,
    // end of adjust_sp_down_imm (I32)
    // 000929: adjust_sp_up_imm (I32)
    // --> [Op1adjustsp_ib#83]
    0x00ca, 0x0083,
    // --> [Op1adjustsp_id#81] and stop
    0x00cd, 0x0081,
    // end of adjust_sp_up_imm (I32)
    // 00092d: brff (I32)
    // --> [Op1brfb#70]
    0x0296, 0x0070,
    // --> [Op2brfd#480] and stop
    0x029b, 0x0480,
    // end of brff (I32)
    // 000931: brif (I32)
    // --> [Op1brib#70]
    0x028e, 0x0070,
    // --> [Op2brid#480] and stop
    0x0293, 0x0480,
    // end of brif (I32)
    // 000935: call (I32)
    // --> [Op1call_id#e8] and stop
    0x0281, 0x00e8,
    // end of call (I32)
    // 000937: copy_special (I32)
    // --> [Op1copysp#89] and stop
    0x002b, 0x0089,
    // end of copy_special (I32)
    // 000939: f32const (I32)
    // stop unless inst_predicate_15
    0x100f,
    // --> [Op2f32imm_z#457] and stop
    0x0123, 0x0457,
    // end of f32const (I32)
    // 00093c: f64const (I32)
    // stop unless inst_predicate_16
    0x1010,
    // --> [Mp2f64imm_z#557] and stop
    0x0125, 0x0557,
    // end of f64const (I32)
    // 00093f: sload16x4_complex (I32)
    // stop unless PredicateView(26)
    0x103f,
    // stop unless inst_predicate_11
    0x100b,
    // --> [Mp3fldWithIndex#923]
    0x0238, 0x0923,
    // --> [Mp3fldWithIndexDisp8#923]
    0x023c, 0x0923,
    // --> [Mp3fldWithIndexDisp32#923] and stop
    0x0241, 0x0923,
    // end of sload16x4_complex (I32)
    // 000947: sload32x2_complex (I32)
    // stop unless PredicateView(26)
    0x103f,
    // stop unless inst_predicate_11
    0x100b,
    // --> [Mp3fldWithIndex#925]
    0x0238, 0x0925,
    // --> [Mp3fldWithIndexDisp8#925]
    0x023c, 0x0925,
    // --> [Mp3fldWithIndexDisp32#925] and stop
    0x0241, 0x0925,
    // end of sload32x2_complex (I32)
    // 00094f: sload8x8_complex (I32)
    // stop unless PredicateView(26)
    0x103f,
    // stop unless inst_predicate_11
    0x100b,
    // --> [Mp3fldWithIndex#920]
    0x0238, 0x0920,
    // --> [Mp3fldWithIndexDisp8#920]
    0x023c, 0x0920,
    // --> [Mp3fldWithIndexDisp32#920] and stop
    0x0241, 0x0920,
    // end of sload8x8_complex (I32)
    // 000957: uload16x4_complex (I32)
    // stop unless PredicateView(26)
    0x103f,
    // stop unless inst_predicate_11
    0x100b,
    // --> [Mp3fldWithIndex#933]
    0x0238, 0x0933,
    // --> [Mp3fldWithIndexDisp8#933]
    0x023c, 0x0933,
    // --> [Mp3fldWithIndexDisp32#933] and stop
    0x0241, 0x0933,
    // end of uload16x4_complex (I32)
    // 00095f: uload32x2_complex (I32)
    // stop unless PredicateView(26)
    0x103f,
    // stop unless inst_predicate_11
    0x100b,
    // --> [Mp3fldWithIndex#935]
    0x0238, 0x0935,
    // --> [Mp3fldWithIndexDisp8#935]
    0x023c, 0x0935,
    // --> [Mp3fldWithIndexDisp32#935] and stop
    0x0241, 0x0935,
    // end of uload32x2_complex (I32)
    // 000967: uload8x8_complex (I32)
    // stop unless PredicateView(26)
    0x103f,
    // stop unless inst_predicate_11
    0x100b,
    // --> [Mp3fldWithIndex#930]
    0x0238, 0x0930,
    // --> [Mp3fldWithIndexDisp8#930]
    0x023c, 0x0930,
    // --> [Mp3fldWithIndexDisp32#930] and stop
    0x0241, 0x0930,
    // end of uload8x8_complex (I32)
    // 00096f: x86_pmuludq (I32)
    // --> [Mp2fa#5f4] and stop
    0x0139, 0x05f4,
    // end of x86_pmuludq (I32)
    // 000971: ceil.f64 (I32)
    // stop unless PredicateView(25)
    // 000971: floor.f64 (I32)
    // stop unless PredicateView(25)
    // 000971: nearest.f64 (I32)
    // stop unless PredicateView(25)
    // 000971: trunc.f64 (I32)
    // stop unless PredicateView(25)
    0x103e,
    // --> [Mp3furmi_rnd#d0b] and stop
    // --> [Mp3furmi_rnd#d0b] and stop
    // --> [Mp3furmi_rnd#d0b] and stop
    // --> [Mp3furmi_rnd#d0b] and stop
    0x0135, 0x0d0b,
    // end of trunc.f64 (I32)
    // end of nearest.f64 (I32)
    // end of floor.f64 (I32)
    // end of ceil.f64 (I32)
    // 000974: copy_to_ssa.f64 (I32)
    // --> [Mp2furm_reg_to_ssa#710] and stop
    0x0031, 0x0710,
    // end of copy_to_ssa.f64 (I32)
    // 000976: fcvt_from_sint.f64 (I32)
    // stop unless inst_predicate_3
    0x1003,
    // --> [DynRexMp2frurm#72a] and stop
    0x012b, 0x072a,
    // end of fcvt_from_sint.f64 (I32)
    // 000979: fpromote.f64 (I32)
    // stop unless inst_predicate_13
    0x100d,
    // --> [Mp2furm#65a] and stop
    0x012d, 0x065a,
    // end of fpromote.f64 (I32)
    // 00097c: load.f64 (I32)
    // --> [Mp2fld#710]
    0x00e2, 0x0710,
    // --> [Mp2fldDisp8#710]
    0x00e6, 0x0710,
    // --> [Mp2fldDisp32#710] and stop
    0x00eb, 0x0710,
    // end of load.f64 (I32)
    // 000982: load_complex.f64 (I32)
    // --> [Mp2fldWithIndex#710]
    0x00ee, 0x0710,
    // --> [Mp2fldWithIndexDisp8#710]
    0x00f2, 0x0710,
    // --> [Mp2fldWithIndexDisp32#710] and stop
    0x00f7, 0x0710,
    // end of load_complex.f64 (I32)
    // 000988: store.f64 (I32)
    // --> [Mp2fst#711]
    0x00fa, 0x0711,
    // --> [Mp2fstDisp8#711]
    0x00fe, 0x0711,
    // --> [Mp2fstDisp32#711] and stop
    0x0103, 0x0711,
    // end of store.f64 (I32)
    // 00098e: store_complex.f64 (I32)
    // --> [Mp2fstWithIndex#711]
    0x0106, 0x0711,
    // --> [Mp2fstWithIndexDisp8#711]
    0x010a, 0x0711,
    // --> [Mp2fstWithIndexDisp32#711] and stop
    0x010f, 0x0711,
    // end of store_complex.f64 (I32)
    // 000994: bitcast.f32 (I32)
    // stop unless inst_predicate_3
    // 000994: bitcast.i64x2 (I32)
    // stop unless inst_predicate_3
    0x1003,
    // --> [Mp2frurm#56e] and stop
    // --> [Mp2frurm#56e] and stop
    0x00d3, 0x056e,
    // end of bitcast.i64x2 (I32)
    // end of bitcast.f32 (I32)
    // 000997: ceil.f32 (I32)
    // stop unless PredicateView(25)
    // 000997: floor.f32 (I32)
    // stop unless PredicateView(25)
    // 000997: nearest.f32 (I32)
    // stop unless PredicateView(25)
    // 000997: trunc.f32 (I32)
    // stop unless PredicateView(25)
    0x103e,
    // --> [Mp3furmi_rnd#d0a] and stop
    // --> [Mp3furmi_rnd#d0a] and stop
    // --> [Mp3furmi_rnd#d0a] and stop
    // --> [Mp3furmi_rnd#d0a] and stop
    0x0135, 0x0d0a,
    // end of trunc.f32 (I32)
    // end of nearest.f32 (I32)
    // end of floor.f32 (I32)
    // end of ceil.f32 (I32)
    // 00099a: copy_to_ssa.f32 (I32)
    // --> [Mp2furm_reg_to_ssa#610] and stop
    0x0031, 0x0610,
    // end of copy_to_ssa.f32 (I32)
    // 00099c: fcvt_from_sint.f32 (I32)
    // stop unless inst_predicate_3
    0x1003,
    // --> [DynRexMp2frurm#62a] and stop
    0x012b, 0x062a,
    // end of fcvt_from_sint.f32 (I32)
    // 00099f: fdemote.f32 (I32)
    // stop unless inst_predicate_14
    0x100e,
    // --> [Mp2furm#75a] and stop
    0x012d, 0x075a,
    // end of fdemote.f32 (I32)
    // 0009a2: load.f32 (I32)
    // --> [Mp2fld#610]
    0x00e2, 0x0610,
    // --> [Mp2fldDisp8#610]
    0x00e6, 0x0610,
    // --> [Mp2fldDisp32#610] and stop
    0x00eb, 0x0610,
    // end of load.f32 (I32)
    // 0009a8: load_complex.f32 (I32)
    // --> [Mp2fldWithIndex#610]
    0x00ee, 0x0610,
    // --> [Mp2fldWithIndexDisp8#610]
    0x00f2, 0x0610,
    // --> [Mp2fldWithIndexDisp32#610] and stop
    0x00f7, 0x0610,
    // end of load_complex.f32 (I32)
    // 0009ae: store.f32 (I32)
    // --> [Mp2fst#611]
    0x00fa, 0x0611,
    // --> [Mp2fstDisp8#611]
    0x00fe, 0x0611,
    // --> [Mp2fstDisp32#611] and stop
    0x0103, 0x0611,
    // end of store.f32 (I32)
    // 0009b4: store_complex.f32 (I32)
    // --> [Mp2fstWithIndex#611]
    0x0106, 0x0611,
    // --> [Mp2fstWithIndexDisp8#611]
    0x010a, 0x0611,
    // --> [Mp2fstWithIndexDisp32#611] and stop
    0x010f, 0x0611,
    // end of store_complex.f32 (I32)
    // 0009ba: band.b8x16 (I32)
    // --> [Mp2fa#5db] and stop
    // 0009ba: band.b16x8 (I32)
    // --> [Mp2fa#5db] and stop
    // 0009ba: band.b32x4 (I32)
    // --> [Mp2fa#5db] and stop
    // 0009ba: band.b64x2 (I32)
    // --> [Mp2fa#5db] and stop
    // 0009ba: band.i8x16 (I32)
    // --> [Mp2fa#5db] and stop
    // 0009ba: band.i16x8 (I32)
    // --> [Mp2fa#5db] and stop
    // 0009ba: band.i32x4 (I32)
    // --> [Mp2fa#5db] and stop
    // 0009ba: band.i64x2 (I32)
    // --> [Mp2fa#5db] and stop
    // 0009ba: band.f32x4 (I32)
    // --> [Mp2fa#5db] and stop
    // 0009ba: band.f64x2 (I32)
    // --> [Mp2fa#5db] and stop
    0x0139, 0x05db,
    // end of band.f64x2 (I32)
    // end of band.f32x4 (I32)
    // end of band.i64x2 (I32)
    // end of band.i32x4 (I32)
    // end of band.i16x8 (I32)
    // end of band.i8x16 (I32)
    // end of band.b64x2 (I32)
    // end of band.b32x4 (I32)
    // end of band.b16x8 (I32)
    // end of band.b8x16 (I32)
    // 0009bc: band_not.b8x16 (I32)
    // --> [Mp2fax#5df] and stop
    // 0009bc: band_not.b16x8 (I32)
    // --> [Mp2fax#5df] and stop
    // 0009bc: band_not.b32x4 (I32)
    // --> [Mp2fax#5df] and stop
    // 0009bc: band_not.b64x2 (I32)
    // --> [Mp2fax#5df] and stop
    // 0009bc: band_not.i8x16 (I32)
    // --> [Mp2fax#5df] and stop
    // 0009bc: band_not.i16x8 (I32)
    // --> [Mp2fax#5df] and stop
    // 0009bc: band_not.i32x4 (I32)
    // --> [Mp2fax#5df] and stop
    // 0009bc: band_not.i64x2 (I32)
    // --> [Mp2fax#5df] and stop
    // 0009bc: band_not.f32x4 (I32)
    // --> [Mp2fax#5df] and stop
    // 0009bc: band_not.f64x2 (I32)
    // --> [Mp2fax#5df] and stop
    0x0247, 0x05df,
    // end of band_not.f64x2 (I32)
    // end of band_not.f32x4 (I32)
    // end of band_not.i64x2 (I32)
    // end of band_not.i32x4 (I32)
    // end of band_not.i16x8 (I32)
    // end of band_not.i8x16 (I32)
    // end of band_not.b64x2 (I32)
    // end of band_not.b32x4 (I32)
    // end of band_not.b16x8 (I32)
    // end of band_not.b8x16 (I32)
    // 0009be: bor.b8x16 (I32)
    // --> [Mp2fa#5eb] and stop
    // 0009be: bor.b16x8 (I32)
    // --> [Mp2fa#5eb] and stop
    // 0009be: bor.b32x4 (I32)
    // --> [Mp2fa#5eb] and stop
    // 0009be: bor.b64x2 (I32)
    // --> [Mp2fa#5eb] and stop
    // 0009be: bor.i8x16 (I32)
    // --> [Mp2fa#5eb] and stop
    // 0009be: bor.i16x8 (I32)
    // --> [Mp2fa#5eb] and stop
    // 0009be: bor.i32x4 (I32)
    // --> [Mp2fa#5eb] and stop
    // 0009be: bor.i64x2 (I32)
    // --> [Mp2fa#5eb] and stop
    // 0009be: bor.f32x4 (I32)
    // --> [Mp2fa#5eb] and stop
    // 0009be: bor.f64x2 (I32)
    // --> [Mp2fa#5eb] and stop
    0x0139, 0x05eb,
    // end of bor.f64x2 (I32)
    // end of bor.f32x4 (I32)
    // end of bor.i64x2 (I32)
    // end of bor.i32x4 (I32)
    // end of bor.i16x8 (I32)
    // end of bor.i8x16 (I32)
    // end of bor.b64x2 (I32)
    // end of bor.b32x4 (I32)
    // end of bor.b16x8 (I32)
    // end of bor.b8x16 (I32)
    // 0009c0: bxor.b8x16 (I32)
    // --> [Mp2fa#5ef] and stop
    // 0009c0: bxor.b16x8 (I32)
    // --> [Mp2fa#5ef] and stop
    // 0009c0: bxor.b32x4 (I32)
    // --> [Mp2fa#5ef] and stop
    // 0009c0: bxor.b64x2 (I32)
    // --> [Mp2fa#5ef] and stop
    // 0009c0: bxor.i8x16 (I32)
    // --> [Mp2fa#5ef] and stop
    // 0009c0: bxor.i16x8 (I32)
    // --> [Mp2fa#5ef] and stop
    // 0009c0: bxor.i32x4 (I32)
    // --> [Mp2fa#5ef] and stop
    // 0009c0: bxor.i64x2 (I32)
    // --> [Mp2fa#5ef] and stop
    // 0009c0: bxor.f32x4 (I32)
    // --> [Mp2fa#5ef] and stop
    // 0009c0: bxor.f64x2 (I32)
    // --> [Mp2fa#5ef] and stop
    0x0139, 0x05ef,
    // end of bxor.f64x2 (I32)
    // end of bxor.f32x4 (I32)
    // end of bxor.i64x2 (I32)
    // end of bxor.i32x4 (I32)
    // end of bxor.i16x8 (I32)
    // end of bxor.i8x16 (I32)
    // end of bxor.b64x2 (I32)
    // end of bxor.b32x4 (I32)
    // end of bxor.b16x8 (I32)
    // end of bxor.b8x16 (I32)
    // 0009c2: load.b8x16 (I32)
    // --> [Op2fld#410]
    // 0009c2: load.b16x8 (I32)
    // --> [Op2fld#410]
    // 0009c2: load.b32x4 (I32)
    // --> [Op2fld#410]
    // 0009c2: load.b64x2 (I32)
    // --> [Op2fld#410]
    // 0009c2: load.i8x16 (I32)
    // --> [Op2fld#410]
    // 0009c2: load.i16x8 (I32)
    // --> [Op2fld#410]
    // 0009c2: load.i32x4 (I32)
    // --> [Op2fld#410]
    // 0009c2: load.i64x2 (I32)
    // --> [Op2fld#410]
    // 0009c2: load.f32x4 (I32)
    // --> [Op2fld#410]
    // 0009c2: load.f64x2 (I32)
    // --> [Op2fld#410]
    0x0200, 0x0410,
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    // --> [Op2fldDisp8#410]
    0x0204, 0x0410,
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    // --> [Op2fldDisp32#410] and stop
    0x0209, 0x0410,
    // end of load.f64x2 (I32)
    // end of load.f32x4 (I32)
    // end of load.i64x2 (I32)
    // end of load.i32x4 (I32)
    // end of load.i16x8 (I32)
    // end of load.i8x16 (I32)
    // end of load.b64x2 (I32)
    // end of load.b32x4 (I32)
    // end of load.b16x8 (I32)
    // end of load.b8x16 (I32)
    // 0009c8: load_complex.b8x16 (I32)
    // --> [Op2fldWithIndex#410]
    // 0009c8: load_complex.b16x8 (I32)
    // --> [Op2fldWithIndex#410]
    // 0009c8: load_complex.b32x4 (I32)
    // --> [Op2fldWithIndex#410]
    // 0009c8: load_complex.b64x2 (I32)
    // --> [Op2fldWithIndex#410]
    // 0009c8: load_complex.i8x16 (I32)
    // --> [Op2fldWithIndex#410]
    // 0009c8: load_complex.i16x8 (I32)
    // --> [Op2fldWithIndex#410]
    // 0009c8: load_complex.i32x4 (I32)
    // --> [Op2fldWithIndex#410]
    // 0009c8: load_complex.i64x2 (I32)
    // --> [Op2fldWithIndex#410]
    // 0009c8: load_complex.f32x4 (I32)
    // --> [Op2fldWithIndex#410]
    // 0009c8: load_complex.f64x2 (I32)
    // --> [Op2fldWithIndex#410]
    0x020c, 0x0410,
    // --> [Op2fldWithIndexDisp8#410]
    // --> [Op2fldWithIndexDisp8#410]
    // --> [Op2fldWithIndexDisp8#410]
    // --> [Op2fldWithIndexDisp8#410]
    // --> [Op2fldWithIndexDisp8#410]
    // --> [Op2fldWithIndexDisp8#410]
    // --> [Op2fldWithIndexDisp8#410]
    // --> [Op2fldWithIndexDisp8#410]
    // --> [Op2fldWithIndexDisp8#410]
    // --> [Op2fldWithIndexDisp8#410]
    0x0210, 0x0410,
    // --> [Op2fldWithIndexDisp32#410] and stop
    // --> [Op2fldWithIndexDisp32#410] and stop
    // --> [Op2fldWithIndexDisp32#410] and stop
    // --> [Op2fldWithIndexDisp32#410] and stop
    // --> [Op2fldWithIndexDisp32#410] and stop
    // --> [Op2fldWithIndexDisp32#410] and stop
    // --> [Op2fldWithIndexDisp32#410] and stop
    // --> [Op2fldWithIndexDisp32#410] and stop
    // --> [Op2fldWithIndexDisp32#410] and stop
    // --> [Op2fldWithIndexDisp32#410] and stop
    0x0215, 0x0410,
    // end of load_complex.f64x2 (I32)
    // end of load_complex.f32x4 (I32)
    // end of load_complex.i64x2 (I32)
    // end of load_complex.i32x4 (I32)
    // end of load_complex.i16x8 (I32)
    // end of load_complex.i8x16 (I32)
    // end of load_complex.b64x2 (I32)
    // end of load_complex.b32x4 (I32)
    // end of load_complex.b16x8 (I32)
    // end of load_complex.b8x16 (I32)
    // 0009ce: store.b8x16 (I32)
    // --> [Op2fst#411]
    // 0009ce: store.b16x8 (I32)
    // --> [Op2fst#411]
    // 0009ce: store.b32x4 (I32)
    // --> [Op2fst#411]
    // 0009ce: store.b64x2 (I32)
    // --> [Op2fst#411]
    // 0009ce: store.i8x16 (I32)
    // --> [Op2fst#411]
    // 0009ce: store.i16x8 (I32)
    // --> [Op2fst#411]
    // 0009ce: store.i32x4 (I32)
    // --> [Op2fst#411]
    // 0009ce: store.i64x2 (I32)
    // --> [Op2fst#411]
    // 0009ce: store.f32x4 (I32)
    // --> [Op2fst#411]
    // 0009ce: store.f64x2 (I32)
    // --> [Op2fst#411]
    0x01e8, 0x0411,
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    // --> [Op2fstDisp8#411]
    0x01ec, 0x0411,
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    // --> [Op2fstDisp32#411] and stop
    0x01f1, 0x0411,
    // end of store.f64x2 (I32)
    // end of store.f32x4 (I32)
    // end of store.i64x2 (I32)
    // end of store.i32x4 (I32)
    // end of store.i16x8 (I32)
    // end of store.i8x16 (I32)
    // end of store.b64x2 (I32)
    // end of store.b32x4 (I32)
    // end of store.b16x8 (I32)
    // end of store.b8x16 (I32)
    // 0009d4: store_complex.b8x16 (I32)
    // --> [Op2fstWithIndex#411]
    // 0009d4: store_complex.b16x8 (I32)
    // --> [Op2fstWithIndex#411]
    // 0009d4: store_complex.b32x4 (I32)
    // --> [Op2fstWithIndex#411]
    // 0009d4: store_complex.b64x2 (I32)
    // --> [Op2fstWithIndex#411]
    // 0009d4: store_complex.i8x16 (I32)
    // --> [Op2fstWithIndex#411]
    // 0009d4: store_complex.i16x8 (I32)
    // --> [Op2fstWithIndex#411]
    // 0009d4: store_complex.i32x4 (I32)
    // --> [Op2fstWithIndex#411]
    // 0009d4: store_complex.i64x2 (I32)
    // --> [Op2fstWithIndex#411]
    // 0009d4: store_complex.f32x4 (I32)
    // --> [Op2fstWithIndex#411]
    // 0009d4: store_complex.f64x2 (I32)
    // --> [Op2fstWithIndex#411]
    0x01f4, 0x0411,
    // --> [Op2fstWithIndexDisp8#411]
    // --> [Op2fstWithIndexDisp8#411]
    // --> [Op2fstWithIndexDisp8#411]
    // --> [Op2fstWithIndexDisp8#411]
    // --> [Op2fstWithIndexDisp8#411]
    // --> [Op2fstWithIndexDisp8#411]
    // --> [Op2fstWithIndexDisp8#411]
    // --> [Op2fstWithIndexDisp8#411]
    // --> [Op2fstWithIndexDisp8#411]
    // --> [Op2fstWithIndexDisp8#411]
    0x01f8, 0x0411,
    // --> [Op2fstWithIndexDisp32#411] and stop
    // --> [Op2fstWithIndexDisp32#411] and stop
    // --> [Op2fstWithIndexDisp32#411] and stop
    // --> [Op2fstWithIndexDisp32#411] and stop
    // --> [Op2fstWithIndexDisp32#411] and stop
    // --> [Op2fstWithIndexDisp32#411] and stop
    // --> [Op2fstWithIndexDisp32#411] and stop
    // --> [Op2fstWithIndexDisp32#411] and stop
    // --> [Op2fstWithIndexDisp32#411] and stop
    // --> [Op2fstWithIndexDisp32#411] and stop
    0x01fd, 0x0411,
    // end of store_complex.f64x2 (I32)
    // end of store_complex.f32x4 (I32)
    // end of store_complex.i64x2 (I32)
    // end of store_complex.i32x4 (I32)
    // end of store_complex.i16x8 (I32)
    // end of store_complex.i8x16 (I32)
    // end of store_complex.b64x2 (I32)
    // end of store_complex.b32x4 (I32)
    // end of store_complex.b16x8 (I32)
    // end of store_complex.b8x16 (I32)
    // 0009da: vconst.b8x16 (I32)
    // skip 2 unless inst_predicate_30
    // 0009da: vconst.b16x8 (I32)
    // skip 2 unless inst_predicate_30
    // 0009da: vconst.b32x4 (I32)
    // skip 2 unless inst_predicate_30
    // 0009da: vconst.b64x2 (I32)
    // skip 2 unless inst_predicate_30
    // 0009da: vconst.i8x16 (I32)
    // skip 2 unless inst_predicate_30
    // 0009da: vconst.i16x8 (I32)
    // skip 2 unless inst_predicate_30
    // 0009da: vconst.i32x4 (I32)
    // skip 2 unless inst_predicate_30
    // 0009da: vconst.i64x2 (I32)
    // skip 2 unless inst_predicate_30
    // 0009da: vconst.f32x4 (I32)
    // skip 2 unless inst_predicate_30
    // 0009da: vconst.f64x2 (I32)
    // skip 2 unless inst_predicate_30
    0x301e,
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    // --> [DynRexMp2vconst_optimized#5ef]
    0x01e2, 0x05ef,
    // skip 2 unless inst_predicate_31
    // skip 2 unless inst_predicate_31
    // skip 2 unless inst_predicate_31
    // skip 2 unless inst_predicate_31
    // skip 2 unless inst_predicate_31
    // skip 2 unless inst_predicate_31
    // skip 2 unless inst_predicate_31
    // skip 2 unless inst_predicate_31
    // skip 2 unless inst_predicate_31
    // skip 2 unless inst_predicate_31
    0x301f,
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    // --> [DynRexMp2vconst_optimized#574]
    0x01e2, 0x0574,
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    // --> [Op2vconst#410] and stop
    0x01e5, 0x0410,
    // end of vconst.f64x2 (I32)
    // end of vconst.f32x4 (I32)
    // end of vconst.i64x2 (I32)
    // end of vconst.i32x4 (I32)
    // end of vconst.i16x8 (I32)
    // end of vconst.i8x16 (I32)
    // end of vconst.b64x2 (I32)
    // end of vconst.b32x4 (I32)
    // end of vconst.b16x8 (I32)
    // end of vconst.b8x16 (I32)
    // 0009e2: vselect.b8x16 (I32)
    // stop unless PredicateView(26)
    // 0009e2: vselect.b16x8 (I32)
    // stop unless PredicateView(26)
    // 0009e2: vselect.i8x16 (I32)
    // stop unless PredicateView(26)
    // 0009e2: vselect.i16x8 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3blend#910] and stop
    // --> [Mp3blend#910] and stop
    // --> [Mp3blend#910] and stop
    // --> [Mp3blend#910] and stop
    0x01bd, 0x0910,
    // end of vselect.i16x8 (I32)
    // end of vselect.i8x16 (I32)
    // end of vselect.b16x8 (I32)
    // end of vselect.b8x16 (I32)
    // 0009e5: x86_pextr.b8x16 (I32)
    // stop unless PredicateView(26)
    // 0009e5: x86_pextr.i8x16 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3r_ib_unsigned_gpr#d14] and stop
    // --> [Mp3r_ib_unsigned_gpr#d14] and stop
    0x01d5, 0x0d14,
    // end of x86_pextr.i8x16 (I32)
    // end of x86_pextr.b8x16 (I32)
    // 0009e8: x86_pinsr.b8x16 (I32)
    // stop unless PredicateView(26)
    // 0009e8: x86_pinsr.i8x16 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3r_ib_unsigned_r#d20] and stop
    // --> [Mp3r_ib_unsigned_r#d20] and stop
    0x01c7, 0x0d20,
    // end of x86_pinsr.i8x16 (I32)
    // end of x86_pinsr.b8x16 (I32)
    // 0009eb: x86_pshufb.b8x16 (I32)
    // stop unless PredicateView(30)
    // 0009eb: x86_pshufb.b16x8 (I32)
    // stop unless PredicateView(30)
    // 0009eb: x86_pshufb.b32x4 (I32)
    // stop unless PredicateView(30)
    // 0009eb: x86_pshufb.b64x2 (I32)
    // stop unless PredicateView(30)
    // 0009eb: x86_pshufb.i8x16 (I32)
    // stop unless PredicateView(30)
    // 0009eb: x86_pshufb.i16x8 (I32)
    // stop unless PredicateView(30)
    // 0009eb: x86_pshufb.i32x4 (I32)
    // stop unless PredicateView(30)
    // 0009eb: x86_pshufb.i64x2 (I32)
    // stop unless PredicateView(30)
    // 0009eb: x86_pshufb.f32x4 (I32)
    // stop unless PredicateView(30)
    // 0009eb: x86_pshufb.f64x2 (I32)
    // stop unless PredicateView(30)
    0x1043,
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    // --> [Mp3fa#900] and stop
    0x01b5, 0x0900,
    // end of x86_pshufb.f64x2 (I32)
    // end of x86_pshufb.f32x4 (I32)
    // end of x86_pshufb.i64x2 (I32)
    // end of x86_pshufb.i32x4 (I32)
    // end of x86_pshufb.i16x8 (I32)
    // end of x86_pshufb.i8x16 (I32)
    // end of x86_pshufb.b64x2 (I32)
    // end of x86_pshufb.b32x4 (I32)
    // end of x86_pshufb.b16x8 (I32)
    // end of x86_pshufb.b8x16 (I32)
    // 0009ee: x86_ptest.b8x16 (I32)
    // stop unless PredicateView(26)
    // 0009ee: x86_ptest.b16x8 (I32)
    // stop unless PredicateView(26)
    // 0009ee: x86_ptest.b32x4 (I32)
    // stop unless PredicateView(26)
    // 0009ee: x86_ptest.b64x2 (I32)
    // stop unless PredicateView(26)
    // 0009ee: x86_ptest.i8x16 (I32)
    // stop unless PredicateView(26)
    // 0009ee: x86_ptest.i16x8 (I32)
    // stop unless PredicateView(26)
    // 0009ee: x86_ptest.i32x4 (I32)
    // stop unless PredicateView(26)
    // 0009ee: x86_ptest.i64x2 (I32)
    // stop unless PredicateView(26)
    // 0009ee: x86_ptest.f32x4 (I32)
    // stop unless PredicateView(26)
    // 0009ee: x86_ptest.f64x2 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    // --> [Mp3fcmp#917] and stop
    0x024b, 0x0917,
    // end of x86_ptest.f64x2 (I32)
    // end of x86_ptest.f32x4 (I32)
    // end of x86_ptest.i64x2 (I32)
    // end of x86_ptest.i32x4 (I32)
    // end of x86_ptest.i16x8 (I32)
    // end of x86_ptest.i8x16 (I32)
    // end of x86_ptest.b64x2 (I32)
    // end of x86_ptest.b32x4 (I32)
    // end of x86_ptest.b16x8 (I32)
    // end of x86_ptest.b8x16 (I32)
    // 0009f1: x86_punpckh.b8x16 (I32)
    // --> [Mp2fa#568] and stop
    // 0009f1: x86_punpckh.i8x16 (I32)
    // --> [Mp2fa#568] and stop
    0x0139, 0x0568,
    // end of x86_punpckh.i8x16 (I32)
    // end of x86_punpckh.b8x16 (I32)
    // 0009f3: x86_punpckl.b8x16 (I32)
    // --> [Mp2fa#560] and stop
    // 0009f3: x86_punpckl.i8x16 (I32)
    // --> [Mp2fa#560] and stop
    0x0139, 0x0560,
    // end of x86_punpckl.i8x16 (I32)
    // end of x86_punpckl.b8x16 (I32)
    // 0009f5: x86_pblendw.b16x8 (I32)
    // stop unless PredicateView(26)
    // 0009f5: x86_pblendw.i16x8 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3fa_ib#d0e] and stop
    // --> [Mp3fa_ib#d0e] and stop
    0x01c1, 0x0d0e,
    // end of x86_pblendw.i16x8 (I32)
    // end of x86_pblendw.b16x8 (I32)
    // 0009f8: x86_pextr.b16x8 (I32)
    // stop unless PredicateView(26)
    // 0009f8: x86_pextr.i16x8 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3r_ib_unsigned_gpr#d15] and stop
    // --> [Mp3r_ib_unsigned_gpr#d15] and stop
    0x01d5, 0x0d15,
    // end of x86_pextr.i16x8 (I32)
    // end of x86_pextr.b16x8 (I32)
    // 0009fb: x86_pinsr.b16x8 (I32)
    // --> [Mp2r_ib_unsigned_r#5c4] and stop
    // 0009fb: x86_pinsr.i16x8 (I32)
    // --> [Mp2r_ib_unsigned_r#5c4] and stop
    0x01cb, 0x05c4,
    // end of x86_pinsr.i16x8 (I32)
    // end of x86_pinsr.b16x8 (I32)
    // 0009fd: x86_punpckh.b16x8 (I32)
    // --> [Mp2fa#569] and stop
    // 0009fd: x86_punpckh.i16x8 (I32)
    // --> [Mp2fa#569] and stop
    0x0139, 0x0569,
    // end of x86_punpckh.i16x8 (I32)
    // end of x86_punpckh.b16x8 (I32)
    // 0009ff: x86_punpckl.b16x8 (I32)
    // --> [Mp2fa#561] and stop
    // 0009ff: x86_punpckl.i16x8 (I32)
    // --> [Mp2fa#561] and stop
    0x0139, 0x0561,
    // end of x86_punpckl.i16x8 (I32)
    // end of x86_punpckl.b16x8 (I32)
    // 000a01: vselect.b32x4 (I32)
    // stop unless PredicateView(26)
    // 000a01: vselect.i32x4 (I32)
    // stop unless PredicateView(26)
    // 000a01: vselect.f32x4 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3blend#914] and stop
    // --> [Mp3blend#914] and stop
    // --> [Mp3blend#914] and stop
    0x01bd, 0x0914,
    // end of vselect.f32x4 (I32)
    // end of vselect.i32x4 (I32)
    // end of vselect.b32x4 (I32)
    // 000a04: x86_pextr.b32x4 (I32)
    // stop unless PredicateView(26)
    // 000a04: x86_pextr.i32x4 (I32)
    // stop unless PredicateView(26)
    // 000a04: x86_pextr.f32x4 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3r_ib_unsigned_gpr#d16] and stop
    // --> [Mp3r_ib_unsigned_gpr#d16] and stop
    // --> [Mp3r_ib_unsigned_gpr#d16] and stop
    0x01d5, 0x0d16,
    // end of x86_pextr.f32x4 (I32)
    // end of x86_pextr.i32x4 (I32)
    // end of x86_pextr.b32x4 (I32)
    // 000a07: x86_pinsr.b32x4 (I32)
    // stop unless PredicateView(26)
    // 000a07: x86_pinsr.i32x4 (I32)
    // stop unless PredicateView(26)
    // 000a07: x86_pinsr.f32x4 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3r_ib_unsigned_r#d22] and stop
    // --> [Mp3r_ib_unsigned_r#d22] and stop
    // --> [Mp3r_ib_unsigned_r#d22] and stop
    0x01c7, 0x0d22,
    // end of x86_pinsr.f32x4 (I32)
    // end of x86_pinsr.i32x4 (I32)
    // end of x86_pinsr.b32x4 (I32)
    // 000a0a: x86_pshufd.b32x4 (I32)
    // --> [Mp2r_ib_unsigned_fpr#570] and stop
    // 000a0a: x86_pshufd.i32x4 (I32)
    // --> [Mp2r_ib_unsigned_fpr#570] and stop
    // 000a0a: x86_pshufd.f32x4 (I32)
    // --> [Mp2r_ib_unsigned_fpr#570] and stop
    0x01b9, 0x0570,
    // end of x86_pshufd.f32x4 (I32)
    // end of x86_pshufd.i32x4 (I32)
    // end of x86_pshufd.b32x4 (I32)
    // 000a0c: x86_punpckh.b32x4 (I32)
    // --> [Mp2fa#56a] and stop
    // 000a0c: x86_punpckh.i32x4 (I32)
    // --> [Mp2fa#56a] and stop
    // 000a0c: x86_punpckh.f32x4 (I32)
    // --> [Mp2fa#56a] and stop
    0x0139, 0x056a,
    // end of x86_punpckh.f32x4 (I32)
    // end of x86_punpckh.i32x4 (I32)
    // end of x86_punpckh.b32x4 (I32)
    // 000a0e: x86_punpckl.b32x4 (I32)
    // --> [Mp2fa#562] and stop
    // 000a0e: x86_punpckl.i32x4 (I32)
    // --> [Mp2fa#562] and stop
    // 000a0e: x86_punpckl.f32x4 (I32)
    // --> [Mp2fa#562] and stop
    0x0139, 0x0562,
    // end of x86_punpckl.f32x4 (I32)
    // end of x86_punpckl.i32x4 (I32)
    // end of x86_punpckl.b32x4 (I32)
    // 000a10: vselect.b64x2 (I32)
    // stop unless PredicateView(26)
    // 000a10: vselect.i64x2 (I32)
    // stop unless PredicateView(26)
    // 000a10: vselect.f64x2 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3blend#915] and stop
    // --> [Mp3blend#915] and stop
    // --> [Mp3blend#915] and stop
    0x01bd, 0x0915,
    // end of vselect.f64x2 (I32)
    // end of vselect.i64x2 (I32)
    // end of vselect.b64x2 (I32)
    // 000a13: x86_punpckh.b64x2 (I32)
    // --> [Mp2fa#56d] and stop
    // 000a13: x86_punpckh.i64x2 (I32)
    // --> [Mp2fa#56d] and stop
    // 000a13: x86_punpckh.f64x2 (I32)
    // --> [Mp2fa#56d] and stop
    0x0139, 0x056d,
    // end of x86_punpckh.f64x2 (I32)
    // end of x86_punpckh.i64x2 (I32)
    // end of x86_punpckh.b64x2 (I32)
    // 000a15: x86_punpckl.b64x2 (I32)
    // --> [Mp2fa#56c] and stop
    // 000a15: x86_punpckl.i64x2 (I32)
    // --> [Mp2fa#56c] and stop
    // 000a15: x86_punpckl.f64x2 (I32)
    // --> [Mp2fa#56c] and stop
    0x0139, 0x056c,
    // end of x86_punpckl.f64x2 (I32)
    // end of x86_punpckl.i64x2 (I32)
    // end of x86_punpckl.b64x2 (I32)
    // 000a17: avg_round.i8x16 (I32)
    // --> [Mp2fa#5e0] and stop
    0x0139, 0x05e0,
    // end of avg_round.i8x16 (I32)
    // 000a19: iabs.i8x16 (I32)
    // stop unless PredicateView(30)
    0x1043,
    // --> [Mp3furm#91c] and stop
    0x01db, 0x091c,
    // end of iabs.i8x16 (I32)
    // 000a1c: iadd.i8x16 (I32)
    // --> [Mp2fa#5fc] and stop
    0x0139, 0x05fc,
    // end of iadd.i8x16 (I32)
    // 000a1e: icmp.i8x16 (I32)
    // skip 2 unless inst_predicate_32
    0x3020,
    // --> [Mp2icscc_fpr#574]
    0x0252, 0x0574,
    // stop unless inst_predicate_33
    0x1021,
    // --> [Mp2icscc_fpr#564] and stop
    0x0253, 0x0564,
    // end of icmp.i8x16 (I32)
    // 000a24: isub.i8x16 (I32)
    // --> [Mp2fa#5f8] and stop
    0x0139, 0x05f8,
    // end of isub.i8x16 (I32)
    // 000a26: sadd_sat.i8x16 (I32)
    // --> [Mp2fa#5ec] and stop
    0x0139, 0x05ec,
    // end of sadd_sat.i8x16 (I32)
    // 000a28: ssub_sat.i8x16 (I32)
    // --> [Mp2fa#5e8] and stop
    0x0139, 0x05e8,
    // end of ssub_sat.i8x16 (I32)
    // 000a2a: swiden_low.i8x16 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3furm#920] and stop
    0x01db, 0x0920,
    // end of swiden_low.i8x16 (I32)
    // 000a2d: uadd_sat.i8x16 (I32)
    // --> [Mp2fa#5dc] and stop
    0x0139, 0x05dc,
    // end of uadd_sat.i8x16 (I32)
    // 000a2f: usub_sat.i8x16 (I32)
    // --> [Mp2fa#5d8] and stop
    0x0139, 0x05d8,
    // end of usub_sat.i8x16 (I32)
    // 000a31: uwiden_low.i8x16 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3furm#930] and stop
    0x01db, 0x0930,
    // end of uwiden_low.i8x16 (I32)
    // 000a34: x86_palignr.i8x16 (I32)
    // stop unless PredicateView(30)
    // 000a34: x86_palignr.i16x8 (I32)
    // stop unless PredicateView(30)
    // 000a34: x86_palignr.i32x4 (I32)
    // stop unless PredicateView(30)
    // 000a34: x86_palignr.i64x2 (I32)
    // stop unless PredicateView(30)
    0x1043,
    // --> [Mp3fa_ib#d0f] and stop
    // --> [Mp3fa_ib#d0f] and stop
    // --> [Mp3fa_ib#d0f] and stop
    // --> [Mp3fa_ib#d0f] and stop
    0x01c1, 0x0d0f,
    // end of x86_palignr.i64x2 (I32)
    // end of x86_palignr.i32x4 (I32)
    // end of x86_palignr.i16x8 (I32)
    // end of x86_palignr.i8x16 (I32)
    // 000a37: x86_pmaxs.i8x16 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3fa#93c] and stop
    0x01b5, 0x093c,
    // end of x86_pmaxs.i8x16 (I32)
    // 000a3a: x86_pmaxu.i8x16 (I32)
    // --> [Mp2fa#5de] and stop
    0x0139, 0x05de,
    // end of x86_pmaxu.i8x16 (I32)
    // 000a3c: x86_pmins.i8x16 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3fa#938] and stop
    0x01b5, 0x0938,
    // end of x86_pmins.i8x16 (I32)
    // 000a3f: x86_pminu.i8x16 (I32)
    // --> [Mp2fa#5da] and stop
    0x0139, 0x05da,
    // end of x86_pminu.i8x16 (I32)
    // 000a41: avg_round.i16x8 (I32)
    // --> [Mp2fa#5e3] and stop
    0x0139, 0x05e3,
    // end of avg_round.i16x8 (I32)
    // 000a43: iabs.i16x8 (I32)
    // stop unless PredicateView(30)
    0x1043,
    // --> [Mp3furm#91d] and stop
    0x01db, 0x091d,
    // end of iabs.i16x8 (I32)
    // 000a46: iadd.i16x8 (I32)
    // --> [Mp2fa#5fd] and stop
    0x0139, 0x05fd,
    // end of iadd.i16x8 (I32)
    // 000a48: icmp.i16x8 (I32)
    // skip 2 unless inst_predicate_32
    0x3020,
    // --> [Mp2icscc_fpr#575]
    0x0252, 0x0575,
    // stop unless inst_predicate_33
    0x1021,
    // --> [Mp2icscc_fpr#565] and stop
    0x0253, 0x0565,
    // end of icmp.i16x8 (I32)
    // 000a4e: imul.i16x8 (I32)
    // --> [Mp2fa#5d5] and stop
    0x0139, 0x05d5,
    // end of imul.i16x8 (I32)
    // 000a50: ishl_imm.i16x8 (I32)
    // --> [Mp2f_ib#6571] and stop
    0x024f, 0x6571,
    // end of ishl_imm.i16x8 (I32)
    // 000a52: isub.i16x8 (I32)
    // --> [Mp2fa#5f9] and stop
    0x0139, 0x05f9,
    // end of isub.i16x8 (I32)
    // 000a54: sadd_sat.i16x8 (I32)
    // --> [Mp2fa#5ed] and stop
    0x0139, 0x05ed,
    // end of sadd_sat.i16x8 (I32)
    // 000a56: snarrow.i16x8 (I32)
    // --> [Mp2fa#563] and stop
    0x0139, 0x0563,
    // end of snarrow.i16x8 (I32)
    // 000a58: sshr_imm.i16x8 (I32)
    // --> [Mp2f_ib#4571] and stop
    0x024f, 0x4571,
    // end of sshr_imm.i16x8 (I32)
    // 000a5a: ssub_sat.i16x8 (I32)
    // --> [Mp2fa#5e9] and stop
    0x0139, 0x05e9,
    // end of ssub_sat.i16x8 (I32)
    // 000a5c: swiden_low.i16x8 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3furm#923] and stop
    0x01db, 0x0923,
    // end of swiden_low.i16x8 (I32)
    // 000a5f: uadd_sat.i16x8 (I32)
    // --> [Mp2fa#5dd] and stop
    0x0139, 0x05dd,
    // end of uadd_sat.i16x8 (I32)
    // 000a61: unarrow.i16x8 (I32)
    // --> [Mp2fa#567] and stop
    0x0139, 0x0567,
    // end of unarrow.i16x8 (I32)
    // 000a63: ushr_imm.i16x8 (I32)
    // --> [Mp2f_ib#2571] and stop
    0x024f, 0x2571,
    // end of ushr_imm.i16x8 (I32)
    // 000a65: usub_sat.i16x8 (I32)
    // --> [Mp2fa#5d9] and stop
    0x0139, 0x05d9,
    // end of usub_sat.i16x8 (I32)
    // 000a67: uwiden_low.i16x8 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3furm#933] and stop
    0x01db, 0x0933,
    // end of uwiden_low.i16x8 (I32)
    // 000a6a: x86_pmaxs.i16x8 (I32)
    // --> [Mp2fa#5ee] and stop
    0x0139, 0x05ee,
    // end of x86_pmaxs.i16x8 (I32)
    // 000a6c: x86_pmaxu.i16x8 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3fa#93e] and stop
    0x01b5, 0x093e,
    // end of x86_pmaxu.i16x8 (I32)
    // 000a6f: x86_pmins.i16x8 (I32)
    // --> [Mp2fa#5ea] and stop
    0x0139, 0x05ea,
    // end of x86_pmins.i16x8 (I32)
    // 000a71: x86_pminu.i16x8 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3fa#93a] and stop
    0x01b5, 0x093a,
    // end of x86_pminu.i16x8 (I32)
    // 000a74: x86_psll.i16x8 (I32)
    // --> [Mp2fa#5f1] and stop
    0x0139, 0x05f1,
    // end of x86_psll.i16x8 (I32)
    // 000a76: x86_psra.i16x8 (I32)
    // --> [Mp2fa#5e1] and stop
    0x0139, 0x05e1,
    // end of x86_psra.i16x8 (I32)
    // 000a78: x86_psrl.i16x8 (I32)
    // --> [Mp2fa#5d1] and stop
    0x0139, 0x05d1,
    // end of x86_psrl.i16x8 (I32)
    // 000a7a: iabs.i32x4 (I32)
    // stop unless PredicateView(30)
    0x1043,
    // --> [Mp3furm#91e] and stop
    0x01db, 0x091e,
    // end of iabs.i32x4 (I32)
    // 000a7d: iadd.i32x4 (I32)
    // --> [Mp2fa#5fe] and stop
    0x0139, 0x05fe,
    // end of iadd.i32x4 (I32)
    // 000a7f: icmp.i32x4 (I32)
    // skip 2 unless inst_predicate_32
    0x3020,
    // --> [Mp2icscc_fpr#576]
    0x0252, 0x0576,
    // stop unless inst_predicate_33
    0x1021,
    // --> [Mp2icscc_fpr#566] and stop
    0x0253, 0x0566,
    // end of icmp.i32x4 (I32)
    // 000a85: imul.i32x4 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3fa#940] and stop
    0x01b5, 0x0940,
    // end of imul.i32x4 (I32)
    // 000a88: ishl_imm.i32x4 (I32)
    // --> [Mp2f_ib#6572] and stop
    0x024f, 0x6572,
    // end of ishl_imm.i32x4 (I32)
    // 000a8a: isub.i32x4 (I32)
    // --> [Mp2fa#5fa] and stop
    0x0139, 0x05fa,
    // end of isub.i32x4 (I32)
    // 000a8c: snarrow.i32x4 (I32)
    // --> [Mp2fa#56b] and stop
    0x0139, 0x056b,
    // end of snarrow.i32x4 (I32)
    // 000a8e: sshr_imm.i32x4 (I32)
    // --> [Mp2f_ib#4572] and stop
    0x024f, 0x4572,
    // end of sshr_imm.i32x4 (I32)
    // 000a90: unarrow.i32x4 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3fa#92b] and stop
    0x01b5, 0x092b,
    // end of unarrow.i32x4 (I32)
    // 000a93: ushr_imm.i32x4 (I32)
    // --> [Mp2f_ib#2572] and stop
    0x024f, 0x2572,
    // end of ushr_imm.i32x4 (I32)
    // 000a95: x86_cvtt2si.i32x4 (I32)
    // stop unless inst_predicate_28
    0x101c,
    // --> [Mp2furm#65b] and stop
    0x012d, 0x065b,
    // end of x86_cvtt2si.i32x4 (I32)
    // 000a98: x86_pmaxs.i32x4 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3fa#93d] and stop
    0x01b5, 0x093d,
    // end of x86_pmaxs.i32x4 (I32)
    // 000a9b: x86_pmaxu.i32x4 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3fa#93f] and stop
    0x01b5, 0x093f,
    // end of x86_pmaxu.i32x4 (I32)
    // 000a9e: x86_pmins.i32x4 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3fa#939] and stop
    0x01b5, 0x0939,
    // end of x86_pmins.i32x4 (I32)
    // 000aa1: x86_pminu.i32x4 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3fa#93b] and stop
    0x01b5, 0x093b,
    // end of x86_pminu.i32x4 (I32)
    // 000aa4: x86_psll.i32x4 (I32)
    // --> [Mp2fa#5f2] and stop
    0x0139, 0x05f2,
    // end of x86_psll.i32x4 (I32)
    // 000aa6: x86_psra.i32x4 (I32)
    // --> [Mp2fa#5e2] and stop
    0x0139, 0x05e2,
    // end of x86_psra.i32x4 (I32)
    // 000aa8: x86_psrl.i32x4 (I32)
    // --> [Mp2fa#5d2] and stop
    0x0139, 0x05d2,
    // end of x86_psrl.i32x4 (I32)
    // 000aaa: iadd.i64x2 (I32)
    // --> [Mp2fa#5d4] and stop
    0x0139, 0x05d4,
    // end of iadd.i64x2 (I32)
    // 000aac: icmp.i64x2 (I32)
    // skip 3 unless PredicateView(26)
    0x403f,
    // skip 2 unless inst_predicate_32
    0x3020,
    // --> [Mp3icscc_fpr#929]
    0x0256, 0x0929,
    // stop unless PredicateView(28)
    0x1041,
    // stop unless inst_predicate_33
    0x1021,
    // --> [Mp3icscc_fpr#937] and stop
    0x0257, 0x0937,
    // end of icmp.i64x2 (I32)
    // 000ab4: ishl_imm.i64x2 (I32)
    // --> [Mp2f_ib#6573] and stop
    0x024f, 0x6573,
    // end of ishl_imm.i64x2 (I32)
    // 000ab6: isub.i64x2 (I32)
    // --> [Mp2fa#5fb] and stop
    0x0139, 0x05fb,
    // end of isub.i64x2 (I32)
    // 000ab8: ushr_imm.i64x2 (I32)
    // --> [Mp2f_ib#2573] and stop
    0x024f, 0x2573,
    // end of ushr_imm.i64x2 (I32)
    // 000aba: x86_psll.i64x2 (I32)
    // --> [Mp2fa#5f3] and stop
    0x0139, 0x05f3,
    // end of x86_psll.i64x2 (I32)
    // 000abc: x86_psrl.i64x2 (I32)
    // --> [Mp2fa#5d3] and stop
    0x0139, 0x05d3,
    // end of x86_psrl.i64x2 (I32)
    // 000abe: fadd.f32x4 (I32)
    // --> [Op2fa#458] and stop
    0x0179, 0x0458,
    // end of fadd.f32x4 (I32)
    // 000ac0: fcmp.f32x4 (I32)
    // --> [Op2pfcmp#4c2] and stop
    0x025b, 0x04c2,
    // end of fcmp.f32x4 (I32)
    // 000ac2: fcvt_from_sint.f32x4 (I32)
    // stop unless inst_predicate_26
    0x101a,
    // --> [Op2furm#45b] and stop
    0x00db, 0x045b,
    // end of fcvt_from_sint.f32x4 (I32)
    // 000ac5: fdiv.f32x4 (I32)
    // --> [Op2fa#45e] and stop
    0x0179, 0x045e,
    // end of fdiv.f32x4 (I32)
    // 000ac7: fmul.f32x4 (I32)
    // --> [Op2fa#459] and stop
    0x0179, 0x0459,
    // end of fmul.f32x4 (I32)
    // 000ac9: fsub.f32x4 (I32)
    // --> [Op2fa#45c] and stop
    0x0179, 0x045c,
    // end of fsub.f32x4 (I32)
    // 000acb: sqrt.f32x4 (I32)
    // --> [Op2furm#451] and stop
    0x00db, 0x0451,
    // end of sqrt.f32x4 (I32)
    // 000acd: x86_fmax.f32x4 (I32)
    // --> [Op2fa#45f] and stop
    0x0179, 0x045f,
    // end of x86_fmax.f32x4 (I32)
    // 000acf: x86_fmin.f32x4 (I32)
    // --> [Op2fa#45d] and stop
    0x0179, 0x045d,
    // end of x86_fmin.f32x4 (I32)
    // 000ad1: x86_insertps.f32x4 (I32)
    // stop unless PredicateView(26)
    0x103f,
    // --> [Mp3fa_ib#d21] and stop
    0x01c1, 0x0d21,
    // end of x86_insertps.f32x4 (I32)
    // 000ad4: fadd.f64x2 (I32)
    // --> [Mp2fa#558] and stop
    0x0139, 0x0558,
    // end of fadd.f64x2 (I32)
    // 000ad6: fcmp.f64x2 (I32)
    // --> [Mp2pfcmp#5c2] and stop
    0x025f, 0x05c2,
    // end of fcmp.f64x2 (I32)
    // 000ad8: fdiv.f64x2 (I32)
    // --> [Mp2fa#55e] and stop
    0x0139, 0x055e,
    // end of fdiv.f64x2 (I32)
    // 000ada: fmul.f64x2 (I32)
    // --> [Mp2fa#559] and stop
    0x0139, 0x0559,
    // end of fmul.f64x2 (I32)
    // 000adc: fsub.f64x2 (I32)
    // --> [Mp2fa#55c] and stop
    0x0139, 0x055c,
    // end of fsub.f64x2 (I32)
    // 000ade: sqrt.f64x2 (I32)
    // --> [Mp2furm#551] and stop
    0x012d, 0x0551,
    // end of sqrt.f64x2 (I32)
    // 000ae0: x86_fmax.f64x2 (I32)
    // --> [Mp2fa#55f] and stop
    0x0139, 0x055f,
    // end of x86_fmax.f64x2 (I32)
    // 000ae2: x86_fmin.f64x2 (I32)
    // --> [Mp2fa#55d] and stop
    0x0139, 0x055d,
    // end of x86_fmin.f64x2 (I32)
    // 000ae4: x86_movlhps.f64x2 (I32)
    // --> [Op2fa#416] and stop
    0x0179, 0x0416,
    // end of x86_movlhps.f64x2 (I32)
    // 000ae6: x86_movsd.f64x2 (I32)
    // --> [Mp2fa#710] and stop
    0x0139, 0x0710,
];

/// x86 level 2 hash tables.
///
/// This hash table, keyed by instruction opcode, contains all the starting offsets for the
/// encodings interpreter, for all the CPU modes. It is jumped to after a lookup on the
/// instruction's controlling type in the level 1 hash table.
pub static LEVEL2: [Level2Entry<u16>; 2262] = [
    // I64
    // 000000: i64, 128 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ushr), offset: 0x00019b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brz), offset: 0x000026 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brnz), offset: 0x000022 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddImm), offset: 0x00005c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::JumpTableEntry), offset: 0x0000ce },
    Level2Entry { opcode: Some(crate::ir::Opcode::JumpTableBase), offset: 0x0000cc },
    Level2Entry { opcode: Some(crate::ir::Opcode::IndirectJumpTableBr), offset: 0x00007e },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcin), offset: 0x000058 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcout), offset: 0x00005a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcarry), offset: 0x000056 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfbin), offset: 0x0000c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfbout), offset: 0x0000ca },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfborrow), offset: 0x0000c8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000002 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x00001c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x00002a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bnot), offset: 0x00001a },
    Level2Entry { opcode: Some(crate::ir::Opcode::FuncAddr), offset: 0x000046 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CallIndirect), offset: 0x000030 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandImm), offset: 0x000004 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BorImm), offset: 0x00001e },
    Level2Entry { opcode: Some(crate::ir::Opcode::BxorImm), offset: 0x00002c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Rotl), offset: 0x0000e6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0000d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x0000d6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RotlImm), offset: 0x0000e8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ishl), offset: 0x000082 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x000140 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RotrImm), offset: 0x0000ec },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x000084 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8), offset: 0x00011e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore8), offset: 0x0000ab },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore8Complex), offset: 0x0000b7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Clz), offset: 0x000034 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sshr), offset: 0x000134 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ctz), offset: 0x00003f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Popcnt), offset: 0x0000dd },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore16), offset: 0x000086 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore16Complex), offset: 0x000092 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16Complex), offset: 0x000103 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8Complex), offset: 0x000124 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16), offset: 0x0000fd },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload32), offset: 0x000111 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore32), offset: 0x00009f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Rotr), offset: 0x0000ea },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload32), offset: 0x000174 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SshrImm), offset: 0x000136 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8x8), offset: 0x00012b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16), offset: 0x000160 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16Complex), offset: 0x000166 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16x4), offset: 0x00016d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16x4), offset: 0x00010a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x00013a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload32x2), offset: 0x000180 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload32x2), offset: 0x000117 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8Complex), offset: 0x00018d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x00019d },
    Level2Entry { opcode: Some(crate::ir::Opcode::StackAddr), offset: 0x000138 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x000017 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SymbolValue), offset: 0x000147 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8), offset: 0x000187 },
    Level2Entry { opcode: Some(crate::ir::Opcode::GetPinnedReg), offset: 0x000052 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bint), offset: 0x000008 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SetPinnedReg), offset: 0x0000f0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x000066 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ConstAddr), offset: 0x000037 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8x8), offset: 0x000194 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uextend), offset: 0x000151 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sextend), offset: 0x0000f4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Selectif), offset: 0x0000ee },
    Level2Entry { opcode: Some(crate::ir::Opcode::SelectifSpectreGuard), offset: 0x0000ee },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000039 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000132 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x000042 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000044 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0000e2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x00003d },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::AdjustSpDown), offset: 0x000000 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IfcmpSp), offset: 0x00007a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0000e4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0000e0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Sdivmodx), offset: 0x0001b1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Smulx), offset: 0x0001b3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Cvtt2si), offset: 0x0001a3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Umulx), offset: 0x0001b7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Udivmodx), offset: 0x0001b5 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Push), offset: 0x0001ad },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x000060 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IcmpImm), offset: 0x000062 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ifcmp), offset: 0x000074 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IfcmpImm), offset: 0x000076 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x000054 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Bsf), offset: 0x00019f },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pop), offset: 0x0001a9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x0000c4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Bsr), offset: 0x0001a1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x00007c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 000080: i32, 128 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brz), offset: 0x0001e3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brnz), offset: 0x0001db },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddImm), offset: 0x00020b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcin), offset: 0x000207 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcout), offset: 0x000209 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcarry), offset: 0x000205 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfbin), offset: 0x000231 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfbout), offset: 0x000235 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfborrow), offset: 0x000233 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0001b9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0001d5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0001eb },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bnot), offset: 0x0001d3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandImm), offset: 0x0001bb },
    Level2Entry { opcode: Some(crate::ir::Opcode::BorImm), offset: 0x0001d7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BxorImm), offset: 0x0001ed },
    Level2Entry { opcode: Some(crate::ir::Opcode::Rotl), offset: 0x000253 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x000174 },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000237 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RotlImm), offset: 0x000257 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ishl), offset: 0x000229 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0002a7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RotrImm), offset: 0x00025d },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x00022d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8), offset: 0x000284 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore8), offset: 0x0000ab },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore8Complex), offset: 0x0000b7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Clz), offset: 0x0001f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sshr), offset: 0x0002a1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ctz), offset: 0x0001fa },
    Level2Entry { opcode: Some(crate::ir::Opcode::Popcnt), offset: 0x000244 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore16), offset: 0x000086 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore16Complex), offset: 0x000092 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16Complex), offset: 0x000277 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8Complex), offset: 0x000290 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16), offset: 0x00026b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8), offset: 0x0002d7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16), offset: 0x0002be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Rotr), offset: 0x000259 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8x8), offset: 0x000194 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SshrImm), offset: 0x0002a5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8x8), offset: 0x00012b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ushr), offset: 0x0002f0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16Complex), offset: 0x0002ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16x4), offset: 0x00016d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16x4), offset: 0x00010a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x00009f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload32x2), offset: 0x000180 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload32x2), offset: 0x000117 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8Complex), offset: 0x0002e3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x0002f4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x0001ce },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bint), offset: 0x0001bf },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x000215 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ireduce), offset: 0x000226 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uextend), offset: 0x0002b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sextend), offset: 0x000261 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Selectif), offset: 0x00025f },
    Level2Entry { opcode: Some(crate::ir::Opcode::SelectifSpectreGuard), offset: 0x00025f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0001f6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00029d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001ff },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000044 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00024d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0001f8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00024f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000249 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Sdivmodx), offset: 0x000304 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Smulx), offset: 0x000306 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Cvtt2si), offset: 0x0002fa },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Umulx), offset: 0x00030a },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Udivmodx), offset: 0x000308 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x00020f },
    Level2Entry { opcode: Some(crate::ir::Opcode::IcmpImm), offset: 0x000211 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ifcmp), offset: 0x00021e },
    Level2Entry { opcode: Some(crate::ir::Opcode::IfcmpImm), offset: 0x000220 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x000203 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Bsf), offset: 0x0002f6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x00022f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Bsr), offset: 0x0002f8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x000224 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 000100: r64, 32 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x0000e2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x00003d },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsNull), offset: 0x00030e },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsInvalid), offset: 0x00030c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0000d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x0000d6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x00013a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x0000e4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x0000e0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x000140 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Null), offset: 0x000310 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000039 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000132 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x000042 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000044 },
    Level2Entry { opcode: None, offset: 0 },
    // 000120: b1, 32 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000338 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0001f8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brz), offset: 0x00032c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brnz), offset: 0x000324 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00024f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000249 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x000318 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000314 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000320 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000334 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bnot), offset: 0x00031c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00015c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00029d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001ff },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000044 },
    Level2Entry { opcode: None, offset: 0 },
    // 000140: i8, 16 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000354 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ireduce), offset: 0x00034b },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0001f8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00029d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00024f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000249 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00015c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bint), offset: 0x00033c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001ff },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000044 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x000346 },
    // 000150: i16, 16 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00024d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ireduce), offset: 0x00034e },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0001f8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00029d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00024f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000249 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00015c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bint), offset: 0x00035a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0001ff },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000044 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x00006f },
    // 000160: b8, 4 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00024d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x000318 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 000164: b16, 4 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00024d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x000318 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 000168: b32, 8 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0001d5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x000318 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bnot), offset: 0x0001d3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00024d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0001eb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0001b9 },
    // 000170: r32, 8 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00024d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x000174 },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000237 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x00009f },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0002a7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000044 },
    Level2Entry { opcode: None, offset: 0 },
    // 000178: b64, 8 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x00001c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x000369 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bnot), offset: 0x00001a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x00002a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000002 },
    // 000180: typeless, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Jump), offset: 0x000399 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload32x2Complex), offset: 0x0003ee },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload32x2Complex), offset: 0x0003b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmullq), offset: 0x00040e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trueif), offset: 0x0003dc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brif), offset: 0x00037b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brff), offset: 0x000373 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmuludq), offset: 0x000411 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trueff), offset: 0x0003d8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86ElfTlsGetAddr), offset: 0x00040a },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86MachoTlsGetAddr), offset: 0x00040c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Debugtrap), offset: 0x00038b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trap), offset: 0x00039d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::F32const), offset: 0x00038f },
    Level2Entry { opcode: Some(crate::ir::Opcode::F64const), offset: 0x000394 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ResumableTrap), offset: 0x00039d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trapff), offset: 0x0003d4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Return), offset: 0x00039f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trapif), offset: 0x0003d6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Call), offset: 0x000383 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::DummySargT), offset: 0x00038d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopySpecial), offset: 0x000389 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::AdjustSpUpImm), offset: 0x00036f },
    Level2Entry { opcode: Some(crate::ir::Opcode::AdjustSpDownImm), offset: 0x00036b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Safepoint), offset: 0x0003a1 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Vcvtudq2ps), offset: 0x000413 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload32Complex), offset: 0x000237 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload32Complex), offset: 0x0003b1 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore32Complex), offset: 0x0002a7 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8x8Complex), offset: 0x0003fc },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8x8Complex), offset: 0x0003c6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16x4Complex), offset: 0x0003e0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16x4Complex), offset: 0x0003a3 },
    // 0001c0: f64, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Floor), offset: 0x000429 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trunc), offset: 0x000429 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Nearest), offset: 0x000429 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x00041e },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000475 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000416 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000421 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000425 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x00041a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00042e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00044a },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::FcvtFromSint), offset: 0x00043c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000497 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000432 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000469 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00045d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00049d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fpromote), offset: 0x000454 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000499 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000493 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0004a5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0004b1 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmin), offset: 0x0004c1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmax), offset: 0x0004bd },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fcmp), offset: 0x000438 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ffcmp), offset: 0x000446 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fadd), offset: 0x000434 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fsub), offset: 0x000459 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmul), offset: 0x000450 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdiv), offset: 0x000442 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sqrt), offset: 0x0004a1 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ceil), offset: 0x000429 },
    // 000200: f32, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Floor), offset: 0x0004ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trunc), offset: 0x0004ca },
    Level2Entry { opcode: Some(crate::ir::Opcode::Nearest), offset: 0x0004ca },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x0004c5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000475 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000416 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000421 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000425 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdemote), offset: 0x0004df },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x00041a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00042e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004ec },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::FcvtFromSint), offset: 0x0004d9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000497 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x0004cf },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000504 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0004f8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000518 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x000520 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000514 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000510 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x00052c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmin), offset: 0x00053c },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmax), offset: 0x000538 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fcmp), offset: 0x0004d5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ffcmp), offset: 0x0004e8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fadd), offset: 0x0004d1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fsub), offset: 0x0004f4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmul), offset: 0x0004f0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdiv), offset: 0x0004e4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sqrt), offset: 0x00051c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ceil), offset: 0x0004ca },
    // 000240: b8x16, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0005bb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000562 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x00058f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0005a7 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000540 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000544 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000546 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000542 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00042e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00054c },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000591 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000587 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000548 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000556 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x000550 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x00059b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x000595 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00058b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000583 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vselect), offset: 0x0005af },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0005b8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x0005b2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x0005b5 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckh), offset: 0x0005be },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckl), offset: 0x0005c0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 000280: b16x8, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0005bb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0005c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x00058f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0005a7 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000540 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000544 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000546 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000542 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00042e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00054c },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000591 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000587 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000548 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000556 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x000550 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x00059b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x000595 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00058b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000583 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vselect), offset: 0x0005af },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0005b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pblendw), offset: 0x0005e3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x0005e6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x0005e9 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckh), offset: 0x0005eb },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckl), offset: 0x0005ed },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 0002c0: b32x4, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0005bb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0005ef },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x00058f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0005a7 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000540 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000544 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000546 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000542 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00042e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00054c },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000591 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000587 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000548 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000556 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x000550 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x00059b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x000595 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00058b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000583 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vselect), offset: 0x000610 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufd), offset: 0x000619 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0005b8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000613 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000616 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckh), offset: 0x00061b },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckl), offset: 0x00061d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 000300: b64x2, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0005bb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x00061f },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x00041f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0005a7 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000540 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000544 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000546 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000542 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00042e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00054c },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000591 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000587 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000548 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000556 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x000550 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x00059b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x000595 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00058b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000583 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vselect), offset: 0x000640 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0005b8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000643 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000646 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckh), offset: 0x000649 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckl), offset: 0x00064b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 000340: i8x16, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckh), offset: 0x0005be },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0005bb },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxs), offset: 0x00068e },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxu), offset: 0x000691 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmins), offset: 0x000693 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pminu), offset: 0x000696 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x00065c },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x00058f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Palignr), offset: 0x00068b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SwidenLow), offset: 0x000681 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0005a7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UwidenLow), offset: 0x000688 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000540 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000544 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000546 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000542 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00042e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00054c },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000591 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000587 },
    Level2Entry { opcode: Some(crate::ir::Opcode::AvgRound), offset: 0x00064d },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000548 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000556 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x000550 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x00059b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x000595 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00058b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000583 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vselect), offset: 0x0005af },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x000654 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x000652 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UaddSat), offset: 0x000684 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SaddSat), offset: 0x00067d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x00065a },
    Level2Entry { opcode: Some(crate::ir::Opcode::UsubSat), offset: 0x000686 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SsubSat), offset: 0x00067f },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x0005b5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iabs), offset: 0x00064f },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x0005b2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckl), offset: 0x0005c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0005b8 },
    // 000380: i16x8, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pblendw), offset: 0x0005e3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x0005e6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psrl), offset: 0x0006f0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psra), offset: 0x0006ee },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxs), offset: 0x0006e2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxu), offset: 0x0006e4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmins), offset: 0x0006e7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pminu), offset: 0x0006e9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0006ab },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x00058f },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckl), offset: 0x0005ed },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Palignr), offset: 0x00068b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0005bb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Snarrow), offset: 0x0006ce },
    Level2Entry { opcode: Some(crate::ir::Opcode::Unarrow), offset: 0x0006d9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SwidenLow), offset: 0x0006d4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0005a7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UwidenLow), offset: 0x0006df },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckh), offset: 0x0005eb },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000540 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000544 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000546 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000542 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00042e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00054c },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000591 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000587 },
    Level2Entry { opcode: Some(crate::ir::Opcode::AvgRound), offset: 0x000698 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000548 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000556 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x000550 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x00059b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x000595 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x0006a7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000583 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SshrImm), offset: 0x0006d0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00058b },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x0006db },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vselect), offset: 0x0005af },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x00069f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x00069d },
    Level2Entry { opcode: Some(crate::ir::Opcode::UaddSat), offset: 0x0006d7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SaddSat), offset: 0x0006cc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x0006a9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UsubSat), offset: 0x0006dd },
    Level2Entry { opcode: Some(crate::ir::Opcode::SsubSat), offset: 0x0006d2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x0005e9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iabs), offset: 0x00069a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x0006a5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psll), offset: 0x0006ec },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0005b8 },
    // 0003c0: i32x4, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psra), offset: 0x00073f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0005bb },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxs), offset: 0x000731 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxu), offset: 0x000734 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmins), offset: 0x000737 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pminu), offset: 0x00073a },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000704 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x00058f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Palignr), offset: 0x00068b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Snarrow), offset: 0x000725 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Unarrow), offset: 0x000729 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0005a7 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000540 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000544 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000546 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000542 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00042e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00054c },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000591 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000587 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000548 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000556 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x000550 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x00059b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x000595 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x000700 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000583 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SshrImm), offset: 0x000727 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00058b },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x00072c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vselect), offset: 0x000610 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Cvtt2si), offset: 0x00072e },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x0006f7 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufd), offset: 0x000619 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x0006f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0005b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000613 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x000702 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000616 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckh), offset: 0x00061b },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckl), offset: 0x00061d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iabs), offset: 0x0006f2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x0006fd },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psll), offset: 0x00073d },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psrl), offset: 0x000741 },
    // 000400: i64x2, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0005bb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x000743 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000757 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x00041f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Palignr), offset: 0x00068b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0005a7 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000540 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000544 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000546 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000542 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00042e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00054c },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000591 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000587 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000548 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000556 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x000550 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x00059b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x000595 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x000753 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000583 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x000778 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00058b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vselect), offset: 0x000640 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x00074b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x000749 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0005b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000643 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x000755 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000646 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckh), offset: 0x000649 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckl), offset: 0x00064b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psll), offset: 0x00077a },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psrl), offset: 0x00077c },
    // 000440: f32x4, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0005bb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x00078d },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000491 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0005a7 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000540 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000544 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000546 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000542 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00042e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00054c },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::FcvtFromSint), offset: 0x000782 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000587 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000548 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000556 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x000550 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000591 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x000595 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00058b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000583 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x00059b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vselect), offset: 0x000610 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmin), offset: 0x0007b2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmax), offset: 0x0007b0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fcmp), offset: 0x000780 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fadd), offset: 0x00077e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fsub), offset: 0x00078b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmul), offset: 0x000789 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdiv), offset: 0x000787 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sqrt), offset: 0x0007ae },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000613 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000616 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Insertps), offset: 0x0007b4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufd), offset: 0x000619 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0005b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckl), offset: 0x00061d },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckh), offset: 0x00061b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 000480: f64x2, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0005bb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0007c1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000491 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0005a7 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000540 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000544 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000546 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x000542 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00042e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00054c },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000591 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000587 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000548 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000556 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x000550 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x00059b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x000595 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00058b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000583 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vselect), offset: 0x000640 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmin), offset: 0x0007e6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmax), offset: 0x0007e4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fcmp), offset: 0x0007b9 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fadd), offset: 0x0007b7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fsub), offset: 0x0007bf },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmul), offset: 0x0007bd },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdiv), offset: 0x0007bb },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sqrt), offset: 0x0007e2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000643 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000646 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckh), offset: 0x000649 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0005b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Movsd), offset: 0x0007ea },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Movlhps), offset: 0x0007e8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckl), offset: 0x00064b },
    Level2Entry { opcode: None, offset: 0 },
    // I32
    // 0004c0: i32, 128 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brz), offset: 0x0007fe },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brnz), offset: 0x0007fa },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddImm), offset: 0x00020b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::JumpTableEntry), offset: 0x00083e },
    Level2Entry { opcode: Some(crate::ir::Opcode::JumpTableBase), offset: 0x00083c },
    Level2Entry { opcode: Some(crate::ir::Opcode::IndirectJumpTableBr), offset: 0x000080 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcin), offset: 0x000207 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcout), offset: 0x000209 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IaddIfcarry), offset: 0x000205 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfbin), offset: 0x000231 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfbout), offset: 0x000235 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsubIfborrow), offset: 0x000233 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0001b9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0001d5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0001eb },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bnot), offset: 0x0001d3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FuncAddr), offset: 0x00080c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CallIndirect), offset: 0x000032 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandImm), offset: 0x0001bb },
    Level2Entry { opcode: Some(crate::ir::Opcode::BorImm), offset: 0x0001d7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BxorImm), offset: 0x0001ed },
    Level2Entry { opcode: Some(crate::ir::Opcode::Rotl), offset: 0x000850 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x000840 },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000846 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RotlImm), offset: 0x000257 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ishl), offset: 0x000819 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0008a8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RotrImm), offset: 0x00025d },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x00022d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8), offset: 0x000883 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore8), offset: 0x00082f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore8Complex), offset: 0x000835 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Clz), offset: 0x000802 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sshr), offset: 0x000897 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ctz), offset: 0x000809 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Popcnt), offset: 0x00084d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore16), offset: 0x000822 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Istore16Complex), offset: 0x000828 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16Complex), offset: 0x00086e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8Complex), offset: 0x000889 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16), offset: 0x000868 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8), offset: 0x0008d3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16), offset: 0x0008b8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Rotr), offset: 0x000859 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8x8), offset: 0x0008e0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SshrImm), offset: 0x0002a5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8x8), offset: 0x000890 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ushr), offset: 0x0008e7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16Complex), offset: 0x0008be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16x4), offset: 0x0008c5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16x4), offset: 0x000875 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0008a2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload32x2), offset: 0x0008cc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload32x2), offset: 0x00087c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8Complex), offset: 0x0008d9 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x0002f4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StackAddr), offset: 0x0008a0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x0007f7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SymbolValue), offset: 0x0008af },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bint), offset: 0x0007ee },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x000812 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ConstAddr), offset: 0x000805 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uextend), offset: 0x0008b2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sextend), offset: 0x000862 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Selectif), offset: 0x00025f },
    Level2Entry { opcode: Some(crate::ir::Opcode::SelectifSpectreGuard), offset: 0x00025f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x0001f6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00029f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x000201 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000044 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00033a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000807 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::AdjustSpDown), offset: 0x0007ec },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IfcmpSp), offset: 0x000817 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000251 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00024b },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Sdivmodx), offset: 0x000304 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Smulx), offset: 0x000306 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Cvtt2si), offset: 0x0008f0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Umulx), offset: 0x00030a },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Udivmodx), offset: 0x000308 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Push), offset: 0x0001af },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x00020f },
    Level2Entry { opcode: Some(crate::ir::Opcode::IcmpImm), offset: 0x000211 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ifcmp), offset: 0x00021e },
    Level2Entry { opcode: Some(crate::ir::Opcode::IfcmpImm), offset: 0x000220 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x000203 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Bsf), offset: 0x0002f6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pop), offset: 0x0001ab },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x00022f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Bsr), offset: 0x0002f8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x000224 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 000540: r32, 32 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00033a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000807 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsNull), offset: 0x0008f8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::IsInvalid), offset: 0x0008f6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x000840 },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000846 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0008a2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000251 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00024b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0008a8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Null), offset: 0x000312 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00015e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00029f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x000201 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000044 },
    Level2Entry { opcode: None, offset: 0 },
    // 000560: b1, 32 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00033a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000807 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brz), offset: 0x000900 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brnz), offset: 0x0008fa },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000251 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00024b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x00031a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000316 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000322 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000336 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bnot), offset: 0x00031e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00015e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00029f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x000201 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000044 },
    Level2Entry { opcode: None, offset: 0 },
    // 000580: i8, 16 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000915 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ireduce), offset: 0x00090f },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000807 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00029f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000251 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00024b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00015e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bint), offset: 0x000906 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x000201 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000044 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x00090c },
    // 000590: i16, 16 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00033a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ireduce), offset: 0x000912 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000807 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00029f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000251 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x00024b },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x00015e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bint), offset: 0x000919 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x000201 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000044 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iconst), offset: 0x000814 },
    // 0005a0: b8, 4 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00033a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x00031a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 0005a4: b16, 4 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00033a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x00031a },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 0005a8: b32, 8 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0001d5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bconst), offset: 0x00031a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bnot), offset: 0x0001d3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x00033a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0001eb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0001b9 },
    // 0005b0: i64, 4 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Bint), offset: 0x000922 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000044 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    // 0005b4: typeless, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Jump), offset: 0x000399 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload32x2Complex), offset: 0x00095f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload32x2Complex), offset: 0x000947 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmullq), offset: 0x00040e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trueif), offset: 0x0003de },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brif), offset: 0x000931 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Brff), offset: 0x00092d },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmuludq), offset: 0x00096f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trueff), offset: 0x0003da },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Debugtrap), offset: 0x00038b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trap), offset: 0x00039d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::F32const), offset: 0x000939 },
    Level2Entry { opcode: Some(crate::ir::Opcode::F64const), offset: 0x00093c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ResumableTrap), offset: 0x00039d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trapff), offset: 0x0003d4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Return), offset: 0x00039f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trapif), offset: 0x0003d6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Call), offset: 0x000935 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::DummySargT), offset: 0x00038d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopySpecial), offset: 0x000937 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::AdjustSpUpImm), offset: 0x000929 },
    Level2Entry { opcode: Some(crate::ir::Opcode::AdjustSpDownImm), offset: 0x000925 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Safepoint), offset: 0x0003a1 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Vcvtudq2ps), offset: 0x000413 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload8x8Complex), offset: 0x000967 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload8x8Complex), offset: 0x00094f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Uload16x4Complex), offset: 0x000957 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sload16x4Complex), offset: 0x00093f },
    // 0005f4: f64, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Floor), offset: 0x000971 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trunc), offset: 0x000971 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Nearest), offset: 0x000971 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000475 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000418 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000423 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000427 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x00041c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000430 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00044c },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::FcvtFromSint), offset: 0x000976 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000589 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x000974 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x000982 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x00097c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00049f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fpromote), offset: 0x000979 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00049b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000495 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x000988 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x00098e },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmin), offset: 0x0004c3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmax), offset: 0x0004bf },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fcmp), offset: 0x00043a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ffcmp), offset: 0x000448 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fadd), offset: 0x000436 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fsub), offset: 0x00045b },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmul), offset: 0x000452 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdiv), offset: 0x000444 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sqrt), offset: 0x0004a3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ceil), offset: 0x000971 },
    // 000634: f32, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Floor), offset: 0x000997 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Trunc), offset: 0x000997 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Nearest), offset: 0x000997 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x000994 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000475 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x000418 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x000423 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x000427 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdemote), offset: 0x00099f },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x00041c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000430 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x0004ee },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::FcvtFromSint), offset: 0x00099c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000589 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x00099a },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x0009a8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0009a2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x00051a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0009ae },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x000516 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000512 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0009b4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmin), offset: 0x00053e },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmax), offset: 0x00053a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fcmp), offset: 0x0004d7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ffcmp), offset: 0x0004ea },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fadd), offset: 0x0004d3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fsub), offset: 0x0004f6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmul), offset: 0x0004f2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdiv), offset: 0x0004e6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sqrt), offset: 0x00051e },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Ceil), offset: 0x000997 },
    // 000674: r64, 2 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x000044 },
    Level2Entry { opcode: None, offset: 0 },
    // 000676: b8x16, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0009ee },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000562 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x0004c8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0009da },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0009ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0009be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0009c0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x0009bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000430 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00054e },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000593 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000589 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x00054a },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x0009c8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0009c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0009d4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0009ce },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00058d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000585 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vselect), offset: 0x0009e2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0009eb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x0009e5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x0009e8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckh), offset: 0x0009f1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckl), offset: 0x0009f3 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 0006b6: b16x8, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0009ee },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0005c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x0004c8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0009da },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0009ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0009be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0009c0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x0009bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000430 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00054e },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000593 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000589 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x00054a },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x0009c8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0009c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0009d4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0009ce },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00058d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000585 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vselect), offset: 0x0009e2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0009eb },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pblendw), offset: 0x0009f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x0009f8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x0009fb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckh), offset: 0x0009fd },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckl), offset: 0x0009ff },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 0006f6: b32x4, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0009ee },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0005ef },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x0004c8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0009da },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0009ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0009be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0009c0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x0009bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000430 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00054e },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000593 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000589 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x00054a },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x0009c8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0009c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0009d4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0009ce },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00058d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000585 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vselect), offset: 0x000a01 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufd), offset: 0x000a0a },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0009eb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000a04 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000a07 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckh), offset: 0x000a0c },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckl), offset: 0x000a0e },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 000736: b64x2, 32 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000589 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckl), offset: 0x000a15 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x00054a },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x0009c8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0009c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0009d4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0009ce },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x00061f },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000585 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00058d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vselect), offset: 0x000a10 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0009ee },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0009da },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0009eb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0009ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0009be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0009c0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckh), offset: 0x000a13 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x0009bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000430 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00054e },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000593 },
    // 000756: i8x16, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckh), offset: 0x0009f1 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0009ee },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxs), offset: 0x000a37 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxu), offset: 0x000a3a },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmins), offset: 0x000a3c },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pminu), offset: 0x000a3f },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x00065c },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x0004c8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Palignr), offset: 0x000a34 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SwidenLow), offset: 0x000a2a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0009da },
    Level2Entry { opcode: Some(crate::ir::Opcode::UwidenLow), offset: 0x000a31 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0009ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0009be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0009c0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x0009bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000430 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00054e },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000593 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000589 },
    Level2Entry { opcode: Some(crate::ir::Opcode::AvgRound), offset: 0x000a17 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x00054a },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x0009c8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0009c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0009d4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0009ce },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00058d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000585 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vselect), offset: 0x0009e2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x000a1e },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x000a1c },
    Level2Entry { opcode: Some(crate::ir::Opcode::UaddSat), offset: 0x000a2d },
    Level2Entry { opcode: Some(crate::ir::Opcode::SaddSat), offset: 0x000a26 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x000a24 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UsubSat), offset: 0x000a2f },
    Level2Entry { opcode: Some(crate::ir::Opcode::SsubSat), offset: 0x000a28 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x0009e8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iabs), offset: 0x000a19 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x0009e5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckl), offset: 0x0009f3 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0009eb },
    // 000796: i16x8, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pblendw), offset: 0x0009f5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x0009f8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psrl), offset: 0x000a78 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psra), offset: 0x000a76 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxs), offset: 0x000a6a },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxu), offset: 0x000a6c },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmins), offset: 0x000a6f },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pminu), offset: 0x000a71 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0006ab },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x0004c8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckl), offset: 0x0009ff },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Palignr), offset: 0x000a34 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0009ee },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Snarrow), offset: 0x000a56 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Unarrow), offset: 0x000a61 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SwidenLow), offset: 0x000a5c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0009da },
    Level2Entry { opcode: Some(crate::ir::Opcode::UwidenLow), offset: 0x000a67 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckh), offset: 0x0009fd },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0009ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0009be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0009c0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x0009bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000430 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00054e },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000593 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000589 },
    Level2Entry { opcode: Some(crate::ir::Opcode::AvgRound), offset: 0x000a41 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x00054a },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x0009c8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0009c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0009d4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0009ce },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x000a50 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000585 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SshrImm), offset: 0x000a58 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00058d },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x000a63 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vselect), offset: 0x0009e2 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x000a48 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x000a46 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UaddSat), offset: 0x000a5f },
    Level2Entry { opcode: Some(crate::ir::Opcode::SaddSat), offset: 0x000a54 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x000a52 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UsubSat), offset: 0x000a65 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SsubSat), offset: 0x000a5a },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x0009fb },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iabs), offset: 0x000a43 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x000a4e },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psll), offset: 0x000a74 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0009eb },
    // 0007d6: i32x4, 64 entries
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psra), offset: 0x000aa6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0009ee },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxs), offset: 0x000a98 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmaxu), offset: 0x000a9b },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pmins), offset: 0x000a9e },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pminu), offset: 0x000aa1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000704 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x0004c8 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Palignr), offset: 0x000a34 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Snarrow), offset: 0x000a8c },
    Level2Entry { opcode: Some(crate::ir::Opcode::Unarrow), offset: 0x000a90 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0009da },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0009ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0009be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0009c0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x0009bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000430 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00054e },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000593 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000589 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x00054a },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x0009c8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0009c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0009d4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0009ce },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x000a88 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000585 },
    Level2Entry { opcode: Some(crate::ir::Opcode::SshrImm), offset: 0x000a8e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00058d },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x000a93 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vselect), offset: 0x000a01 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Cvtt2si), offset: 0x000a95 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x000a7f },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufd), offset: 0x000a0a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x000a7d },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0009eb },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000a04 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x000a8a },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000a07 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckh), offset: 0x000a0c },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckl), offset: 0x000a0e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iabs), offset: 0x000a7a },
    Level2Entry { opcode: Some(crate::ir::Opcode::Imul), offset: 0x000a85 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psll), offset: 0x000aa4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psrl), offset: 0x000aa8 },
    // 000816: i64x2, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0009ee },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bitcast), offset: 0x000994 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x000757 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Palignr), offset: 0x000a34 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0009da },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0009ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0009be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0009c0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x0009bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000430 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00054e },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000593 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000589 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x00054a },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x0009c8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0009c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0009d4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0009ce },
    Level2Entry { opcode: Some(crate::ir::Opcode::IshlImm), offset: 0x000ab4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000585 },
    Level2Entry { opcode: Some(crate::ir::Opcode::UshrImm), offset: 0x000ab8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00058d },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vselect), offset: 0x000a10 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Icmp), offset: 0x000aac },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Iadd), offset: 0x000aaa },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0009eb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Isub), offset: 0x000ab6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckh), offset: 0x000a13 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckl), offset: 0x000a15 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psll), offset: 0x000aba },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Psrl), offset: 0x000abc },
    // 000856: f32x4, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0009ee },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x00078d },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000491 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0009da },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0009ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0009be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0009c0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x0009bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000430 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00054e },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::FcvtFromSint), offset: 0x000ac2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000589 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x00054a },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x0009c8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0009c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000593 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0009ce },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00058d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000585 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0009d4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vselect), offset: 0x000a01 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmin), offset: 0x000acf },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmax), offset: 0x000acd },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fcmp), offset: 0x000ac0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fadd), offset: 0x000abe },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fsub), offset: 0x000ac9 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmul), offset: 0x000ac7 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdiv), offset: 0x000ac5 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sqrt), offset: 0x000acb },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pextr), offset: 0x000a04 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pinsr), offset: 0x000a07 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Insertps), offset: 0x000ad1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufd), offset: 0x000a0a },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0009eb },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckl), offset: 0x000a0e },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckh), offset: 0x000a0c },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    // 000896: f64x2, 64 entries
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Ptest), offset: 0x0009ee },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::RawBitcast), offset: 0x0007c1 },
    Level2Entry { opcode: Some(crate::ir::Opcode::ScalarToVector), offset: 0x000491 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vconst), offset: 0x0009da },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Band), offset: 0x0009ba },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bor), offset: 0x0009be },
    Level2Entry { opcode: Some(crate::ir::Opcode::Bxor), offset: 0x0009c0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::BandNot), offset: 0x0009bc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Copy), offset: 0x000430 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fill), offset: 0x00054e },
    Level2Entry { opcode: Some(crate::ir::Opcode::FillNop), offset: 0x00044e },
    Level2Entry { opcode: Some(crate::ir::Opcode::Spill), offset: 0x000593 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regmove), offset: 0x000589 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyToSsa), offset: 0x00054a },
    Level2Entry { opcode: Some(crate::ir::Opcode::CopyNop), offset: 0x00003b },
    Level2Entry { opcode: Some(crate::ir::Opcode::LoadComplex), offset: 0x0009c8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Load), offset: 0x0009c2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::StoreComplex), offset: 0x0009d4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Store), offset: 0x0009ce },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regspill), offset: 0x00058d },
    Level2Entry { opcode: Some(crate::ir::Opcode::Regfill), offset: 0x000585 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Vselect), offset: 0x000a10 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmin), offset: 0x000ae2 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Fmax), offset: 0x000ae0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fcmp), offset: 0x000ad6 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fadd), offset: 0x000ad4 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fsub), offset: 0x000adc },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fmul), offset: 0x000ada },
    Level2Entry { opcode: Some(crate::ir::Opcode::Fdiv), offset: 0x000ad8 },
    Level2Entry { opcode: Some(crate::ir::Opcode::Sqrt), offset: 0x000ade },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Pshufb), offset: 0x0009eb },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckh), offset: 0x000a13 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Punpckl), offset: 0x000a15 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Movsd), offset: 0x000ae6 },
    Level2Entry { opcode: Some(crate::ir::Opcode::X86Movlhps), offset: 0x000ae4 },
    Level2Entry { opcode: None, offset: 0 },
    Level2Entry { opcode: None, offset: 0 },
];

/// x86 level 1 hash table for the CPU mode I64.
///
/// This hash table, keyed by instruction controlling type, contains all the level 2
/// hash-tables offsets for the given CPU mode, as well as a legalization identifier indicating
/// which legalization scheme to apply when the instruction doesn't have any valid encoding for
/// this CPU mode.
pub static LEVEL1_I64: [Level1Entry<u16>; 32] = [
    Level1Entry { ty: ir::types::INVALID, log2len: 6, offset: 0x000180, legalize: 0 }, // expand_flags
    Level1Entry { ty: ir::types::F32X4, log2len: 6, offset: 0x000440, legalize: 3 }, // x86_narrow_avx
    Level1Entry { ty: ir::types::B16X8, log2len: 6, offset: 0x000280, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::B64X2, log2len: 6, offset: 0x000300, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::I8X16, log2len: 6, offset: 0x000340, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::B8X16, log2len: 6, offset: 0x000240, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::I16X8, log2len: 6, offset: 0x000380, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::I64X2, log2len: 6, offset: 0x000400, legalize: 3 }, // x86_narrow_avx
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::F64X2, log2len: 6, offset: 0x000480, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::I32X4, log2len: 6, offset: 0x0003c0, legalize: 3 }, // x86_narrow_avx
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::B1, log2len: 5, offset: 0x000120, legalize: 0 }, // expand_flags
    Level1Entry { ty: ir::types::B8, log2len: 2, offset: 0x000160, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::B16, log2len: 2, offset: 0x000164, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::B32, log2len: 3, offset: 0x000168, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::B64, log2len: 3, offset: 0x000178, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::I8, log2len: 4, offset: 0x000140, legalize: 4 }, // x86_widen
    Level1Entry { ty: ir::types::I16, log2len: 4, offset: 0x000150, legalize: 4 }, // x86_widen
    Level1Entry { ty: ir::types::I32, log2len: 7, offset: 0x000080, legalize: 1 }, // x86_expand
    Level1Entry { ty: ir::types::I64, log2len: 7, offset: 0x000000, legalize: 1 }, // x86_expand
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::F32, log2len: 6, offset: 0x000200, legalize: 1 }, // x86_expand
    Level1Entry { ty: ir::types::F64, log2len: 6, offset: 0x0001c0, legalize: 1 }, // x86_expand
    Level1Entry { ty: ir::types::B32X4, log2len: 6, offset: 0x0002c0, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::R32, log2len: 3, offset: 0x000170, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::R64, log2len: 5, offset: 0x000100, legalize: 1 }, // x86_expand
];

/// x86 level 1 hash table for the CPU mode I32.
///
/// This hash table, keyed by instruction controlling type, contains all the level 2
/// hash-tables offsets for the given CPU mode, as well as a legalization identifier indicating
/// which legalization scheme to apply when the instruction doesn't have any valid encoding for
/// this CPU mode.
pub static LEVEL1_I32: [Level1Entry<u16>; 32] = [
    Level1Entry { ty: ir::types::INVALID, log2len: 6, offset: 0x0005b4, legalize: 0 }, // expand_flags
    Level1Entry { ty: ir::types::F32X4, log2len: 6, offset: 0x000856, legalize: 3 }, // x86_narrow_avx
    Level1Entry { ty: ir::types::B16X8, log2len: 6, offset: 0x0006b6, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::B64X2, log2len: 5, offset: 0x000736, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::I8X16, log2len: 6, offset: 0x000756, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::I16X8, log2len: 6, offset: 0x000796, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::I64X2, log2len: 6, offset: 0x000816, legalize: 3 }, // x86_narrow_avx
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::F64X2, log2len: 6, offset: 0x000896, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::I32X4, log2len: 6, offset: 0x0007d6, legalize: 3 }, // x86_narrow_avx
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::B1, log2len: 5, offset: 0x000560, legalize: 0 }, // expand_flags
    Level1Entry { ty: ir::types::B8, log2len: 2, offset: 0x0005a0, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::B16, log2len: 2, offset: 0x0005a4, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::B32, log2len: 3, offset: 0x0005a8, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::B8X16, log2len: 6, offset: 0x000676, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::I8, log2len: 4, offset: 0x000580, legalize: 4 }, // x86_widen
    Level1Entry { ty: ir::types::I16, log2len: 4, offset: 0x000590, legalize: 4 }, // x86_widen
    Level1Entry { ty: ir::types::I32, log2len: 7, offset: 0x0004c0, legalize: 1 }, // x86_expand
    Level1Entry { ty: ir::types::I64, log2len: 2, offset: 0x0005b0, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::INVALID, log2len: !0, offset: 0, legalize: 2 },
    Level1Entry { ty: ir::types::F32, log2len: 6, offset: 0x000634, legalize: 1 }, // x86_expand
    Level1Entry { ty: ir::types::F64, log2len: 6, offset: 0x0005f4, legalize: 1 }, // x86_expand
    Level1Entry { ty: ir::types::B32X4, log2len: 6, offset: 0x0006f6, legalize: 2 }, // x86_narrow
    Level1Entry { ty: ir::types::R32, log2len: 5, offset: 0x000540, legalize: 1 }, // x86_expand
    Level1Entry { ty: ir::types::R64, log2len: 1, offset: 0x000674, legalize: 2 }, // x86_narrow
];

/// x86 recipe names, using the same recipe index spaces as the one specified by the
/// corresponding binemit file.
static RECIPE_NAMES: [&str; 363] = [
    "get_pinned_reg",
    "RexOp1set_pinned_reg",
    "DynRexOp1umr",
    "RexOp1umr",
    "Op1umr",
    "Op1rmov",
    "RexOp1rmov",
    "Op1pu_id",
    "RexOp1pu_id",
    "RexOp1u_id",
    "RexOp1pu_iq",
    "Op1pu_id_bool",
    "RexOp1pu_id_bool",
    "Op1u_id_z",
    "RexOp1u_id_z",
    "null",
    "Op2urm_noflags_abcd",
    "RexOp2urm_noflags",
    "Op2urm_noflags",
    "RexOp1urm_noflags",
    "RexOp1copysp",
    "Op1copysp",
    "Op1umr_reg_to_ssa",
    "RexOp1umr_reg_to_ssa",
    "Mp2furm_reg_to_ssa",
    "RexMp2furm_reg_to_ssa",
    "dummy_sarg_t",
    "Op1ldWithIndex",
    "RexOp1ldWithIndex",
    "Op2ldWithIndex",
    "RexOp2ldWithIndex",
    "Op1ldWithIndexDisp8",
    "RexOp1ldWithIndexDisp8",
    "Op2ldWithIndexDisp8",
    "RexOp2ldWithIndexDisp8",
    "Op1ldWithIndexDisp32",
    "RexOp1ldWithIndexDisp32",
    "Op2ldWithIndexDisp32",
    "RexOp2ldWithIndexDisp32",
    "Op1stWithIndex",
    "RexOp1stWithIndex",
    "Mp1stWithIndex",
    "RexMp1stWithIndex",
    "Op1stWithIndexDisp8",
    "RexOp1stWithIndexDisp8",
    "Mp1stWithIndexDisp8",
    "RexMp1stWithIndexDisp8",
    "Op1stWithIndexDisp32",
    "RexOp1stWithIndexDisp32",
    "Mp1stWithIndexDisp32",
    "RexMp1stWithIndexDisp32",
    "Op1stWithIndex_abcd",
    "RexOp1stWithIndex_abcd",
    "Op1stWithIndexDisp8_abcd",
    "RexOp1stWithIndexDisp8_abcd",
    "Op1stWithIndexDisp32_abcd",
    "RexOp1stWithIndexDisp32_abcd",
    "Op1st",
    "RexOp1st",
    "Mp1st",
    "RexMp1st",
    "Op1stDisp8",
    "RexOp1stDisp8",
    "Mp1stDisp8",
    "RexMp1stDisp8",
    "Op1stDisp32",
    "RexOp1stDisp32",
    "Mp1stDisp32",
    "RexMp1stDisp32",
    "Op1st_abcd",
    "Op1stDisp8_abcd",
    "Op1stDisp32_abcd",
    "Op1spillSib32",
    "RexOp1spillSib32",
    "Op1regspill32",
    "RexOp1regspill32",
    "Op1ld",
    "RexOp1ld",
    "Op2ld",
    "RexOp2ld",
    "Op1ldDisp8",
    "RexOp1ldDisp8",
    "Op2ldDisp8",
    "RexOp2ldDisp8",
    "Op1ldDisp32",
    "RexOp1ldDisp32",
    "Op2ldDisp32",
    "RexOp2ldDisp32",
    "Op1fillSib32",
    "RexOp1fillSib32",
    "Op1regfill32",
    "RexOp1regfill32",
    "fillnull",
    "ffillnull",
    "Op1pushq",
    "RexOp1pushq",
    "Op1popq",
    "RexOp1popq",
    "stacknull",
    "Op1adjustsp",
    "RexOp1adjustsp",
    "Op1adjustsp_ib",
    "Op1adjustsp_id",
    "RexOp1adjustsp_ib",
    "RexOp1adjustsp_id",
    "Mp2frurm",
    "RexMp2frurm",
    "Mp2rfumr",
    "RexMp2rfumr",
    "Op2furm",
    "RexOp2furm",
    "Op2frmov",
    "RexOp2frmov",
    "Mp2fld",
    "RexMp2fld",
    "Mp2fldDisp8",
    "RexMp2fldDisp8",
    "Mp2fldDisp32",
    "RexMp2fldDisp32",
    "Mp2fldWithIndex",
    "RexMp2fldWithIndex",
    "Mp2fldWithIndexDisp8",
    "RexMp2fldWithIndexDisp8",
    "Mp2fldWithIndexDisp32",
    "RexMp2fldWithIndexDisp32",
    "Mp2fst",
    "RexMp2fst",
    "Mp2fstDisp8",
    "RexMp2fstDisp8",
    "Mp2fstDisp32",
    "RexMp2fstDisp32",
    "Mp2fstWithIndex",
    "RexMp2fstWithIndex",
    "Mp2fstWithIndexDisp8",
    "RexMp2fstWithIndexDisp8",
    "Mp2fstWithIndexDisp32",
    "RexMp2fstWithIndexDisp32",
    "Mp2ffillSib32",
    "RexMp2ffillSib32",
    "Mp2fregfill32",
    "RexMp2fregfill32",
    "Mp2fspillSib32",
    "RexMp2fspillSib32",
    "Mp2fregspill32",
    "RexMp2fregspill32",
    "Op2f32imm_z",
    "Mp2f64imm_z",
    "RexOp2f32imm_z",
    "RexMp2f64imm_z",
    "DynRexMp2frurm",
    "Mp2furm",
    "RexMp2furm",
    "Mp2rfurm",
    "RexMp2rfurm",
    "Mp3furmi_rnd",
    "RexMp3furmi_rnd",
    "Mp2fa",
    "RexMp2fa",
    "Op2fcscc",
    "RexOp2fcscc",
    "Mp2fcscc",
    "RexMp2fcscc",
    "Op2fcmp",
    "RexOp2fcmp",
    "Mp2fcmp",
    "RexMp2fcmp",
    "DynRexOp1rr",
    "RexOp1rr",
    "DynRexOp1rout",
    "RexOp1rout",
    "DynRexOp1rin",
    "RexOp1rin",
    "DynRexOp1rio",
    "RexOp1rio",
    "DynRexOp1r_ib",
    "RexOp1r_ib",
    "DynRexOp1r_id",
    "RexOp1r_id",
    "DynRexOp1ur",
    "RexOp1ur",
    "Op1ur",
    "Op1rr",
    "DynRexOp2rrx",
    "RexOp2rrx",
    "DynRexOp1div",
    "RexOp1div",
    "DynRexOp1mulx",
    "RexOp1mulx",
    "Op2fa",
    "RexOp2fa",
    "Op2fax",
    "RexOp2fax",
    "Op1rc",
    "RexOp1rc",
    "Mp2urm",
    "RexMp2urm",
    "DynRexOp2bsf_and_bsr",
    "RexOp2bsf_and_bsr",
    "DynRexOp1icscc",
    "RexOp1icscc",
    "DynRexOp1icscc_ib",
    "RexOp1icscc_ib",
    "DynRexOp1icscc_id",
    "RexOp1icscc_id",
    "DynRexOp1rcmp",
    "RexOp1rcmp",
    "DynRexOp1rcmp_ib",
    "RexOp1rcmp_ib",
    "DynRexOp1rcmp_id",
    "RexOp1rcmp_id",
    "Op1rcmp_sp",
    "RexOp1rcmp_sp",
    "Op2seti_abcd",
    "RexOp2seti",
    "Op2setf_abcd",
    "RexOp2setf",
    "DynRexOp2cmov",
    "RexOp2cmov",
    "Mp3fa",
    "DynRexMp3fa",
    "Mp2r_ib_unsigned_fpr",
    "DynRexMp2r_ib_unsigned_fpr",
    "Mp3blend",
    "DynRexMp3blend",
    "Mp3fa_ib",
    "DynRexMp3fa_ib",
    "null_fpr",
    "Mp3r_ib_unsigned_r",
    "DynRexMp3r_ib_unsigned_r",
    "Mp2r_ib_unsigned_r",
    "DynRexMp2r_ib_unsigned_r",
    "RexMp3r_ib_unsigned_r",
    "DynRexMp2fa",
    "DynRexOp2fa",
    "Mp3r_ib_unsigned_gpr",
    "DynRexMp3r_ib_unsigned_gpr",
    "RexMp3r_ib_unsigned_gpr",
    "Mp3furm",
    "DynRexMp3furm",
    "EvexMp2evex_reg_rm_128",
    "DynRexMp2furm",
    "DynRexMp2vconst_optimized",
    "Op2vconst",
    "DynRexOp2vconst",
    "Op2fst",
    "DynRexOp2fst",
    "Op2fstDisp8",
    "DynRexOp2fstDisp8",
    "Op2fstDisp32",
    "DynRexOp2fstDisp32",
    "Op2fstWithIndex",
    "RexOp2fstWithIndex",
    "Op2fstWithIndexDisp8",
    "RexOp2fstWithIndexDisp8",
    "Op2fstWithIndexDisp32",
    "RexOp2fstWithIndexDisp32",
    "Op2fld",
    "DynRexOp2fld",
    "Op2fldDisp8",
    "DynRexOp2fldDisp8",
    "Op2fldDisp32",
    "DynRexOp2fldDisp32",
    "Op2fldWithIndex",
    "RexOp2fldWithIndex",
    "Op2fldWithIndexDisp8",
    "RexOp2fldWithIndexDisp8",
    "Op2fldWithIndexDisp32",
    "RexOp2fldWithIndexDisp32",
    "Op2fspillSib32",
    "RexOp2fspillSib32",
    "Op2fregspill32",
    "RexOp2fregspill32",
    "Op2ffillSib32",
    "RexOp2ffillSib32",
    "Op2fregfill32",
    "RexOp2fregfill32",
    "Op2furm_reg_to_ssa",
    "RexOp2furm_reg_to_ssa",
    "Mp3fld",
    "DynRexMp3fld",
    "Mp3fldDisp8",
    "DynRexMp3fldDisp8",
    "Mp3fldDisp32",
    "DynRexMp3fldDisp32",
    "Mp3fldWithIndex",
    "RexMp3fldWithIndex",
    "Mp3fldWithIndexDisp8",
    "RexMp3fldWithIndexDisp8",
    "Mp3fldWithIndexDisp32",
    "RexMp3fldWithIndexDisp32",
    "EvexMp3evex_reg_vvvv_rm_128",
    "Mp2fax",
    "DynRexMp2fax",
    "Mp3fcmp",
    "DynRexMp3fcmp",
    "Mp2f_ib",
    "DynRexMp2f_ib",
    "Mp2icscc_fpr",
    "DynRexMp2icscc_fpr",
    "Mp3icscc_fpr",
    "DynRexMp3icscc_fpr",
    "Op2pfcmp",
    "DynRexOp2pfcmp",
    "Mp2pfcmp",
    "DynRexMp2pfcmp",
    "DynRexOp2furm",
    "Op1fnaddr4",
    "RexOp1fnaddr8",
    "Op1allones_fnaddr4",
    "RexOp1allones_fnaddr8",
    "RexOp1pcrel_fnaddr8",
    "RexOp1got_fnaddr8",
    "Op1gvaddr4",
    "RexOp1gvaddr8",
    "RexOp1pcrel_gvaddr8",
    "RexOp1got_gvaddr8",
    "RexOp1spaddr_id",
    "Op1spaddr_id",
    "RexOp1const_addr",
    "Op1const_addr",
    "Op1call_id",
    "Op1call_plt_id",
    "Op1call_r",
    "RexOp1call_r",
    "Op1ret",
    "Op1jmpb",
    "Op1jmpd",
    "Op1brib",
    "RexOp1brib",
    "Op2brid",
    "RexOp2brid",
    "Op1brfb",
    "RexOp1brfb",
    "Op2brfd",
    "RexOp2brfd",
    "Op1tjccb",
    "RexOp1tjccb",
    "Op1tjccd",
    "RexOp1tjccd",
    "Op1t8jccd_long",
    "Op1t8jccb_abcd",
    "RexOp1t8jccb",
    "Op1t8jccd_abcd",
    "RexOp1t8jccd",
    "RexOp1jt_entry",
    "Op1jt_entry",
    "RexOp1jt_base",
    "Op1jt_base",
    "RexOp1indirect_jmp",
    "Op1indirect_jmp",
    "Op2trap",
    "debugtrap",
    "trapif",
    "trapff",
    "Op1pu_id_ref",
    "RexOp1pu_id_ref",
    "Op1is_zero",
    "RexOp1is_zero",
    "Op1is_invalid",
    "RexOp1is_invalid",
    "safepoint",
    "elf_tls_get_addr",
    "macho_tls_get_addr",
];

/// x86 recipe constraints list, using the same recipe index spaces as the one
/// specified by the corresponding binemit file. These constraints are used by register
/// allocation to select the right location to use for input and output values.
static RECIPE_CONSTRAINTS: [RecipeConstraints; 363] = [
    // Constraints for recipe get_pinned_reg:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(31),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1set_pinned_reg:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe DynRexOp1umr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1umr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1umr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1rmov:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1rmov:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1pu_id:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pu_id:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1u_id:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pu_iq:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1pu_id_bool:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pu_id_bool:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1u_id_z:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1u_id_z:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe null:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2urm_noflags_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2urm_noflags:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2urm_noflags:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1urm_noflags:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1copysp:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1copysp:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1umr_reg_to_ssa:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1umr_reg_to_ssa:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2furm_reg_to_ssa:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2furm_reg_to_ssa:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe dummy_sarg_t:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1ldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1ldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2ldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1ldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1ldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2ldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1ldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1ldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2ldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp1stWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp1stWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp1stWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp1stWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp1stWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp1stWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stWithIndex_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stWithIndex_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stWithIndexDisp8_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stWithIndexDisp8_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stWithIndexDisp32_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stWithIndexDisp32_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1st:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1st:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp1st:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp1st:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp1stDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp1stDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1stDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp1stDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp1stDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1st_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stDisp8_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1stDisp32_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1spillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1spillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1regspill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1regspill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1ld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1ld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2ld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1ldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1ldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2ldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1ldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1ldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2ldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1fillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1fillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1regfill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1regfill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe fillnull:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe ffillnull:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1pushq:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pushq:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1popq:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1popq:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe stacknull:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1adjustsp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1adjustsp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1adjustsp_ib:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1adjustsp_id:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1adjustsp_ib:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1adjustsp_id:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2frurm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2frurm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2rfumr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2rfumr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2furm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2furm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2frmov:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2frmov:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fst:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fst:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fstDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fstDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fstDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fstDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fstWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fstWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fstWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fstWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fstWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fstWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2ffillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2ffillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fregfill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fregfill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fspillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fspillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2fregspill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2fregspill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2f32imm_z:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2f64imm_z:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp2f32imm_z:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp2f64imm_z:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexMp2frurm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2furm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2furm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp2rfurm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp2rfurm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp3furmi_rnd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp3furmi_rnd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2fa:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp2fa:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2fcscc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp2fcscc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2fcscc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp2fcscc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2fcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp2fcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2fcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp2fcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1rr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1rout:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rout:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1rin:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rin:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1rio:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedTied(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedTied(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: true,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rio:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedTied(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedTied(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: true,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1r_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1r_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1r_id:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1r_id:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1ur:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1ur:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1ur:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1rr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp2rrx:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp2rrx:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1div:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedTied(16),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedTied(18),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedTied(16),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedTied(18),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1div:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedTied(16),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedTied(18),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedTied(16),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedTied(18),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1mulx:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedTied(16),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedTied(16),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(18),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1mulx:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedTied(16),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedTied(16),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(18),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2fa:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp2fa:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2fax:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(1),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp2fax:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(1),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1rc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(17),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(17),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2urm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp2urm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp2bsf_and_bsr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp2bsf_and_bsr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1icscc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1icscc:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1icscc_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1icscc_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1icscc_id:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1icscc_id:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1rcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1rcmp_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rcmp_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp1rcmp_id:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rcmp_id:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1rcmp_sp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1rcmp_sp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2seti_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2seti:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2setf_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2setf:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe DynRexOp2cmov:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(2),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2cmov:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(2),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp3fa:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexMp3fa:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2r_ib_unsigned_fpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexMp2r_ib_unsigned_fpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp3blend:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(2),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexMp3blend:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(2),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp3fa_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexMp3fa_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe null_fpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp3r_ib_unsigned_r:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexMp3r_ib_unsigned_r:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2r_ib_unsigned_r:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexMp2r_ib_unsigned_r:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp3r_ib_unsigned_r:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexMp2fa:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp2fa:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp3r_ib_unsigned_gpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexMp3r_ib_unsigned_gpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexMp3r_ib_unsigned_gpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp3furm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe DynRexMp3furm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe EvexMp2evex_reg_rm_128:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexMp2furm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe DynRexMp2vconst_optimized:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2vconst:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe DynRexOp2vconst:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fst:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe DynRexOp2fst:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fstDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe DynRexOp2fstDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fstDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe DynRexOp2fstDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fstWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2fstWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fstWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2fstWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fstWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2fstWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe DynRexOp2fld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe DynRexOp2fldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe DynRexOp2fldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2fldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2fldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2fldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fspillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2fspillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fregspill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2fregspill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2ffillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2ffillSib32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2fregfill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2fregfill32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Stack,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2furm_reg_to_ssa:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2furm_reg_to_ssa:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp3fld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe DynRexMp3fld:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp3fldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe DynRexMp3fldDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp3fldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe DynRexMp3fldDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp3fldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp3fldWithIndex:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp3fldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp3fldWithIndexDisp8:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Mp3fldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexMp3fldWithIndexDisp32:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe EvexMp3evex_reg_vvvv_rm_128:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2fax:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(1),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexMp2fax:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(1),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp3fcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexMp3fcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2f_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexMp2f_ib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2icscc_fpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexMp2icscc_fpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp3icscc_fpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexMp3icscc_fpr:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Op2pfcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp2pfcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe Mp2pfcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexMp2pfcmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Tied(0),
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: true,
        clobbers_flags: true,
    },
    // Constraints for recipe DynRexOp2furm:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &FPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1fnaddr4:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1fnaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1allones_fnaddr4:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1allones_fnaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pcrel_fnaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1got_fnaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1gvaddr4:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1gvaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pcrel_gvaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1got_gvaddr8:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1spaddr_id:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1spaddr_id:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1const_addr:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1const_addr:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1call_id:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1call_plt_id:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1call_r:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1call_r:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1ret:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1jmpb:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1jmpd:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1brib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1brib:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2brid:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2brid:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1brfb:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1brfb:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2brfd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp2brfd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1tjccb:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1tjccb:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1tjccd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1tjccd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1t8jccd_long:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1t8jccb_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1t8jccb:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1t8jccd_abcd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1t8jccd:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1jt_entry:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1jt_entry:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1jt_base:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1jt_base:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe RexOp1indirect_jmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1indirect_jmp:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op2trap:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe debugtrap:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe trapif:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe trapff:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(32),
                regclass: &FLAG_DATA,
            },
        ],
        outs: &[],
        fixed_ins: true,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: false,
    },
    // Constraints for recipe Op1pu_id_ref:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1pu_id_ref:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1is_zero:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1is_zero:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe Op1is_invalid:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR8_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe RexOp1is_invalid:
    RecipeConstraints {
        ins: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &GPR_DATA,
            },
        ],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::Reg,
                regclass: &ABCD_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe safepoint:
    RecipeConstraints {
        ins: &[],
        outs: &[],
        fixed_ins: false,
        fixed_outs: false,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe elf_tls_get_addr:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(16),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
    // Constraints for recipe macho_tls_get_addr:
    RecipeConstraints {
        ins: &[],
        outs: &[
            OperandConstraint {
                kind: ConstraintKind::FixedReg(16),
                regclass: &GPR_DATA,
            },
        ],
        fixed_ins: false,
        fixed_outs: true,
        tied_ops: false,
        clobbers_flags: true,
    },
];

/// x86 recipe sizing descriptors, using the same recipe index spaces as the one
/// specified by the corresponding binemit file. These are used to compute the final size of an
/// instruction, as well as to compute the range of branches.
static RECIPE_SIZING: [RecipeSizing; 363] = [
    // Code size information for recipe get_pinned_reg:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1set_pinned_reg:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1umr:
    RecipeSizing {
        base_size: 2,
        compute_size: size_with_inferred_rex_for_inreg0_outreg0,
        branch_range: None,
    },
    // Code size information for recipe RexOp1umr:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1umr:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1rmov:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rmov:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1pu_id:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pu_id:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1u_id:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pu_iq:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1pu_id_bool:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pu_id_bool:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1u_id_z:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1u_id_z:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe null:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2urm_noflags_abcd:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2urm_noflags:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2urm_noflags:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1urm_noflags:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1copysp:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1copysp:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1umr_reg_to_ssa:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1umr_reg_to_ssa:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2furm_reg_to_ssa:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2furm_reg_to_ssa:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe dummy_sarg_t:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1ldWithIndex:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp1ldWithIndex:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Op2ldWithIndex:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp2ldWithIndex:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Op1ldWithIndexDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1ldWithIndexDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2ldWithIndexDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2ldWithIndexDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1ldWithIndexDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1ldWithIndexDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2ldWithIndexDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2ldWithIndexDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1stWithIndex:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stWithIndex:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp1stWithIndex:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp1stWithIndex:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1stWithIndexDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stWithIndexDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp1stWithIndexDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp1stWithIndexDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1stWithIndexDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stWithIndexDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp1stWithIndexDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp1stWithIndexDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1stWithIndex_abcd:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stWithIndex_abcd:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1stWithIndexDisp8_abcd:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stWithIndexDisp8_abcd:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1stWithIndexDisp32_abcd:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stWithIndexDisp32_abcd:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1st:
    RecipeSizing {
        base_size: 2,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1st:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp1st:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp1st:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1stDisp8:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp1stDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp1stDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1stDisp32:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1stDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp1stDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp1stDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1st_abcd:
    RecipeSizing {
        base_size: 2,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1stDisp8_abcd:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1stDisp32_abcd:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1spillSib32:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1spillSib32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1regspill32:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1regspill32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1ld:
    RecipeSizing {
        base_size: 2,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp1ld:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Op2ld:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp2ld:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Op1ldDisp8:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp1ldDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Op2ldDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp2ldDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Op1ldDisp32:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp1ldDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Op2ldDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp2ldDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Op1fillSib32:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1fillSib32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1regfill32:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1regfill32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe fillnull:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe ffillnull:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1pushq:
    RecipeSizing {
        base_size: 1,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pushq:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1popq:
    RecipeSizing {
        base_size: 1,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1popq:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe stacknull:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1adjustsp:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1adjustsp:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1adjustsp_ib:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1adjustsp_id:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1adjustsp_ib:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1adjustsp_id:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2frurm:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2frurm:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2rfumr:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2rfumr:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2furm:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2furm:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2frmov:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2frmov:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fld:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fld:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Mp2fldDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fldDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Mp2fldDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fldDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Mp2fldWithIndex:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fldWithIndex:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Mp2fldWithIndexDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fldWithIndexDisp8:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fldWithIndexDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fldWithIndexDisp32:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fst:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fst:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp2fstDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fstDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp2fstDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fstDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp2fstWithIndex:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fstWithIndex:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Mp2fstWithIndexDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fstWithIndexDisp8:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fstWithIndexDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fstWithIndexDisp32:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2ffillSib32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2ffillSib32:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fregfill32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fregfill32:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fspillSib32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fspillSib32:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fregspill32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fregspill32:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2f32imm_z:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2f64imm_z:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2f32imm_z:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2f64imm_z:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp2frurm:
    RecipeSizing {
        base_size: 4,
        compute_size: size_with_inferred_rex_for_inreg0_outreg0,
        branch_range: None,
    },
    // Code size information for recipe Mp2furm:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2furm:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2rfurm:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2rfurm:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp3furmi_rnd:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp3furmi_rnd:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fa:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fa:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fcscc:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fcscc:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fcscc:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fcscc:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fcmp:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fcmp:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fcmp:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2fcmp:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1rr:
    RecipeSizing {
        base_size: 2,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rr:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1rout:
    RecipeSizing {
        base_size: 2,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rout:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1rin:
    RecipeSizing {
        base_size: 2,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rin:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1rio:
    RecipeSizing {
        base_size: 2,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rio:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1r_ib:
    RecipeSizing {
        base_size: 3,
        compute_size: size_with_inferred_rex_for_inreg0,
        branch_range: None,
    },
    // Code size information for recipe RexOp1r_ib:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1r_id:
    RecipeSizing {
        base_size: 6,
        compute_size: size_with_inferred_rex_for_inreg0,
        branch_range: None,
    },
    // Code size information for recipe RexOp1r_id:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1ur:
    RecipeSizing {
        base_size: 2,
        compute_size: size_with_inferred_rex_for_inreg0,
        branch_range: None,
    },
    // Code size information for recipe RexOp1ur:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1ur:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1rr:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp2rrx:
    RecipeSizing {
        base_size: 3,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe RexOp2rrx:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1div:
    RecipeSizing {
        base_size: 2,
        compute_size: size_with_inferred_rex_for_inreg2,
        branch_range: None,
    },
    // Code size information for recipe RexOp1div:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1mulx:
    RecipeSizing {
        base_size: 2,
        compute_size: size_with_inferred_rex_for_inreg1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1mulx:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fa:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fa:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fax:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fax:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1rc:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rc:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2urm:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp2urm:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp2bsf_and_bsr:
    RecipeSizing {
        base_size: 3,
        compute_size: size_with_inferred_rex_for_inreg0_outreg0,
        branch_range: None,
    },
    // Code size information for recipe RexOp2bsf_and_bsr:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1icscc:
    RecipeSizing {
        base_size: 5,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1icscc:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1icscc_ib:
    RecipeSizing {
        base_size: 6,
        compute_size: size_with_inferred_rex_for_inreg0,
        branch_range: None,
    },
    // Code size information for recipe RexOp1icscc_ib:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1icscc_id:
    RecipeSizing {
        base_size: 9,
        compute_size: size_with_inferred_rex_for_inreg0,
        branch_range: None,
    },
    // Code size information for recipe RexOp1icscc_id:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1rcmp:
    RecipeSizing {
        base_size: 2,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rcmp:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1rcmp_ib:
    RecipeSizing {
        base_size: 3,
        compute_size: size_with_inferred_rex_for_inreg0,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rcmp_ib:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp1rcmp_id:
    RecipeSizing {
        base_size: 6,
        compute_size: size_with_inferred_rex_for_inreg0,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rcmp_id:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1rcmp_sp:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1rcmp_sp:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2seti_abcd:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2seti:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2setf_abcd:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2setf:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp2cmov:
    RecipeSizing {
        base_size: 3,
        compute_size: size_with_inferred_rex_for_cmov,
        branch_range: None,
    },
    // Code size information for recipe RexOp2cmov:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp3fa:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp3fa:
    RecipeSizing {
        base_size: 5,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe Mp2r_ib_unsigned_fpr:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp2r_ib_unsigned_fpr:
    RecipeSizing {
        base_size: 5,
        compute_size: size_with_inferred_rex_for_inreg0_outreg0,
        branch_range: None,
    },
    // Code size information for recipe Mp3blend:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp3blend:
    RecipeSizing {
        base_size: 5,
        compute_size: size_with_inferred_rex_for_inreg1_inreg2,
        branch_range: None,
    },
    // Code size information for recipe Mp3fa_ib:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp3fa_ib:
    RecipeSizing {
        base_size: 6,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe null_fpr:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp3r_ib_unsigned_r:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp3r_ib_unsigned_r:
    RecipeSizing {
        base_size: 6,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe Mp2r_ib_unsigned_r:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp2r_ib_unsigned_r:
    RecipeSizing {
        base_size: 5,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe RexMp3r_ib_unsigned_r:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp2fa:
    RecipeSizing {
        base_size: 4,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp2fa:
    RecipeSizing {
        base_size: 3,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe Mp3r_ib_unsigned_gpr:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp3r_ib_unsigned_gpr:
    RecipeSizing {
        base_size: 6,
        compute_size: size_with_inferred_rex_for_inreg0_outreg0,
        branch_range: None,
    },
    // Code size information for recipe RexMp3r_ib_unsigned_gpr:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp3furm:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp3furm:
    RecipeSizing {
        base_size: 5,
        compute_size: size_with_inferred_rex_for_inreg0_outreg0,
        branch_range: None,
    },
    // Code size information for recipe EvexMp2evex_reg_rm_128:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp2furm:
    RecipeSizing {
        base_size: 4,
        compute_size: size_with_inferred_rex_for_inreg0_outreg0,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp2vconst_optimized:
    RecipeSizing {
        base_size: 4,
        compute_size: size_with_inferred_rex_for_outreg0,
        branch_range: None,
    },
    // Code size information for recipe Op2vconst:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp2vconst:
    RecipeSizing {
        base_size: 7,
        compute_size: size_with_inferred_rex_for_outreg0,
        branch_range: None,
    },
    // Code size information for recipe Op2fst:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp2fst:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_or_offset_inreg1_plus_rex_prefix_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe Op2fstDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp2fstDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_inreg1_plus_rex_prefix_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe Op2fstDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_sib_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp2fstDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_sib_inreg1_plus_rex_prefix_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe Op2fstWithIndex:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fstWithIndex:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Op2fstWithIndexDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fstWithIndexDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fstWithIndexDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fstWithIndexDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fld:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp2fld:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_0_plus_rex_prefix_for_inreg0_outreg0,
        branch_range: None,
    },
    // Code size information for recipe Op2fldDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp2fldDisp8:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_sib_for_inreg_0_plus_rex_prefix_for_inreg0_outreg0,
        branch_range: None,
    },
    // Code size information for recipe Op2fldDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp2fldDisp32:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_sib_for_inreg_0_plus_rex_prefix_for_inreg0_outreg0,
        branch_range: None,
    },
    // Code size information for recipe Op2fldWithIndex:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fldWithIndex:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Op2fldWithIndexDisp8:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fldWithIndexDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fldWithIndexDisp32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fldWithIndexDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fspillSib32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fspillSib32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fregspill32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fregspill32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2ffillSib32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2ffillSib32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2fregfill32:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2fregfill32:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2furm_reg_to_ssa:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp2furm_reg_to_ssa:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp3fld:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp3fld:
    RecipeSizing {
        base_size: 5,
        compute_size: size_plus_maybe_sib_or_offset_for_inreg_0_plus_rex_prefix_for_inreg0_outreg0,
        branch_range: None,
    },
    // Code size information for recipe Mp3fldDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp3fldDisp8:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_sib_for_inreg_0_plus_rex_prefix_for_inreg0_outreg0,
        branch_range: None,
    },
    // Code size information for recipe Mp3fldDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: size_plus_maybe_sib_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp3fldDisp32:
    RecipeSizing {
        base_size: 9,
        compute_size: size_plus_maybe_sib_for_inreg_0_plus_rex_prefix_for_inreg0_outreg0,
        branch_range: None,
    },
    // Code size information for recipe Mp3fldWithIndex:
    RecipeSizing {
        base_size: 6,
        compute_size: size_plus_maybe_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe RexMp3fldWithIndex:
    RecipeSizing {
        base_size: 7,
        compute_size: size_plus_maybe_offset_for_inreg_0,
        branch_range: None,
    },
    // Code size information for recipe Mp3fldWithIndexDisp8:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp3fldWithIndexDisp8:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp3fldWithIndexDisp32:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexMp3fldWithIndexDisp32:
    RecipeSizing {
        base_size: 11,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe EvexMp3evex_reg_vvvv_rm_128:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Mp2fax:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp2fax:
    RecipeSizing {
        base_size: 4,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe Mp3fcmp:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp3fcmp:
    RecipeSizing {
        base_size: 5,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe Mp2f_ib:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp2f_ib:
    RecipeSizing {
        base_size: 5,
        compute_size: size_with_inferred_rex_for_inreg0,
        branch_range: None,
    },
    // Code size information for recipe Mp2icscc_fpr:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp2icscc_fpr:
    RecipeSizing {
        base_size: 4,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe Mp3icscc_fpr:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp3icscc_fpr:
    RecipeSizing {
        base_size: 5,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe Op2pfcmp:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp2pfcmp:
    RecipeSizing {
        base_size: 4,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe Mp2pfcmp:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe DynRexMp2pfcmp:
    RecipeSizing {
        base_size: 5,
        compute_size: size_with_inferred_rex_for_inreg0_inreg1,
        branch_range: None,
    },
    // Code size information for recipe DynRexOp2furm:
    RecipeSizing {
        base_size: 3,
        compute_size: size_with_inferred_rex_for_inreg0_outreg0,
        branch_range: None,
    },
    // Code size information for recipe Op1fnaddr4:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1fnaddr8:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1allones_fnaddr4:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1allones_fnaddr8:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pcrel_fnaddr8:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1got_fnaddr8:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1gvaddr4:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1gvaddr8:
    RecipeSizing {
        base_size: 10,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pcrel_gvaddr8:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1got_gvaddr8:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1spaddr_id:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1spaddr_id:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1const_addr:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1const_addr:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1call_id:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1call_plt_id:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1call_r:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1call_r:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1ret:
    RecipeSizing {
        base_size: 1,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1jmpb:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 2, bits: 8 }),
    },
    // Code size information for recipe Op1jmpd:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 5, bits: 32 }),
    },
    // Code size information for recipe Op1brib:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 2, bits: 8 }),
    },
    // Code size information for recipe RexOp1brib:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 3, bits: 8 }),
    },
    // Code size information for recipe Op2brid:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 6, bits: 32 }),
    },
    // Code size information for recipe RexOp2brid:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 7, bits: 32 }),
    },
    // Code size information for recipe Op1brfb:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 2, bits: 8 }),
    },
    // Code size information for recipe RexOp1brfb:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 3, bits: 8 }),
    },
    // Code size information for recipe Op2brfd:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 6, bits: 32 }),
    },
    // Code size information for recipe RexOp2brfd:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 7, bits: 32 }),
    },
    // Code size information for recipe Op1tjccb:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 4, bits: 8 }),
    },
    // Code size information for recipe RexOp1tjccb:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 5, bits: 8 }),
    },
    // Code size information for recipe Op1tjccd:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 8, bits: 32 }),
    },
    // Code size information for recipe RexOp1tjccd:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 9, bits: 32 }),
    },
    // Code size information for recipe Op1t8jccd_long:
    RecipeSizing {
        base_size: 12,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 12, bits: 32 }),
    },
    // Code size information for recipe Op1t8jccb_abcd:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 4, bits: 8 }),
    },
    // Code size information for recipe RexOp1t8jccb:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 5, bits: 8 }),
    },
    // Code size information for recipe Op1t8jccd_abcd:
    RecipeSizing {
        base_size: 8,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 8, bits: 32 }),
    },
    // Code size information for recipe RexOp1t8jccd:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: Some(BranchRange { origin: 9, bits: 32 }),
    },
    // Code size information for recipe RexOp1jt_entry:
    RecipeSizing {
        base_size: 4,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe Op1jt_entry:
    RecipeSizing {
        base_size: 3,
        compute_size: size_plus_maybe_offset_for_inreg_1,
        branch_range: None,
    },
    // Code size information for recipe RexOp1jt_base:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1jt_base:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1indirect_jmp:
    RecipeSizing {
        base_size: 3,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1indirect_jmp:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op2trap:
    RecipeSizing {
        base_size: 2,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe debugtrap:
    RecipeSizing {
        base_size: 1,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe trapif:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe trapff:
    RecipeSizing {
        base_size: 4,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1pu_id_ref:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1pu_id_ref:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1is_zero:
    RecipeSizing {
        base_size: 5,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1is_zero:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe Op1is_invalid:
    RecipeSizing {
        base_size: 6,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe RexOp1is_invalid:
    RecipeSizing {
        base_size: 7,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe safepoint:
    RecipeSizing {
        base_size: 0,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe elf_tls_get_addr:
    RecipeSizing {
        base_size: 16,
        compute_size: base_size,
        branch_range: None,
    },
    // Code size information for recipe macho_tls_get_addr:
    RecipeSizing {
        base_size: 9,
        compute_size: base_size,
        branch_range: None,
    },
];

pub static INFO: isa::EncInfo = isa::EncInfo {
    constraints: &RECIPE_CONSTRAINTS,
    sizing: &RECIPE_SIZING,
    names: &RECIPE_NAMES,
};
