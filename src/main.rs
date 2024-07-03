use bytecode::{chunk::Chunk, debug, opcode::OpCode, value::Value};
use compiler::register::RegisterAllocator;
use lexer::{scanner::Scanner, token::TokenType};
use vm::vm::VM;

pub mod bytecode;
pub mod lexer;
pub mod vm;
pub mod compiler;


fn main() {
    let mut chunk = Chunk::new_chunk();

 
    /*chunk.write(OpCode::LoadConstant(0, Value::int(3)), 1); 
    chunk.write(OpCode::LoadConstant(1, Value::int(4)), 2); 
    chunk.write(OpCode::Negate(1,1), 3); 
    chunk.write(OpCode::Divide(2,1,0), 4); 
    chunk.write(OpCode::Pop(1),4);
    chunk.write(OpCode::Return(2), 5);
    let mut vm = VM::new(chunk);
    vm.run();
    let source_code = r#"// This is NOT a comment OR IS IT &^^^^
    # This is another comment
    var x = 42
    var y = x + 3.14
    print(x, y)
    x++
    x--
    x //= y
    if x > y {
        print("x is greater")
    } else {
        print("y is greater")
    }"#;
    let mut scanner = Scanner::new(source_code.to_string());
let tokens = scanner.scan_tokens();

for token in tokens {
    println!("{:?}", token);*/
    let mut allocator = RegisterAllocator::new();
    println!("Initial state of registers:");
    for reg in allocator.free_registers.iter() {
        println!("Register {}: {:?}", reg.index, reg);
    }
    let reg1 = allocator.allocate(Value:: (42)).unwrap();
    let reg2 = allocator.allocate(Value::int(100)).unwrap();
    let reg3 = allocator.allocate(Value::int(7)).unwrap();

    println!("\nAfter allocating three registers with values:");
    for reg in allocator.free_registers.iter() {
        println!("Register {}: {:?}", reg.index, reg);
    }
    allocator.free(reg2);
    println!("\nAfter freeing one register:");
    for reg in allocator.free_registers.iter() {
        println!("Register {}: {:?}", reg.index, reg);
    }


}

fn test_lexer() {
    let source_code = r#"
        fn fib(n) {
            if n <= 1 {
                return n
            }
            return fib(n - 1) + fib(n - 2)
        }

        let num = 10
        let result = fib(num)
        print(result)
    "#;

    let mut scanner = Scanner::new(source_code.to_string());

    loop {
        let token = scanner.scan_token();

        match token.token {
            TokenType::EOF => {
                println!("End of file reached.");
                break;
            }
            _ => println!("{:?}", token),
        }
    }
}
