//src/jit.rs
//THIS IS TO DO AND CURRENTLY NOT WORKING
use crate::bytecode::value::{self, TAG_INT};
use crate::bytecode::{
    opcode::OpCode,
    value::{FunctionObject, ValueType},
};
use crate::vm::vm_helpers::{
    tundra_array_get, tundra_array_set, tundra_call_raw, tundra_get_global, tundra_invoke,
    tundra_new_array, tundra_pow, tundra_pow_i64, tundra_pow_i64_mod, tundra_print,
    tundra_set_global, tundra_store_register,
};
use cranelift_codegen::ir::condcodes::IntCC;
use cranelift_codegen::ir::InstBuilder;
use cranelift_codegen::ir::{types, types::I64, AbiParam, MemFlags, Signature};
use cranelift_codegen::isa;
use cranelift_codegen::settings::{self, Configurable};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{default_libcall_names, DataId, FuncId, Linkage, Module};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub type JittedFn = extern "C" fn(i64, i64) -> i64;
use crate::bytecode::value::TAG_BOOL;
pub struct JitContext {
    module: JITModule,
    pow_i64_id: FuncId,
    pow_mod_id: FuncId,
    pow_id: FuncId,
    invoke_id: FuncId,
    new_array_id: FuncId,
    array_get_id: FuncId,
    array_set_id: FuncId,
    get_global_id: FuncId,
    set_global_id: FuncId,
    print_id: FuncId,
    call_raw_id: FuncId,
    store_reg_id: FuncId,
    interned_strings: HashMap<String, DataId>,
}

unsafe impl Send for JitContext {}

impl JitContext {
    pub fn new() -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.set("is_pic", "false").unwrap();
        let isa = isa::lookup_by_name("x86_64")
            .unwrap()
            .finish(settings::Flags::new(flag_builder))
            .unwrap();
        let mut jit_builder = JITBuilder::with_isa(isa, default_libcall_names());
        jit_builder.symbol("tundra_pow_i64", tundra_pow_i64 as *const u8);
        jit_builder.symbol("tundra_pow_i64_mod", tundra_pow_i64_mod as *const u8);
        jit_builder.symbol("tundra_pow", tundra_pow as *const u8);
        jit_builder.symbol("tundra_invoke", tundra_invoke as *const u8);
        jit_builder.symbol("tundra_new_array", tundra_new_array as *const u8);
        jit_builder.symbol("tundra_array_get", tundra_array_get as *const u8);
        jit_builder.symbol("tundra_array_set", tundra_array_set as *const u8);
        jit_builder.symbol("tundra_print", tundra_print as *const u8);
        jit_builder.symbol("tundra_get_global", tundra_get_global as *const u8);
        jit_builder.symbol("tundra_set_global", tundra_set_global as *const u8);
        jit_builder.symbol("tundra_call_raw", tundra_call_raw as *const u8);
        jit_builder.symbol("tundra_store_register", tundra_store_register as *const u8);

        let mut module = JITModule::new(jit_builder);
        let ptr_ty = module.target_config().pointer_type();

        let mut sig = Signature::new(module.isa().default_call_conv());
        sig.params.push(AbiParam::new(I64));
        sig.params.push(AbiParam::new(I64));
        sig.returns.push(AbiParam::new(I64));
        let pow_i64_id = module
            .declare_function("tundra_pow_i64", Linkage::Import, &sig)
            .unwrap();

        let mut sig_mod = Signature::new(module.isa().default_call_conv());
        sig_mod.params.push(AbiParam::new(I64));
        sig_mod.params.push(AbiParam::new(I64));
        sig_mod.params.push(AbiParam::new(I64));
        sig_mod.returns.push(AbiParam::new(I64));
        let pow_mod_id = module
            .declare_function("tundra_pow_i64_mod", Linkage::Import, &sig_mod)
            .unwrap();

        let mut sig_pow = Signature::new(module.isa().default_call_conv());
        sig_pow.params.push(AbiParam::new(I64));
        sig_pow.params.push(AbiParam::new(I64));
        sig_pow.params.push(AbiParam::new(I64));
        sig_pow.returns.push(AbiParam::new(I64));
        let pow_id = module
            .declare_function("tundra_pow", Linkage::Import, &sig_pow)
            .unwrap();

        let mut sig = Signature::new(module.isa().default_call_conv());
        sig.params.push(AbiParam::new(ptr_ty));
        sig.params.push(AbiParam::new(I64));
        sig.params.push(AbiParam::new(I64));
        sig.params.push(AbiParam::new(I64));
        sig.returns.push(AbiParam::new(I64));
        let invoke_id = module
            .declare_function("tundra_invoke", Linkage::Import, &sig)
            .unwrap();

        let mut sig = Signature::new(module.isa().default_call_conv());
        sig.params.push(AbiParam::new(ptr_ty));
        sig.params.push(AbiParam::new(I64));
        sig.returns.push(AbiParam::new(ptr_ty));
        let new_array_id = module
            .declare_function("tundra_new_array", Linkage::Import, &sig)
            .unwrap();

        let mut sig = Signature::new(module.isa().default_call_conv());
        sig.params.push(AbiParam::new(ptr_ty));
        sig.params.push(AbiParam::new(I64));
        sig.params.push(AbiParam::new(I64));
        sig.returns.push(AbiParam::new(I64));
        let array_get_id = module
            .declare_function("tundra_array_get", Linkage::Import, &sig)
            .unwrap();

        let mut sig = Signature::new(module.isa().default_call_conv());
        sig.params.push(AbiParam::new(ptr_ty));
        sig.params.push(AbiParam::new(I64));
        sig.params.push(AbiParam::new(I64));
        sig.params.push(AbiParam::new(I64));
        let array_set_id = module
            .declare_function("tundra_array_set", Linkage::Import, &sig)
            .unwrap();

        let mut sig = Signature::new(module.isa().default_call_conv());
        sig.params.push(AbiParam::new(ptr_ty));
        sig.params.push(AbiParam::new(I64));
        let print_id = module
            .declare_function("tundra_print", Linkage::Import, &sig)
            .unwrap();

        let mut sig = Signature::new(module.isa().default_call_conv());
        sig.params.push(AbiParam::new(ptr_ty));
        sig.params.push(AbiParam::new(ptr_ty));
        sig.params.push(AbiParam::new(I64));
        sig.returns.push(AbiParam::new(I64));
        let get_global_id = module
            .declare_function("tundra_get_global", Linkage::Import, &sig)
            .unwrap();

        let mut sig = Signature::new(module.isa().default_call_conv());
        sig.params.push(AbiParam::new(ptr_ty));
        sig.params.push(AbiParam::new(ptr_ty));
        sig.params.push(AbiParam::new(I64));
        sig.params.push(AbiParam::new(I64));
        let set_global_id = module
            .declare_function("tundra_set_global", Linkage::Import, &sig)
            .unwrap();

        let mut sig_raw = Signature::new(module.isa().default_call_conv());
        sig_raw.params.push(AbiParam::new(ptr_ty));
        sig_raw.params.push(AbiParam::new(I64));
        sig_raw.params.push(AbiParam::new(I64));
        sig_raw.params.push(AbiParam::new(I64));
        sig_raw.returns.push(AbiParam::new(I64));
        let call_raw_id = module
            .declare_function("tundra_call_raw", Linkage::Import, &sig_raw)
            .unwrap();

        let mut sig_store = Signature::new(module.isa().default_call_conv());
        sig_store.params.push(AbiParam::new(ptr_ty));
        sig_store.params.push(AbiParam::new(I64));
        sig_store.params.push(AbiParam::new(I64));

        let store_reg_id = module
            .declare_function("tundra_store_register", Linkage::Import, &sig_store)
            .unwrap();
        Self {
            module,
            pow_i64_id,
            pow_mod_id,
            pow_id,
            invoke_id,
            new_array_id,
            array_get_id,
            array_set_id,
            get_global_id,
            set_global_id,
            print_id,
            call_raw_id,
            store_reg_id,
            interned_strings: HashMap::new(),
        }
    }

    pub fn compile_function(&mut self, fobj: &mut FunctionObject) {
        let ptr_ty = self.module.target_config().pointer_type();

        let mut sig = Signature::new(self.module.isa().default_call_conv());
        sig.params.push(AbiParam::new(I64));
        sig.params.push(AbiParam::new(I64));
        for _ in 0..fobj.arity {
            sig.params.push(AbiParam::new(I64));
        }
        sig.returns.push(AbiParam::new(I64));

        let name = format!("tundra_{}", fobj.name);
        let func_id = self
            .module
            .declare_function(&name, Linkage::Local, &sig)
            .unwrap();

        let mut ctx = self.module.make_context();
        ctx.func.signature = sig.clone();

        let pow_i64_ref = self
            .module
            .declare_func_in_func(self.pow_i64_id, &mut ctx.func);
        let pow_mod_ref = self
            .module
            .declare_func_in_func(self.pow_mod_id, &mut ctx.func);
        let pow_ref = self.module.declare_func_in_func(self.pow_id, &mut ctx.func);
        let invoke_ref = self
            .module
            .declare_func_in_func(self.invoke_id, &mut ctx.func);
        let call_raw_ref = self
            .module
            .declare_func_in_func(self.call_raw_id, &mut ctx.func);
        let new_array_ref = self
            .module
            .declare_func_in_func(self.new_array_id, &mut ctx.func);
        let array_get_ref = self
            .module
            .declare_func_in_func(self.array_get_id, &mut ctx.func);
        let array_set_ref = self
            .module
            .declare_func_in_func(self.array_set_id, &mut ctx.func);
        let print_ref = self
            .module
            .declare_func_in_func(self.print_id, &mut ctx.func);
        let get_global_ref = self
            .module
            .declare_func_in_func(self.get_global_id, &mut ctx.func);
        let set_global_ref = self
            .module
            .declare_func_in_func(self.set_global_id, &mut ctx.func);
        let store_reg_ref = self
            .module
            .declare_func_in_func(self.store_reg_id, &mut ctx.func);

        let direct_ref = self.module.declare_func_in_func(func_id, &mut ctx.func);

        let mut fn_ctx = FunctionBuilderContext::new();
        let mut fb = FunctionBuilder::new(&mut ctx.func, &mut fn_ctx);
        let entry = fb.create_block();
        fb.append_block_params_for_function_params(entry);
        fb.switch_to_block(entry);
        let vm_ptr = fb.block_params(entry)[0];
        let base = fb.block_params(entry)[1];

        let maxr = fobj.chunk.borrow().max_register;
        let mut vars = Vec::with_capacity(maxr);
        for i in 0..maxr {
            let v = Variable::from_u32(i as u32);
            fb.declare_var(v, I64);
            vars.push(v);
        }
        let zero = fb.ins().iconst(I64, 0);
        for &v in &vars {
            fb.def_var(v, zero);
        }
        for i in 0..fobj.arity {
            let arg_val = fb.block_params(entry)[2 + i];
            fb.def_var(vars[i], arg_val);
        }

        let code = fobj.chunk.borrow().code.clone();
        let code_len = code.len();
        let mut blocks = code.iter().map(|_| fb.create_block()).collect::<Vec<_>>();
        let exit_block = fb.create_block();
        let mut preds = vec![Vec::new(); code_len];
        for (pc, op) in code.iter().enumerate() {
            match op {
                OpCode::JumpIfFalse(_, offset) => {
                    let fall = pc + 1;
                    let tgt = pc + 1 + offset;
                    if fall < code_len {
                        preds[fall].push(pc)
                    }
                    if tgt < code_len {
                        preds[tgt].push(pc)
                    }
                }
                OpCode::Jump(offset) => {
                    let tgt = pc + 1 + offset;
                    if tgt < code_len {
                        preds[tgt].push(pc)
                    }
                }
                OpCode::Loop(back) => {
                    let tgt = pc + 1 - back;
                    preds[tgt].push(pc);
                }
                OpCode::Return(_) => {}

                OpCode::IncLoopIfLess(_, _, target) => {
                    if pc + 1 < code_len {
                        preds[pc + 1].push(pc);
                    }

                    if *target < code_len {
                        preds[*target].push(pc);
                    }
                }
                _ => {
                    if pc + 1 < code_len {
                        preds[pc + 1].push(pc);
                    }
                }
            }
        }

        let mut seal_at = vec![Vec::new(); code_len];
        for j in 0..code_len {
            let last = preds[j].iter().cloned().max().unwrap_or(j);
            seal_at[last].push(j);
        }

        fb.ins().jump(blocks[0], &[]);

        fn tag_bool<'f>(
            fb: &mut FunctionBuilder<'f>,
            rawb: cranelift_codegen::ir::Value,
        ) -> cranelift_codegen::ir::Value {
            let i64b = fb.ins().uextend(I64, rawb);
            let sh = fb.ins().ishl_imm(i64b, 3);
            fb.ins().bor_imm(sh, TAG_BOOL as i64)
        }

        for (pc, op) in code.into_iter().enumerate() {
            let this_block = blocks[pc];
            fb.switch_to_block(this_block);
            let mut is_terminator = false;
            match op {
                OpCode::LoadConstant(d, val) => {
                    let raw: i64 = val.as_i64();
                    let imm = fb.ins().iconst(I64, raw);
                    fb.def_var(vars[d], imm);
                }

                OpCode::Move(d, s) => {
                    let v = fb.use_var(vars[s]);
                    fb.def_var(vars[d], v);
                }

                OpCode::Add(d, a, b) => {
                    let a_tag = fb.use_var(vars[a]);
                    let b_tag = fb.use_var(vars[b]);

                    let a_val = fb.ins().ushr_imm(a_tag, 3);
                    let b_val = fb.ins().ushr_imm(b_tag, 3);

                    let raw_sum = fb.ins().iadd(a_val, b_val);

                    let shifted = fb.ins().ishl_imm(raw_sum, 3);
                    let tagged = fb.ins().bor_imm(shifted, 1);
                    fb.def_var(vars[d], tagged);
                }

                OpCode::Subtract(d, a, b) => {
                    let a_tag = fb.use_var(vars[a]);
                    let b_tag = fb.use_var(vars[b]);
                    let a_val = fb.ins().ushr_imm(a_tag, 3);
                    let b_val = fb.ins().ushr_imm(b_tag, 3);
                    let raw = fb.ins().isub(a_val, b_val);
                    let shifted = fb.ins().ishl_imm(raw, 3);
                    let tagged = fb.ins().bor_imm(shifted, 1);
                    fb.def_var(vars[d], tagged);
                }

                OpCode::Multiply(d, a, b) => {
                    let a_tag = fb.use_var(vars[a]);
                    let b_tag = fb.use_var(vars[b]);
                    let a_val = fb.ins().ushr_imm(a_tag, 3);
                    let b_val = fb.ins().ushr_imm(b_tag, 3);
                    let raw = fb.ins().imul(a_val, b_val);
                    let shifted = fb.ins().ishl_imm(raw, 3);
                    let tagged = fb.ins().bor_imm(shifted, 1);
                    fb.def_var(vars[d], tagged);
                }

                OpCode::IntDivide(d, a, b) => {
                    let a_tag = fb.use_var(vars[a]);
                    let b_tag = fb.use_var(vars[b]);
                    let a_val = fb.ins().ushr_imm(a_tag, 3);
                    let b_val = fb.ins().ushr_imm(b_tag, 3);
                    let raw = fb.ins().sdiv(a_val, b_val);
                    let shifted = fb.ins().ishl_imm(raw, 3);
                    let tagged = fb.ins().bor_imm(shifted, 1);
                    fb.def_var(vars[d], tagged);
                }

                OpCode::Mod(d, a, b) => {
                    let a_tag = fb.use_var(vars[a]);
                    let b_tag = fb.use_var(vars[b]);
                    let a_val = fb.ins().ushr_imm(a_tag, 3);
                    let b_val = fb.ins().ushr_imm(b_tag, 3);
                    let raw = fb.ins().srem(a_val, b_val);
                    let shifted = fb.ins().ishl_imm(raw, 3);
                    let tagged = fb.ins().bor_imm(shifted, 1);
                    fb.def_var(vars[d], tagged);
                }

                OpCode::Divide(d, a, b) => {
                    let a_bits = fb.use_var(vars[a]);
                    let b_bits = fb.use_var(vars[b]);
                    let a_f = fb.ins().bitcast(types::F64, MemFlags::new(), a_bits);
                    let b_f = fb.ins().bitcast(types::F64, MemFlags::new(), b_bits);
                    let r_f = fb.ins().fdiv(a_f, b_f);
                    let r_i = fb.ins().bitcast(types::I64, MemFlags::new(), r_f);
                    fb.def_var(vars[d], r_i);
                }

                OpCode::Exponentiate(d, a, b) => {
                    let x1 = fb.use_var(vars[a]);
                    let x2 = fb.use_var(vars[b]);
                    let a_v = fb.ins().ushr_imm(x1, 3);
                    let b_v = fb.ins().ushr_imm(x2, 3);
                    let call = fb.ins().call(pow_i64_ref, &[a_v, b_v]);
                    let raw = fb.inst_results(call)[0];

                    let sh = fb.ins().ishl_imm(raw, 3);
                    let tagged = fb.ins().bor_imm(sh, TAG_INT as i64);
                    fb.def_var(vars[d], tagged);
                }
                OpCode::BitwiseAnd(d, a, b) => {
                    let x1 = fb.use_var(vars[a]);
                    let x2 = fb.use_var(vars[b]);
                    let x3 = fb.ins().ushr_imm(x1, 3);
                    let x4 = fb.ins().ushr_imm(x2, 3);
                    let u = fb.ins().band(x3, x4);
                    let sh = fb.ins().ishl_imm(u, 3);
                    let tagged = fb.ins().bor_imm(sh, TAG_INT as i64);
                    fb.def_var(vars[d], tagged);
                }
                OpCode::BitwiseOr(d, a, b) => {
                    let x1 = fb.use_var(vars[a]);
                    let x2 = fb.use_var(vars[b]);
                    let x3 = fb.ins().ushr_imm(x1, 3);
                    let x4 = fb.ins().ushr_imm(x2, 3);
                    let u = fb.ins().bor(x3, x4);
                    let sh = fb.ins().ishl_imm(u, 3);
                    let tagged = fb.ins().bor_imm(sh, TAG_INT as i64);
                    fb.def_var(vars[d], tagged);
                }
                OpCode::BitwiseXor(d, a, b) => {
                    let x1 = fb.use_var(vars[a]);
                    let x2 = fb.use_var(vars[b]);
                    let x3 = fb.ins().ushr_imm(x1, 3);
                    let x4 = fb.ins().ushr_imm(x2, 3);
                    let u = fb.ins().bxor(x3, x4);
                    let sh = fb.ins().ishl_imm(u, 3);
                    let tagged = fb.ins().bor_imm(sh, TAG_INT as i64);
                    fb.def_var(vars[d], tagged);
                }
                OpCode::BitwiseNot(d, s) => {
                    let x1 = fb.use_var(vars[s]);
                    let x3 = fb.ins().ushr_imm(x1, 3);
                    let u = fb.ins().bnot(x3);
                    let sh = fb.ins().ishl_imm(u, 3);
                    let tagged = fb.ins().bor_imm(sh, TAG_INT as i64);
                    fb.def_var(vars[d], tagged);
                }
                OpCode::Less(d, a, b) => {
                    let a_tag = fb.use_var(vars[a]);
                    let b_tag = fb.use_var(vars[b]);
                    let a_val = fb.ins().ushr_imm(a_tag, 3);
                    let b_val = fb.ins().ushr_imm(b_tag, 3);

                    let b1 = fb.ins().icmp(IntCC::SignedLessThan, a_val, b_val);
                    let tagged = tag_bool(&mut fb, b1);
                    fb.def_var(vars[d], tagged);
                }
                OpCode::LessEqual(d, a, b) => {
                    let a_tag = fb.use_var(vars[a]);
                    let b_tag = fb.use_var(vars[b]);
                    let a_val = fb.ins().ushr_imm(a_tag, 3);
                    let b_val = fb.ins().ushr_imm(b_tag, 3);
                    let b1 = fb.ins().icmp(IntCC::SignedLessThanOrEqual, a_val, b_val);
                    let tagged = tag_bool(&mut fb, b1);
                    fb.def_var(vars[d], tagged);
                }

                OpCode::Greater(d, a, b) => {
                    let a_tag = fb.use_var(vars[a]);
                    let b_tag = fb.use_var(vars[b]);
                    let a_val = fb.ins().ushr_imm(a_tag, 3);
                    let b_val = fb.ins().ushr_imm(b_tag, 3);
                    let b1 = fb.ins().icmp(IntCC::SignedGreaterThan, a_val, b_val);
                    let tagged = tag_bool(&mut fb, b1);
                    fb.def_var(vars[d], tagged);
                }

                OpCode::GreaterEqual(d, a, b) => {
                    let a_tag = fb.use_var(vars[a]);
                    let b_tag = fb.use_var(vars[b]);
                    let a_val = fb.ins().ushr_imm(a_tag, 3);
                    let b_val = fb.ins().ushr_imm(b_tag, 3);
                    let b1 = fb.ins().icmp(IntCC::SignedGreaterThanOrEqual, a_val, b_val);
                    let tagged = tag_bool(&mut fb, b1);
                    fb.def_var(vars[d], tagged);
                }

                OpCode::Equal(d, a, b) => {
                    let a_tag = fb.use_var(vars[a]);
                    let b_tag = fb.use_var(vars[b]);
                    let a_val = fb.ins().ushr_imm(a_tag, 3);
                    let b_val = fb.ins().ushr_imm(b_tag, 3);
                    let b1 = fb.ins().icmp(IntCC::Equal, a_val, b_val);
                    let tagged = tag_bool(&mut fb, b1);
                    fb.def_var(vars[d], tagged);
                }
                OpCode::NotEqual(d, a, b) => {
                    let a_tag = fb.use_var(vars[a]);
                    let b_tag = fb.use_var(vars[b]);
                    let a_val = fb.ins().ushr_imm(a_tag, 3);
                    let b_val = fb.ins().ushr_imm(b_tag, 3);
                    let b1 = fb.ins().icmp(IntCC::NotEqual, a_val, b_val);
                    let tagged = tag_bool(&mut fb, b1);
                    fb.def_var(vars[d], tagged);
                }
                OpCode::JumpIfFalse(r, offset) => {
                    let cond_tag = fb.use_var(vars[r]);
                    let cond_val = fb.ins().ushr_imm(cond_tag, 3);
                    let zero = fb.ins().iconst(I64, 0);
                    let cond_b1 = fb.ins().icmp(IntCC::NotEqual, cond_val, zero);
                    let tgt_idx = pc + 1 + offset as usize;
                    let target = if tgt_idx == code_len {
                        exit_block
                    } else {
                        blocks[tgt_idx]
                    };
                    let next = blocks[pc + 1];
                    fb.ins().brif(cond_b1, next, &[], target, &[]);
                    let nnn = pc + 1 + offset;
                    preds[nnn].push(pc);

                    preds[pc + 1].push(pc);
                    is_terminator = true;
                }

                OpCode::Jump(off) => {
                    let tgt_idx = pc + 1 + off as usize;
                    let target = if tgt_idx == code_len {
                        exit_block
                    } else {
                        blocks[tgt_idx]
                    };
                    fb.ins().jump(target, &[]);
                    let nnn = pc + 1 + off;
                    preds[nnn].push(pc);
                    is_terminator = true;
                }
                OpCode::Loop(back) => {
                    let target = blocks[pc + 1 - back as usize];
                    fb.ins().jump(target, &[]);
                    let nnn = pc + 1 - back;
                    preds[nnn].push(pc);
                    is_terminator = true;
                }

                OpCode::GetLocal(d, slot) => {
                    let v = fb.use_var(vars[slot]);
                    fb.def_var(vars[d], v);
                }
                OpCode::SetLocal(src, slot) => {
                    let v = fb.use_var(vars[src]);
                    fb.def_var(vars[slot], v);
                }
                OpCode::Call(dest, callee_reg, _argc) => {
                    let rel = fb.ins().iconst(I64, callee_reg as i64);
                    let abs = fb.ins().iadd(base, rel);

                    let callee_val = fb.use_var(vars[callee_reg]);
                    fb.ins().call(store_reg_ref, &[vm_ptr, abs, callee_val]);

                    let raw_arg = fb.use_var(vars[callee_reg + 1]);
                    let call = fb.ins().call(call_raw_ref, &[vm_ptr, base, abs, raw_arg]);
                    let rv = fb.inst_results(call)[0];
                    fb.def_var(vars[dest], rv);
                }
                OpCode::NewArray(d, len_reg) => {
                    let len_val = fb.use_var(vars[len_reg]);
                    let call = fb.ins().call(new_array_ref, &[vm_ptr, len_val]);
                    let arr = fb.inst_results(call)[0];
                    fb.def_var(vars[d], arr);
                }
                OpCode::GetIndex(d, arr, idx) => {
                    let arr_val = fb.use_var(vars[arr]);
                    let idx_val = fb.use_var(vars[idx]);
                    let call = fb.ins().call(array_get_ref, &[vm_ptr, arr_val, idx_val]);
                    let element = fb.inst_results(call)[0];
                    fb.def_var(vars[d], element);
                }
                OpCode::SetIndex(arr, idx, val_reg) => {
                    let arr_val = fb.use_var(vars[arr]);
                    let idx_val = fb.use_var(vars[idx]);
                    let val = fb.use_var(vars[val_reg]);
                    fb.ins()
                        .call(array_set_ref, &[vm_ptr, arr_val, idx_val, val]);
                }
                OpCode::Print(r) => {
                    let rel = fb.ins().iconst(I64, r as i64);
                    let abs = fb.ins().iadd(base, rel);
                    fb.ins().call(print_ref, &[vm_ptr, abs]);
                }

                OpCode::True(d) => {
                    let raw1 = fb.ins().iconst(I64, 1);
                    let tagged = tag_bool(&mut fb, raw1);
                    fb.def_var(vars[d], tagged);
                }
                OpCode::False(d) => {
                    let raw0 = fb.ins().iconst(I64, 0);
                    let tagged = tag_bool(&mut fb, raw0);
                    fb.def_var(vars[d], tagged);
                }
                OpCode::None(d) => {
                    let zero = fb.ins().iconst(I64, 0);
                    fb.def_var(vars[d], zero);
                }

                OpCode::Return(r) => {
                    let rv = fb.use_var(vars[r]);
                    fb.ins().return_(&[rv]);
                    is_terminator = true;
                }

                OpCode::GetGlobal(dest, ref name) => {
                    let slice: &'static [u8] = *GLOBAL_NAMES
                        .lock()
                        .unwrap()
                        .get(name.as_str())
                        .expect("global not interned");

                    let ptr_const = fb.ins().iconst(ptr_ty, slice.as_ptr() as i64);
                    let len_const = fb.ins().iconst(I64, slice.len() as i64);

                    let call = fb
                        .ins()
                        .call(get_global_ref, &[vm_ptr, ptr_const, len_const]);
                    let returned = fb.inst_results(call)[0];
                    fb.def_var(vars[dest], returned);
                }

                OpCode::SetGlobal(src, ref name) => {
                    let slice: &'static [u8] = *GLOBAL_NAMES
                        .lock()
                        .unwrap()
                        .get(name.as_str())
                        .expect("global not interned");

                    let ptr_const = fb.ins().iconst(ptr_ty, slice.as_ptr() as i64);
                    let len_const = fb.ins().iconst(I64, slice.len() as i64);
                    let val = fb.use_var(vars[src]);

                    fb.ins()
                        .call(set_global_ref, &[vm_ptr, ptr_const, len_const, val]);
                }

                OpCode::IncLoopIfLess(idx, limit, target) => {
                    let header_blk = blocks[target];
                    let exit_blk = blocks[pc + 1];

                    let idx_tag = fb.use_var(vars[idx]);
                    let lim_tag = fb.use_var(vars[limit]);
                    let idx_raw = fb.ins().ushr_imm(idx_tag, 3);
                    let lim_raw = fb.ins().ushr_imm(lim_tag, 3);

                    let cond = fb.ins().icmp(IntCC::SignedLessThan, idx_raw, lim_raw);

                    let inc_raw = fb.ins().iadd_imm(idx_raw, 1);
                    let inc_shift = fb.ins().ishl_imm(inc_raw, 3);
                    let inc_tag = fb.ins().bor_imm(inc_shift, TAG_INT as i64);
                    fb.def_var(vars[idx], inc_tag);

                    fb.ins().brif(cond, header_blk, &[], exit_blk, &[]);

                    is_terminator = true;
                }

                _ => {}
            }
            if !is_terminator {
                if let Some(&next_block) = blocks.get(pc + 1) {
                    fb.ins().jump(next_block, &[]);
                } else {
                    fb.ins().jump(exit_block, &[]);
                }
            }
            for &j in &seal_at[pc] {
                fb.seal_block(blocks[j]);
            }
        }
        fb.switch_to_block(exit_block);
        let rvals = &[fb.ins().iconst(I64, 0)];
        fb.ins().return_(rvals);
        fb.seal_block(exit_block);

        fb.seal_all_blocks();
        fb.finalize();

        self.module.define_function(func_id, &mut ctx).unwrap();
        self.module.clear_context(&mut ctx);
        self.module.finalize_definitions();
        let ptr = self.module.get_finalized_function(func_id);
        fobj.jitted = Some(ptr as *const u8);
    }
}

lazy_static! {
    pub static ref GLOBAL_NAMES: std::sync::Mutex<HashMap<&'static str, &'static [u8]>> =
        std::sync::Mutex::new(HashMap::new());
}
