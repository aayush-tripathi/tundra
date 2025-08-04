// src/vm/vm_helpers.rs

use super::vm::{JIT_CTX, VM};
use crate::bytecode::value::{Value, ValueType};
use std::panic;

/// Integer exponentiation 
#[no_mangle]
pub extern "C" fn tundra_pow_i64(base: i64, exp: i64) -> i64 {
    if exp < 0 {
        panic!("Negative exponent {} for integer power", exp);
    }
    let mut result = 1i64;
    for _ in 0..(exp as u32) {
        result = result
            .checked_mul(base)
            .unwrap_or_else(|| panic!("Overflow computing {}**{}", base, exp));
    }
    result
}

/// Compute base.pow(exp) % modulo
pub extern "C" fn tundra_pow_i64_mod(base: i64, exp: i64, modu: i64) -> i64 {
    let mut result = 1i64;
    let mut b = base.rem_euclid(modu);
    let mut e = if exp < 0 { 0 } else { exp as u64 };
    while e > 0 {
        if e & 1 == 1 {
            result = (result.wrapping_mul(b)).rem_euclid(modu);
        }
        b = (b.wrapping_mul(b)).rem_euclid(modu);
        e >>= 1;
    }
    result.rem_euclid(modu)
}
#[no_mangle]
pub extern "C" fn tundra_pow(base: i64, exp: i64, modu: Option<i64>) -> i64 {
    if let Some(m) = modu {
        tundra_pow_i64_mod(base, exp, m)
    } else {
        tundra_pow_i64(base, exp)
    }
}
/// Unified invoke for both native and user functions.
#[no_mangle]
pub extern "C" fn tundra_invoke(vm_ptr: i64, base: i64, callee_slot: i64, argc: i64) -> i64 {
    let vm = unsafe { &mut *(vm_ptr as *mut VM) };
    vm.invoke_from_jit(base as usize, callee_slot as usize, argc as usize)
}

/// Allocate a new array: returns a register index (i64).
#[no_mangle]
pub extern "C" fn tundra_new_array(vm_ptr: i64, length: i64) -> i64 {
    let vm = unsafe { &mut *(vm_ptr as *mut VM) };
    let slot = vm.new_array_jit(length as usize);
    vm.registers[slot].as_i64()
}
/// Array read:
#[no_mangle]
pub extern "C" fn tundra_array_get(vm_ptr: i64, arr_slot: i64, idx: i64) -> i64 {
    let vm = unsafe { &mut *(vm_ptr as *mut VM) };
    let slot = vm.array_get_jit(arr_slot as usize, idx as usize);
    vm.registers[slot].as_i64()
}

/// Array write:
#[no_mangle]
pub extern "C" fn tundra_array_set(vm_ptr: i64, arr_slot: i64, idx: i64, val_slot: i64) {
    let vm = unsafe { &mut *(vm_ptr as *mut VM) };
    vm.array_set_jit(arr_slot as usize, idx as usize, val_slot as usize);
}

/// Print helper:
#[no_mangle]
pub extern "C" fn tundra_print(vm_ptr: i64, val_slot: i64) {
    let vm = unsafe { &mut *(vm_ptr as *mut VM) };
    vm.print_jit(val_slot as usize);
}

#[no_mangle]
pub extern "C" fn tundra_get_global(vm_ptr: *mut VM, name_ptr: *const u8, len: i64) -> i64 {
    let name = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(name_ptr, len as usize))
    };
    eprintln!("[jit] tundra_get_global looking up `{}`…", name);
    let vm = unsafe { &mut *vm_ptr };
    eprintln!("[jit] vm.globals contains:");
    for k in vm.globals.keys() {
        eprintln!("      – `{}`", k);
    }
    let val = vm
        .globals
        .get(name)
        .unwrap_or_else(|| panic!("[jit] undefined global `{}`", name))
        .clone();
    val.as_i64()
}

/// Set a global from JIT’d code
#[no_mangle]
pub extern "C" fn tundra_set_global(vm_ptr: *mut VM, name_ptr: *const u8, len: i64, raw: i64) {
    let name = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(name_ptr, len as usize))
    };
    let vm = unsafe { &mut *vm_ptr };

    vm.globals.insert(name, unsafe { Value::from_i64(raw) });
}

#[no_mangle]
pub unsafe extern "C" fn tundra_apply_variadic(
    fn_ptr: *const u8,
    vm_ptr: i64,
    base: i64,
    argv: *const i64,
    argc: i64,
) -> i64 {
    let args = std::slice::from_raw_parts(argv, argc as usize);
    match argc {
        0 => {
            let fp: extern "C" fn(i64, i64) -> i64 = core::mem::transmute(fn_ptr);
            fp(vm_ptr, base)
        }
        1 => {
            let fp: extern "C" fn(i64, i64, i64) -> i64 = core::mem::transmute(fn_ptr);
            fp(vm_ptr, base, args[0])
        }
        2 => {
            let fp: extern "C" fn(i64, i64, i64, i64) -> i64 = core::mem::transmute(fn_ptr);
            fp(vm_ptr, base, args[0], args[1])
        }
        3 => {
            let fp: extern "C" fn(i64, i64, i64, i64, i64) -> i64 = core::mem::transmute(fn_ptr);
            fp(vm_ptr, base, args[0], args[1], args[2])
        }
        _ => panic!(">4-arg call not yet wired up in tundra_apply_variadic"),
    }
}

#[no_mangle]
pub extern "C" fn tundra_call_raw(vm_ptr: i64, base: i64, callee: i64, raw_arg: i64) -> i64 {
    let vm = unsafe { &mut *(vm_ptr as *mut VM) };
    let f_rc = match &vm.registers[callee as usize].value {
        ValueType::Function(rc) => rc.clone(),
        _ => panic!("tundra_call_raw on non-function"),
    };

    {
        let mut fobj = f_rc.borrow_mut();
        if fobj.jitted.is_none() {
            JIT_CTX.lock().unwrap().compile_function(&mut *fobj);
        }
    }
    let ptr = f_rc.borrow().jitted.expect("compiled ptr");
    let fp: extern "C" fn(i64, i64, i64) -> i64 = unsafe { std::mem::transmute(ptr) };
    fp(vm_ptr, base, raw_arg)
}

/// Overwrite `vm.registers[slot]`
#[no_mangle]
pub extern "C" fn tundra_store_register(vm_ptr: i64, slot: i64, raw: i64) {
    use crate::bytecode::value::Value;
    let vm = unsafe { &mut *(vm_ptr as *mut VM) };
    vm.registers[slot as usize] = unsafe { Value::from_i64(raw) };
}
