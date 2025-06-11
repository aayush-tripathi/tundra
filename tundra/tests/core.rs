// tests/test_suite.rs

use std::cell::RefCell;
use std::rc::Rc;

use tundra::bytecode::chunk::Chunk;
use tundra::bytecode::value::Value;
use tundra::compiler::compiler::Compiler;
use tundra::compiler::register::RegisterAllocator;
use tundra::lexer::scanner::Scanner;

/// Helper to compile source and return whether it compiled successfully.
fn compile_src(src: &str) -> bool {
    let tokens = Scanner::new(src.to_string()).scan_tokens();
    let chunk = Rc::new(RefCell::new(Chunk::new()));
    let mut comp = Compiler::new(chunk.clone());
    comp.parser.tokens = tokens;
    comp.parser.current_idx = 0;
    comp.parser.advance_token();
    comp.compile(src)
}

// ---------- RegisterAllocator tests ----------

#[test]
fn reg_alloc_store_and_retrieve() {
    let mut alloc = RegisterAllocator::new();
    let r = alloc.allocate(Value::int(42)).expect("should allocate");
    assert_eq!(*alloc.get_value(r), Value::int(42));
}

#[test]
fn reg_alloc_last_allocated_and_free() {
    let mut alloc = RegisterAllocator::new();
    let r1 = alloc.allocate(Value::int(1)).unwrap();
    let r2 = alloc.allocate(Value::int(2)).unwrap();
    assert_eq!(alloc.last_allocated(), Some(r2));
    alloc.free(r2);
    assert_eq!(alloc.last_allocated(), Some(r1));
}

#[test]
fn reg_alloc_reuse_freed() {
    let mut alloc = RegisterAllocator::new();
    let r = alloc.allocate(Value::none()).unwrap();
    alloc.free(r);
    let r2 = alloc.allocate(Value::none()).unwrap();
    assert_eq!(r2, r, "freed register should be reused first");
}

#[test]
fn reg_alloc_reserve() {
    let mut alloc = RegisterAllocator::new();
    // reserve register 0
    alloc.reserve(0);
    // now allocate once; should never get 0
    let r = alloc.allocate(Value::none()).unwrap();
    assert_ne!(r, 0, "reserved register 0 should not be allocated");
}

#[test]
fn reg_alloc_out_of_registers() {
    let mut alloc = RegisterAllocator::new();
    // exhaust all 1024 registers
    let mut regs = Vec::with_capacity(1024);
    for _ in 0..1024 {
        regs.push(alloc.allocate(Value::none()).unwrap());
    }
    // next allocation must error
    assert!(alloc.allocate(Value::none()).is_err());
    // free one and allocate again
    alloc.free(regs[512]);
    assert!(alloc.allocate(Value::none()).is_ok());
}

// ---------- Basic Tundra code compilation tests ----------

#[test]
fn compile_simple_print_sum() {
    assert!(compile_src("print(1+2)"));
}

#[test]
fn compile_var_and_print() {
    let src = "var x = 7\nprint(x)";
    assert!(compile_src(src));
}

#[test]
fn compile_syntax_error() {
    // missing '=' should fail
    assert!(!compile_src("var x 7"));
}

#[test]
fn compile_if_else() {
    let code = r#"if(true):
    print(5)
else:
    print(6)"#;
    assert!(compile_src(code));
}

#[test]
fn compile_for_loop() {
    let code = r#"for i in range(3):
    print(i)"#;
    assert!(compile_src(code));
}

#[test]
fn compile_while_loop() {
    let code = r#"var i = 0
while(i < 2):
    print(i)
    i = i + 1"#;
    assert!(compile_src(code));
}
