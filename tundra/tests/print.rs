use std::{cell::RefCell, rc::Rc};
use tundra::{
    bytecode::chunk::Chunk, compiler::compiler::Compiler, compiler::parser::Parser,
    lexer::scanner::Scanner,
};

fn compile_src(src: &str) -> bool {
    let tokens = Scanner::new(src.to_string()).scan_tokens();

    let mut chunk = Rc::new(RefCell::new(Chunk::new()));
    let mut comp = Compiler::new(chunk.clone());
    comp.parser.tokens = tokens;
    comp.parser.current_idx = 0;
    comp.parser.advance_token();

    let ok = comp.compile(src);
    ok
}

#[test]
fn print_mid_file_with_newline() {
    let src = r#"
var x = 123
print(x)
var y = x * 2
"#;
    assert!(
        compile_src(src),
        "Expected normal print in the middle of a file to succeed"
    );
}

#[test]
fn print_at_eof_without_explicit_newline() {
    let src = "print(42)";
    assert!(
        compile_src(src),
        "Trailing print() at EOF (no final '\\n') should now compile"
    );
}

#[test]
fn print_followed_by_whitespace_only() {
    let src = "print(99)   \t  ";
    assert!(
        compile_src(src),
        "print() with only spaces/tabs after it (but before EOF) should still compile"
    );
}

#[test]
fn multiple_statements_on_one_line_after_print_rejected() {
    let src = "print(1) var a = 5";
    assert!(
        !compile_src(src),
        "print() followed immediately by another statement on same line should error"
    );
}

#[test]
fn invalid_followup_expression_after_print() {
    let src = "print(3) 3 + 4";
    assert!(
        !compile_src(src),
        "print() followed by an expression on same line should still be rejected"
    );
}

#[test]
fn blank_lines_and_trailing_print() {
    let src = r#"
print(7)

"#;
    assert!(
        compile_src(src),
        "print() with subsequent blank lines should compile"
    );
}

#[test]
fn print_in_function_body_at_eof() {
    let src = r#"
fun foo():
    print(5)
"#;
    assert!(
        compile_src(src),
        "print() as last line inside an indented block at EOF should compile"
    );
}
