use std::{cell::RefCell, env, fs, path::Path, process, rc::Rc};

use tundra::{
    bytecode::chunk::Chunk,
    bytecode::debug::disassemble_chunk,
    compiler::compiler::Compiler,
    lexer::scanner::Scanner,
    vm::{interpretresult::InterpretResult, vm::VM},
};

fn print_usage_and_exit() -> ! {
    eprintln!("Usage: tundra <script>.tundra [--debug]");
    process::exit(1);
}

fn main() {
    let mut args = env::args();
    let _ = args.next(); // program name
    let script = match args.next() {
        Some(s) => s,
        None => print_usage_and_exit(),
    };

    let path = Path::new(&script);
    if path.extension().and_then(|e| e.to_str()) != Some("tundra") {
        eprintln!("Error: file must have .tundra extension");
        print_usage_and_exit();
    }

    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading {}: {}", script, e);
            process::exit(1);
        }
    };

    // Compile
    let chunk = Rc::new(RefCell::new(Chunk::new()));
    let mut compiler = Compiler::new(chunk.clone());
    if !compiler.compile(&source) {
        eprintln!("Compile error");
        process::exit(1);
    }

    //  bytecode dump
    let debug = args.any(|a| a == "--debug");
    if debug {
        disassemble_chunk(&chunk.borrow(), &format!("== {} ==", script));
    }

    // interpreter only run
    let mut vm = VM::new_interpreter_only(chunk, Box::leak(Box::new(std::io::stdout())));
    match vm.run() {
        InterpretResult::Ok => {}
        InterpretResult::CompileError => {
            eprintln!("Compile error");
            process::exit(1);
        }
        InterpretResult::RuntimeError => {
            eprintln!("Runtime error");
            process::exit(1);
        }
    }
}
