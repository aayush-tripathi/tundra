//src/compiler/compiler.rs
use crate::{bytecode::value::FunctionObject, jit::GLOBAL_NAMES};

use super::parser::{get_rule, Local, Parser, Precedence};
use super::register::RegisterAllocator;
#[allow(dead_code)]
use crate::{
    bytecode::{chunk::Chunk, opcode::OpCode, value::Value},
    lexer::{
        self,
        scanner::Scanner,
        token::{Token, TokenType},
    },
};
use std::cell::RefCell;
use std::rc::Rc;
pub struct LoopContext {
    start_pc: usize,
    breaks: Vec<usize>,
    continues: Vec<usize>,
}

pub struct Compiler {
    compiling_chunk: Rc<RefCell<Chunk>>,
    pub parser: Parser,
    scope_depth: usize,
    locals: Vec<Local>,
    register_allocator: RegisterAllocator,
    last_value_reg: Option<usize>,
    loop_stack: Vec<LoopContext>,
    current_for: Option<(usize, usize)>,
}

impl Compiler {
    pub fn new(chunk: Rc<RefCell<Chunk>>) -> Self {
        Compiler {
            compiling_chunk: chunk,
            parser: Parser::new(),
            scope_depth: 0,
            locals: Vec::new(),
            register_allocator: RegisterAllocator::new(),
            last_value_reg: None,
            loop_stack: Vec::new(),
            current_for: None,
        }
    }

    pub fn compile(&mut self, source: &str) -> bool {
        let tokens = Scanner::new(source.to_string()).scan_tokens();
        self.parser.tokens = tokens;
        self.parser.current_idx = 0;
        self.reset_error();
        self.parser.advance_token();
        while self.match_token(TokenType::Newline) {}
        while !self.match_token(TokenType::EOF) {
            self.declaration();
            while self.match_token(TokenType::Newline) {}
        }
        self.end_compiler();
        let mut max_reg = 0;
        {
            let chunk = self.compiling_chunk.borrow();
            for op in &chunk.code {
                for &r in &op.regs() {
                    max_reg = max_reg.max(r + 1);
                }
            }
        }
        self.compiling_chunk.borrow_mut().max_register = max_reg;
        !self.parser.had_error
    }

    fn declaration(&mut self) {
        if self.match_token(TokenType::Fun) {
            self.fun_declaration();
        } else if self.match_token(TokenType::Var) {
            self.var_declaration();
        } else {
            self.statement();
        }
        if self.parser.had_error {
            self.synchronize();
        }
    }

    fn var_declaration(&mut self) {
        let name = self.consume_identifier("Expect variable name.");
        let init_reg = if self.match_token(TokenType::Equal) {
            self.parse_expression()
        } else {
            let r = self.register_allocator.allocate(Value::none()).unwrap();
            self.emit_none(r);
            r
        };
        self.consume(
            TokenType::Newline,
            "Expect newline after variable declaration.",
        );

        if self.scope_depth > 0 {
            self.locals.push(Local {
                name,
                depth: self.scope_depth,
                initialized: true,
                register: init_reg,
            });
        } else {
            self.emit_define_global(init_reg, name.clone());
            self.register_allocator.free(init_reg);
        }
    }

    fn fun_declaration(&mut self) {
        let name = self.consume_identifier("Expect function name.");
        self.consume(TokenType::LeftParen, "Expect '(' after function name.");
        let mut arity = 0;
        let mut param_names = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                param_names.push(self.consume_identifier("Expect parameter name."));
                arity += 1;
                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.");
        self.consume(TokenType::Colon, "Expect ':' after signature.");
        self.consume(TokenType::Newline, "Expect newline after signature.");
        self.consume(
            TokenType::Indent,
            "Expect indented block for function body.",
        );

        let fn_chunk = Rc::new(RefCell::new(Chunk::new()));
        let mut fn_comp = Compiler::new(fn_chunk.clone());
        std::mem::swap(&mut fn_comp.parser, &mut self.parser);
        fn_comp.scope_depth = self.scope_depth + 1;
        fn_comp.begin_scope();

        for (slot, pname) in param_names.iter().enumerate() {
            fn_comp.locals.push(Local {
                name: pname.clone(),
                depth: fn_comp.scope_depth,
                initialized: true,
                register: slot,
            });
            fn_comp.register_allocator.reserve(slot);
        }

        fn_comp.block();
        fn_comp.emit_return(0);
        std::mem::swap(&mut fn_comp.parser, &mut self.parser);
        {
            let mut max_reg = 0;
            {
                let fc = fn_chunk.borrow();
                for op in &fc.code {
                    for &r in &op.regs() {
                        if r + 1 > max_reg {
                            max_reg = r + 1;
                        }
                    }
                }
            }

            fn_chunk.borrow_mut().max_register = max_reg;
        }

        let func_val = Value::function(FunctionObject {
            name: name.clone(),
            arity,
            chunk: fn_chunk,
            jitted: None,
        });
        let fn_reg = self.register_allocator.allocate(func_val.clone()).unwrap();
        self.emit_load_constant(fn_reg, func_val);
        self.emit_define_global(fn_reg, name);
        self.register_allocator.free(fn_reg);

        while self.match_token(TokenType::Newline) {}
    }

    fn statement(&mut self) {
        match self.parser.current.token {
            TokenType::Print => {
                self.advance();
                self.print_statement();
            }
            TokenType::Return => {
                self.advance();
                self.return_statement();
            }
            TokenType::If => {
                self.advance();
                self.if_statement();
            }
            TokenType::While => {
                self.advance();
                self.while_statement();
            }
            TokenType::For => {
                self.advance();
                self.for_statement();
            }
            TokenType::Indent => {
                self.advance();
                self.block();
            }
            TokenType::Break => {
                self.advance();
                self.break_statement();
            }
            TokenType::Continue => {
                self.advance();
                self.continue_statement();
            }
            _ => self.expression_statement(),
        }
    }

    fn print_statement(&mut self) {
        self.consume(TokenType::LeftParen, "Expect '(' after 'print'.");
        let value_reg = self.parse_expression();
        self.consume(TokenType::RightParen, "Expect ')' after value.");
        self.consume(TokenType::Newline, "Expect newline after print.");
        self.emit_print(value_reg);
        self.register_allocator.free(value_reg);
    }

    fn for_statement(&mut self) {
        let loop_var = self.consume_identifier("Expect loop variable name after 'for'.");
        self.consume(TokenType::In, "Expect 'in' after loop variable.");

        self.begin_scope();
        let i_reg = self.register_allocator.allocate(Value::none()).unwrap();
        self.locals.push(Local {
            name: loop_var.clone(),
            depth: self.scope_depth,
            initialized: true,
            register: i_reg,
        });

        if let Some((start_reg, stop_reg, step_reg, is_simple)) = self.try_parse_range_args() {
            self.current_for = Some((i_reg, step_reg));
            self.emit_set_local(start_reg, i_reg);

            let loop_start = self.compiling_chunk.borrow().code.len();
            self.loop_stack.push(LoopContext {
                start_pc: loop_start,
                breaks: Vec::new(),
                continues: Vec::new(),
            });

            let cond = self.register_allocator.allocate(Value::none()).unwrap();
            self.emit_less(cond, i_reg, stop_reg);
            let exit = self.emit_jump(OpCode::JumpIfFalse(cond, 0));
            self.register_allocator.free(cond);

            self.consume(TokenType::Colon, "Expect ':' after for");
            self.consume(TokenType::Newline, "Expect newline");
            self.consume(TokenType::Indent, "Expect indent");
            while self.match_token(TokenType::Newline) {}
            self.block();

            if is_simple {
                self.emit_inc_loop_if_less(i_reg, stop_reg, loop_start);
            } else {
                self.emit_add(i_reg, i_reg, step_reg);
                self.emit_loop(loop_start);
            }
            self.patch_jump(exit);

            self.current_for = None;
            self.register_allocator.free(start_reg);
            self.register_allocator.free(stop_reg);
            self.register_allocator.free(step_reg);
        } else {
            let count = self.parse_expression();
            self.consume(TokenType::Colon, "Expect ':' after for clauses.");
            self.consume(TokenType::Newline, "Expect newline after ':'.");
            self.emit_set_local(count, i_reg);

            let loop_start = self.compiling_chunk.borrow().code.len();
            self.loop_stack.push(LoopContext {
                start_pc: loop_start,
                breaks: Vec::new(),
                continues: Vec::new(),
            });
            let zero = self.register_allocator.allocate(Value::int(0)).unwrap();
            self.emit_load_constant(zero, Value::int(0));
            let cond = self.register_allocator.allocate(Value::none()).unwrap();
            self.emit_greater(cond, i_reg, zero);
            let exit_jump = self.emit_jump(OpCode::JumpIfFalse(cond, 0));

            self.consume(TokenType::Indent, "Expect indented block after 'for'.");
            while self.match_token(TokenType::Newline) {}
            self.block();

            self.emit_subtract(i_reg, i_reg, zero);
            self.emit_loop(loop_start);
            self.patch_jump(exit_jump);

            self.register_allocator.free(count);
            self.register_allocator.free(zero);
            self.register_allocator.free(cond);
        }

        let ctx = self.loop_stack.pop().unwrap();
        for &slot in &ctx.continues {
            self.patch_jump(slot);
        }
        for &slot in &ctx.breaks {
            self.patch_jump(slot);
        }
        self.end_scope();
    }

    fn return_statement(&mut self) {
        let value_reg = if !self.check(TokenType::Newline) {
            self.parse_expression()
        } else {
            let r = self.register_allocator.allocate(Value::none()).unwrap();
            self.emit_none(r);
            r
        };
        self.consume(TokenType::Newline, "Expect newline after return value.");
        self.emit_return(value_reg);
        self.register_allocator.free(value_reg);
    }

    fn if_statement(&mut self) {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        let cond_reg = self.parse_expression();
        self.consume(TokenType::RightParen, "Expect ')' after if condition.");
        let jump_to_else = self.emit_jump(OpCode::JumpIfFalse(cond_reg, 0));

        self.consume(TokenType::Colon, "Expect ':' after condition.");
        self.consume(TokenType::Newline, "Expect newline after ':'.");
        if self.match_token(TokenType::Indent) {
            self.block();
        } else {
            self.statement();
        }

        if self.match_token(TokenType::Else) {
            let jump_over = self.emit_jump(OpCode::Jump(0));
            self.patch_jump(jump_to_else);
            self.consume(TokenType::Colon, "Expect ':' after 'else'.");
            self.consume(TokenType::Newline, "Expect newline after ':'.");
            if self.match_token(TokenType::Indent) {
                self.block();
            } else {
                self.statement();
            }
            self.patch_jump(jump_over);
        } else {
            self.patch_jump(jump_to_else);
        }

        self.register_allocator.free(cond_reg);
    }

    fn while_statement(&mut self) {
        let loop_start = self.compiling_chunk.borrow().code.len();
        self.loop_stack.push(LoopContext {
            start_pc: loop_start,
            breaks: Vec::new(),
            continues: Vec::new(),
        });

        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.");
        let cond_reg = self.parse_expression();
        self.consume(TokenType::RightParen, "Expect ')' after while condition.");

        let exit_j = self.emit_jump(OpCode::JumpIfFalse(cond_reg, 0));
        self.consume(TokenType::Colon, "Expect ':' after while condition.");
        self.consume(TokenType::Newline, "Expect newline after ':'.");
        if self.match_token(TokenType::Indent) {
            self.block();
        } else {
            self.statement();
        }

        self.emit_loop(loop_start);
        self.patch_jump(exit_j);

        let ctx = self.loop_stack.pop().unwrap();
        for slot in ctx.continues {
            self.patch_jump(slot);
        }
        for slot in ctx.breaks {
            self.patch_jump(slot);
        }
        self.register_allocator.free(cond_reg);
    }

    fn block(&mut self) {
        self.begin_scope();
        while self.match_token(TokenType::Newline) {}

        while !self.check(TokenType::EOF) {
            if self.check(TokenType::Indent) && self.peek_token(0) == TokenType::Dedent {
                self.advance();
                self.advance();
                while self.match_token(TokenType::Newline) {}
                continue;
            }

            if self.check(TokenType::Dedent) {
                let mut look = 0;
                while self.peek_token(look) == TokenType::Newline {
                    look += 1;
                }
                if self.peek_token(look) == TokenType::Indent {
                    self.advance();
                    while self.match_token(TokenType::Newline) {}
                    self.advance();
                    while self.match_token(TokenType::Newline) {}
                    continue;
                }
                break;
            }

            self.declaration();
            while self.match_token(TokenType::Newline) {}
        }

        self.consume(TokenType::Dedent, "Expect dedent after block.");
        self.end_scope();
    }

    fn peek_token(&self, n: usize) -> TokenType {
        self.parser
            .tokens
            .get(self.parser.current_idx + n)
            .map(|t| t.token)
            .unwrap_or(TokenType::EOF)
    }

    fn expression_statement(&mut self) {
        let r = self.parse_expression();
        self.consume(TokenType::Newline, "Expect newline after expression.");
        self.emit_pop(r);
        self.register_allocator.free(r);
    }

    fn parse_expression(&mut self) -> usize {
        self.parse_precedence(Precedence::Assignment)
    }
    pub fn array_literal(&mut self, _can_assign: bool) {
        while self.match_token(TokenType::Newline) {}
        if self.match_token(TokenType::Indent) {
            while self.match_token(TokenType::Newline) {}
        }

        let mut elem_regs = Vec::new();
        if !self.check(TokenType::RightBracket) {
            loop {
                elem_regs.push(self.parse_expression());

                while self.match_token(TokenType::Newline) {}
                if !self.match_token(TokenType::Comma) {
                    break;
                }

                while self.match_token(TokenType::Newline) {}
            }
        }

        while self.match_token(TokenType::Newline) {}

        let _ = self.match_token(TokenType::Dedent);

        self.consume(TokenType::RightBracket, "Expect ']' after array literal.");

        let len = elem_regs.len() as i64;
        let len_reg = self.register_allocator.allocate(Value::int(len)).unwrap();
        self.emit_load_constant(len_reg, Value::int(len));

        let dest = self.register_allocator.allocate(Value::none()).unwrap();
        self.emit_byte(OpCode::NewArray(dest, len_reg));

        self.register_allocator.free(len_reg);

        for (i, elem_reg) in elem_regs.into_iter().enumerate() {
            let idx = i as i64;
            let idx_reg = self.register_allocator.allocate(Value::int(idx)).unwrap();
            self.emit_load_constant(idx_reg, Value::int(idx));
            self.emit_byte(OpCode::SetIndex(dest, idx_reg, elem_reg));

            self.register_allocator.free(idx_reg);
            self.register_allocator.free(elem_reg);
        }

        self.last_value_reg = Some(dest);
    }

    pub fn index(&mut self, can_assign: bool) {
        let array_reg = self.register_allocator.last_allocated().unwrap();

        let idx_reg = self.parse_expression();
        self.consume(TokenType::RightBracket, "Expect ']' after index.");

        if can_assign && self.match_token(TokenType::Equal) {
            let val_reg = self.parse_expression();

            self.emit_byte(OpCode::SetIndex(array_reg, idx_reg, val_reg));

            self.register_allocator.free(val_reg);
            self.register_allocator.free(idx_reg);

            self.last_value_reg = Some(array_reg);
        } else {
            let dest = self.register_allocator.allocate(Value::none()).unwrap();

            self.emit_byte(OpCode::GetIndex(dest, array_reg, idx_reg));

            self.register_allocator.free(idx_reg);

            self.last_value_reg = Some(dest);
        }
    }
    fn parse_precedence(&mut self, prec: Precedence) -> usize {
        self.advance();
        let prefix = get_rule(self.parser.previous.token)
            .prefix
            .expect("Expect expression.");
        let can_assign = prec <= Precedence::Assignment;
        prefix(self, can_assign);

        while prec <= get_rule(self.parser.current.token).precedence {
            self.advance();
            let infix = get_rule(self.parser.previous.token)
                .infix
                .expect("Expect infix operator.");
            infix(self, can_assign);
        }

        self.last_value_reg
            .clone()
            .expect("Expression did not produce a value")
    }

    pub fn grouping(&mut self, _can_assign: bool) {
        let r = self.parse_expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }
    pub fn call(&mut self, _can_assign: bool) {
        let callee = self.register_allocator.last_allocated().unwrap();

        let mut arg_regs = Vec::new();
        while !self.check(TokenType::RightParen) {
            arg_regs.push(self.parse_expression());
            if !self.match_token(TokenType::Comma) {
                break;
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after arguments.");

        let dest = self.register_allocator.allocate(Value::none()).unwrap();

        for (i, &raw) in arg_regs.iter().enumerate().rev() {
            let target = callee + 1 + i;
            if raw != target {
                self.emit_byte(OpCode::Move(target, raw));
            }
        }

        self.emit_byte(OpCode::Call(dest, callee, arg_regs.len()));

        for &r in &arg_regs {
            self.register_allocator.free(r);
        }

        self.register_allocator.free(callee);

        self.last_value_reg = Some(dest);
    }

    fn end_compiler(&mut self) {
        let r = self.last_value_reg.unwrap_or(0);
        self.emit_return(r);
    }

    fn emit_byte(&mut self, op: OpCode) {
        self.compiling_chunk
            .borrow_mut()
            .write(op, self.parser.previous.end_line);
    }
    fn emit_jump(&mut self, op: OpCode) -> usize {
        self.emit_byte(op);
        self.compiling_chunk.borrow().code.len() - 1
    }
    fn emit_loop(&mut self, start: usize) {
        let here = self.compiling_chunk.borrow().code.len();

        let back = (here + 1) - start;
        self.emit_byte(OpCode::Loop(back));
    }
    fn patch_jump(&mut self, idx: usize) {
        let target = self.compiling_chunk.borrow().code.len();

        let offset = target - (idx + 1);

        let new_op = match self.compiling_chunk.borrow().code[idx] {
            OpCode::JumpIfFalse(r, _) => OpCode::JumpIfFalse(r, offset),
            OpCode::Jump(_) => OpCode::Jump(offset),
            _ => panic!("Invalid patch target"),
        };
        self.compiling_chunk.borrow_mut().code[idx] = new_op;
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }
    fn end_scope(&mut self) {
        self.scope_depth -= 1;
        while let Some(loc) = self.locals.last() {
            if loc.depth > self.scope_depth {
                let loc = self.locals.pop().unwrap();
                self.register_allocator.free(loc.register);
                self.emit_pop(loc.register);
            } else {
                break;
            }
        }
    }
    pub fn unary(&mut self, _can_assign: bool) {
        let op = self.parser.previous.token;
        let rhs = self.parse_precedence(Precedence::Unary);
        match op {
            TokenType::Minus => self.emit_negate(rhs, rhs),
            TokenType::Bang | TokenType::Not => {
                let tmp = self.register_allocator.allocate(Value::none()).unwrap();
                let zero = self.register_allocator.allocate(Value::none()).unwrap();
                self.emit_load_constant(zero, Value::boolean(false));
                self.emit_equal(tmp, rhs, zero);
                self.register_allocator.free(rhs);
                self.register_allocator.free(zero);
            }
            _ => unreachable!(),
        }
    }

    pub fn prefix_incdec(&mut self, _can_assign: bool) {
        let op = self.parser.previous.token;
        self.variable(false);
        let var_reg = self.register_allocator.last_allocated().unwrap();
        let one = self.register_allocator.allocate(Value::int(1)).unwrap();
        if op == TokenType::PlusPlus {
            self.emit_add(var_reg, var_reg, one);
        } else {
            self.emit_subtract(var_reg, var_reg, one);
        }
    }
    fn break_statement(&mut self) {
        let slot = self.emit_jump(OpCode::Jump(0));

        self.loop_stack
            .last_mut()
            .expect("`break` outside loop")
            .breaks
            .push(slot);

        self.consume(TokenType::Newline, "Expect newline after 'break'.");
    }

    fn continue_statement(&mut self) {
        if let Some((i_reg, step_reg)) = self.current_for {
            self.emit_add(i_reg, i_reg, step_reg);
        }

        let loop_start = self
            .loop_stack
            .last()
            .expect("`continue` outside loop")
            .start_pc;
        self.emit_loop(loop_start);
        self.consume(TokenType::Newline, "Expect newline after 'continue'.");
    }

    pub fn literal_int(&mut self, _can_assign: bool) {
        let s = self.parser.previous.lexeme.trim();
        let v = s
            .parse::<i64>()
            .expect(&format!("Invalid integer literal `{}`", s));
        let r = self
            .register_allocator
            .allocate(Value::int(v))
            .expect("Out of registers");
        self.emit_load_constant(r, Value::int(v));
    }
    pub fn literal_float(&mut self, _can_assign: bool) {
        let s = self.parser.previous.lexeme.trim();
        let v = s
            .parse::<f64>()
            .expect(&format!("Invalid integer literal `{}`", s));
        let r = self
            .register_allocator
            .allocate(Value::float(v))
            .expect("Out of registers");
        self.emit_load_constant(r, Value::float(v));
    }
    pub fn literal_string(&mut self, _can_assign: bool) {
        let s = self.parser.previous.lexeme.clone();
        let r = self
            .register_allocator
            .allocate(Value::string(s.clone()))
            .unwrap();
        self.emit_load_constant(r, Value::string(s));
    }
    pub fn literal_char(&mut self, _can_assign: bool) {
        let ch = self.parser.previous.lexeme.chars().nth(1).unwrap();
        let r = self.register_allocator.allocate(Value::char(ch)).unwrap();
        self.emit_load_constant(r, Value::char(ch));
    }
    pub fn literal_bool(&mut self, _can_assign: bool) {
        let flag = self.parser.previous.token == TokenType::True;
        let r = self
            .register_allocator
            .allocate(Value::boolean(flag))
            .unwrap();
        self.emit_load_constant(r, Value::boolean(flag));
    }
    pub fn literal_none(&mut self, _can_assign: bool) {
        let r = self.register_allocator.allocate(Value::none()).unwrap();
        self.emit_none(r);
        self.register_allocator.free(r);
    }

    pub fn variable(&mut self, can_assign: bool) {
        let name = self.parser.previous.lexeme.clone();

        if let Some(slot) = self.resolve_local(&name) {
            if can_assign && self.match_token(TokenType::Equal) {
                let rhs = self.parse_expression();
                self.emit_set_local(rhs, slot);
                self.last_value_reg = Some(rhs);
            } else {
                let dst = self.register_allocator.allocate(Value::none()).unwrap();
                self.emit_get_local(dst, slot);
            }
            return;
        }

        if can_assign && self.match_token(TokenType::Equal) {
            let rhs = self.parse_expression();
            self.emit_set_global(rhs, name.clone());
            self.last_value_reg = Some(rhs);
            return;
        }

        let getter_reg = self.register_allocator.allocate(Value::none()).unwrap();
        self.emit_get_global(getter_reg, name.clone());

        if can_assign {
            if self.match_token(TokenType::Equal) {
                let rhs = self.parse_expression();
                self.emit_set_global(rhs, name.clone());
                self.register_allocator.free(getter_reg);
            } else if self.match_token(TokenType::PlusEqual) {
                let rhs = self.parse_expression();
                self.emit_add(getter_reg, getter_reg, rhs);
                self.emit_set_global(getter_reg, name.clone());
                self.register_allocator.free(rhs);
            } else if self.match_token(TokenType::MinusEqual) {
                let rhs = self.parse_expression();
                self.emit_subtract(getter_reg, getter_reg, rhs);
                self.emit_set_global(getter_reg, name.clone());
                self.register_allocator.free(rhs);
            } else if self.match_token(TokenType::StarEqual) {
                let rhs = self.parse_expression();
                self.emit_multiply(getter_reg, getter_reg, rhs);
                self.emit_set_global(getter_reg, name.clone());
                self.register_allocator.free(rhs);
            } else if self.match_token(TokenType::SlashEqual) {
                let rhs = self.parse_expression();
                self.emit_divide(getter_reg, getter_reg, rhs);
                self.emit_set_global(getter_reg, name.clone());
                self.register_allocator.free(rhs);
            } else if self.match_token(TokenType::SlashSlashEqual) {
                let rhs = self.parse_expression();
                self.emit_int_divide(getter_reg, getter_reg, rhs);
                self.emit_set_global(getter_reg, name.clone());
                self.register_allocator.free(rhs);
            } else if self.match_token(TokenType::PercentEqual) {
                let rhs = self.parse_expression();
                self.emit_int_divide(getter_reg, getter_reg, rhs);
                self.emit_set_global(getter_reg, name.clone());
                self.register_allocator.free(rhs);
            } else if self.match_token(TokenType::StarStarEqual) {
                let rhs = self.parse_expression();
                self.emit_exponentiate(getter_reg, getter_reg, rhs);
                self.emit_set_global(getter_reg, name.clone());
                self.register_allocator.free(rhs);
            } else if self.match_token(TokenType::AmpersandEqual) {
                let rhs = self.parse_expression();
                self.emit_bitwise_and(getter_reg, getter_reg, rhs);
                self.emit_set_global(getter_reg, name.clone());
                self.register_allocator.free(rhs);
            } else if self.match_token(TokenType::PipeEqual) {
                let rhs = self.parse_expression();
                self.emit_bitwise_or(getter_reg, getter_reg, rhs);
                self.emit_set_global(getter_reg, name.clone());
                self.register_allocator.free(rhs);
            } else if self.match_token(TokenType::CaretEqual) {
                let rhs = self.parse_expression();
                self.emit_bitwise_xor(getter_reg, getter_reg, rhs);
                self.emit_set_global(getter_reg, name.clone());
                self.register_allocator.free(rhs);
            }
        }
        self.last_value_reg = Some(getter_reg);
    }

    fn resolve_local(&self, name: &str) -> Option<usize> {
        for (slot, local) in self.locals.iter().enumerate().rev() {
            if local.name == name && local.depth > 0 {
                return Some(local.register);
            }
        }
        None
    }

    fn try_parse_range_args(&mut self) -> Option<(usize, usize, usize, bool)> {
        if self.check(TokenType::Identifier) && self.parser.current.lexeme == "range" {
            self.advance();
            self.consume(TokenType::LeftParen, "Expect '(' after 'range'.");

            let mut regs = Vec::new();
            regs.push(self.parse_expression());
            while self.match_token(TokenType::Comma) {
                regs.push(self.parse_expression());
            }
            self.consume(TokenType::RightParen, "Expect ')' after range args.");

            let n_args = regs.len();
            let simple = n_args < 3;
            let (start_reg, stop_reg, step_reg) = match regs.len() {
                1 => {
                    let stop = regs[0];

                    let st = self.register_allocator.allocate(Value::int(0)).unwrap();
                    self.emit_load_constant(st, Value::int(0));

                    let sp = self.register_allocator.allocate(Value::int(1)).unwrap();
                    self.emit_load_constant(sp, Value::int(1));
                    (st, stop, sp)
                }
                2 => {
                    let start = regs[0];
                    let stop = regs[1];

                    let sp = self.register_allocator.allocate(Value::int(1)).unwrap();
                    self.emit_load_constant(sp, Value::int(1));
                    (start, stop, sp)
                }
                3 => (regs[0], regs[1], regs[2]),
                _ => {
                    self.error_at_current("range() takes 1 to 3 arguments");
                    return None;
                }
            };

            return Some((start_reg, stop_reg, step_reg, simple));
        }
        None
    }

    pub fn binary(&mut self, _can_assign: bool) {
        let op = self.parser.previous.token;
        let rule = get_rule(op);

        let lhs_orig = self.register_allocator.last_allocated().unwrap();

        let lhs_reg = {
            let lhs_val = self.register_allocator.get_value(lhs_orig).clone();

            let tmp = self.register_allocator.allocate(lhs_val).unwrap();

            self.emit_move(tmp, lhs_orig);
            tmp
        };

        let next_prec = if op == TokenType::StarStar {
            rule.precedence
        } else {
            Precedence::next_rule(rule.precedence)
        };
        let rhs_reg = self.parse_precedence(next_prec);

        let res = self.register_allocator.allocate(Value::none()).unwrap();

        match op {
            TokenType::Plus => self.emit_add(res, lhs_reg, rhs_reg),
            TokenType::Minus => self.emit_subtract(res, lhs_reg, rhs_reg),
            TokenType::Star => self.emit_multiply(res, lhs_reg, rhs_reg),
            TokenType::Slash => self.emit_divide(res, lhs_reg, rhs_reg),
            TokenType::SlashSlash => self.emit_int_divide(res, lhs_reg, rhs_reg),
            TokenType::Percent => self.emit_modulo(res, lhs_reg, rhs_reg),
            TokenType::StarStar => self.emit_exponentiate(res, lhs_reg, rhs_reg),
            TokenType::Caret => self.emit_bitwise_xor(res, lhs_reg, rhs_reg),
            TokenType::Ampersand => self.emit_bitwise_and(res, lhs_reg, rhs_reg),
            TokenType::Pipe => self.emit_bitwise_or(res, lhs_reg, rhs_reg),

            TokenType::BangEqual => {
                self.emit_equal(res, lhs_reg, rhs_reg);
                self.emit_negate(res, res);
            }
            TokenType::EqualEqual => self.emit_equal(res, lhs_reg, rhs_reg),
            TokenType::Greater => self.emit_greater(res, lhs_reg, rhs_reg),
            TokenType::GreaterEqual => self.emit_greater_equal(res, lhs_reg, rhs_reg),
            TokenType::Less => self.emit_less(res, lhs_reg, rhs_reg),
            TokenType::LessEqual => self.emit_less_equal(res, lhs_reg, rhs_reg),
            TokenType::And => self.emit_bitwise_and(res, lhs_reg, rhs_reg),
            TokenType::Or => self.emit_bitwise_or(res, lhs_reg, rhs_reg),
            _ => unreachable!("binary() got {:?}", op),
        };

        self.register_allocator.free(lhs_reg);
        self.register_allocator.free(rhs_reg);

        self.last_value_reg = Some(res);
    }

    fn consume_identifier(&mut self, msg: &str) -> String {
        self.consume(TokenType::Identifier, msg).lexeme.clone()
    }

    fn reset_error(&mut self) {
        self.parser.panic_mode = false;
        self.parser.had_error = false;
    }
    fn synchronize(&mut self) {
        self.parser.panic_mode = false;
        while self.parser.current.token != TokenType::EOF {
            match self.parser.current.token {
                TokenType::Var
                | TokenType::Print
                | TokenType::Return
                | TokenType::If
                | TokenType::While => return,
                _ => self.advance(),
            }
        }
    }
    fn advance(&mut self) {
        self.parser.advance_token();
    }

    fn consume(&mut self, tk: TokenType, msg: &str) -> Token {
        if self.parser.current.token == tk {
            let t = self.parser.current.clone();
            self.advance();
            t
        } else {
            self.error(msg);

            self.parser.current.clone()
        }
    }
    fn match_token(&mut self, tk: TokenType) -> bool {
        if self.parser.current.token == tk {
            self.advance();
            true
        } else {
            false
        }
    }
    fn check(&self, tk: TokenType) -> bool {
        self.parser.current.token == tk
    }
    fn error(&mut self, msg: &str) {
        self.error_at(self.parser.previous.clone(), msg)
    }
    fn error_at_current(&mut self, msg: &str) {
        self.error_at(self.parser.current.clone(), msg)
    }
    fn error_at(&mut self, token: Token, msg: &str) {
        if self.parser.panic_mode {
            return;
        }
        self.parser.panic_mode = true;
        eprint!("[line {}] Error", token.end_line);
        if token.token == TokenType::EOF {
            eprint!(" at end");
        } else {
            eprint!(" at '{}'", token.lexeme);
        }
        eprintln!(": {}", msg);
        self.parser.had_error = true;
    }

    fn emit_load_constant(&mut self, r: usize, v: Value) {
        self.emit_byte(OpCode::LoadConstant(r, v.clone()));
        self.last_value_reg = Some(r);
    }

    pub fn intern(name: &str) -> &'static str {
        use std::collections::hash_map::Entry;

        let mut table = GLOBAL_NAMES.lock().unwrap();

        if let Some((&key, slice)) = table.get_key_value(name) {
            if slice.is_empty() {
                *table.get_mut(key).unwrap() = key.as_bytes();
            }
            return key;
        }

        let leaked: &'static str = Box::leak(name.to_string().into_boxed_str());
        table.insert(leaked, leaked.as_bytes());
        leaked
    }

    fn emit_get_global(&mut self, r: usize, name: String) {
        Self::intern(&name);
        self.emit_byte(OpCode::GetGlobal(r, name));
    }

    fn emit_define_global(&mut self, r: usize, name: String) {
        Self::intern(&name);
        self.emit_byte(OpCode::DefineGlobal(r, name));
    }

    fn emit_set_global(&mut self, r: usize, name: String) {
        Self::intern(&name);
        self.emit_byte(OpCode::SetGlobal(r, name));
    }
    fn emit_get_local(&mut self, d: usize, s: usize) {
        self.emit_byte(OpCode::GetLocal(d, s));
        self.last_value_reg = Some(d);
    }
    fn emit_set_local(&mut self, d: usize, s: usize) {
        self.emit_byte(OpCode::SetLocal(d, s));
        self.last_value_reg = Some(d);
    }
    fn emit_pop(&mut self, r: usize) {
        self.emit_byte(OpCode::Pop(r));
        self.last_value_reg = Some(r);
    }
    fn emit_none(&mut self, r: usize) {
        self.emit_byte(OpCode::None(r));
        self.last_value_reg = Some(r);
    }
    fn emit_add(&mut self, d: usize, a: usize, b: usize) {
        self.emit_byte(OpCode::Add(d, a, b));
        self.last_value_reg = Some(d);
    }
    fn emit_subtract(&mut self, d: usize, a: usize, b: usize) {
        self.emit_byte(OpCode::Subtract(d, a, b));
        self.last_value_reg = Some(d);
    }
    fn emit_multiply(&mut self, d: usize, a: usize, b: usize) {
        self.emit_byte(OpCode::Multiply(d, a, b));
        self.last_value_reg = Some(d);
    }
    fn emit_divide(&mut self, d: usize, a: usize, b: usize) {
        self.emit_byte(OpCode::Divide(d, a, b));
        self.last_value_reg = Some(d);
    }
    fn emit_exponentiate(&mut self, d: usize, a: usize, b: usize) {
        self.emit_byte(OpCode::Exponentiate(d, a, b));
        self.last_value_reg = Some(d);
    }
    fn emit_int_divide(&mut self, d: usize, a: usize, b: usize) {
        self.emit_byte(OpCode::IntDivide(d, a, b));
        self.last_value_reg = Some(d);
    }
    fn emit_modulo(&mut self, d: usize, a: usize, b: usize) {
        self.emit_byte(OpCode::Mod(d, a, b));
        self.last_value_reg = Some(d);
    }
    fn emit_return(&mut self, r: usize) {
        self.emit_byte(OpCode::Return(r));
    }
    fn emit_bitwise_not(&mut self, d: usize, s: usize) {
        self.emit_byte(OpCode::BitwiseNot(d, s));
        self.last_value_reg = Some(d);
    }
    fn emit_negate(&mut self, d: usize, s: usize) {
        self.emit_byte(OpCode::Negate(d, s));
        self.last_value_reg = Some(d);
    }
    fn emit_print(&mut self, r: usize) {
        self.emit_byte(OpCode::Print(r));
    }
    fn emit_equal(&mut self, d: usize, a: usize, b: usize) {
        self.emit_byte(OpCode::Equal(d, a, b));
        self.last_value_reg = Some(d);
    }
    fn emit_greater(&mut self, d: usize, a: usize, b: usize) {
        self.emit_byte(OpCode::Greater(d, a, b));
        self.last_value_reg = Some(d);
    }
    fn emit_less(&mut self, d: usize, a: usize, b: usize) {
        self.emit_byte(OpCode::Less(d, a, b));
        self.last_value_reg = Some(d);
    }
    fn emit_bitwise_and(&mut self, d: usize, a: usize, b: usize) {
        self.emit_byte(OpCode::BitwiseAnd(d, a, b));
        self.last_value_reg = Some(d);
    }
    fn emit_bitwise_or(&mut self, d: usize, a: usize, b: usize) {
        self.emit_byte(OpCode::BitwiseOr(d, a, b));
        self.last_value_reg = Some(d);
    }
    fn emit_bitwise_xor(&mut self, d: usize, a: usize, b: usize) {
        self.emit_byte(OpCode::BitwiseXor(d, a, b));
        self.last_value_reg = Some(d);
    }
    fn emit_greater_equal(&mut self, d: usize, a: usize, b: usize) {
        self.emit_byte(OpCode::GreaterEqual(d, a, b));
        self.last_value_reg = Some(d);
    }
    fn emit_inc_loop_if_less(&mut self, idx_reg: usize, limit_reg: usize, target_ip: usize) {
        self.emit_byte(OpCode::IncLoopIfLess(idx_reg, limit_reg, target_ip));
    }
    fn emit_less_equal(&mut self, d: usize, a: usize, b: usize) {
        self.emit_byte(OpCode::LessEqual(d, a, b));
        self.last_value_reg = Some(d);
    }
    fn emit_not_equal(&mut self, d: usize, a: usize, b: usize) {
        self.emit_byte(OpCode::NotEqual(d, a, b));
        self.last_value_reg = Some(d);
    }
    fn emit_move(&mut self, d: usize, s: usize) {
        self.emit_byte(OpCode::Move(d, s));
        self.last_value_reg = Some(d);
    }
}
