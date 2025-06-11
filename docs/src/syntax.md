# Syntax Overview

Tundra uses a clean, Python‐inspired syntax with significant (indentation) whitespace and familiar control constructs. Below is the high‐level grammar:

```bnf
<program>        ::= ( <simple_stmt> | <compound_stmt> )* EOF

<simple_stmt>    ::= <small_stmt> NEWLINE
<small_stmt>     ::= <var_decl>
                   | <expression_stmt>
                   | <print_stmt>

<compound_stmt>  ::= <if_stmt>
                   | <for_stmt>
                   | <while_stmt>
                   | <fun_decl>
                   | <class_decl>

<suite>          ::= <simple_stmt>
                   | NEWLINE INDENT
                       ( <simple_stmt> | <compound_stmt> )+
                     DEDENT

<var_decl>       ::= "var" IDENTIFIER [ "=" <expression> ]

<fun_decl>       ::= "fun" IDENTIFIER "(" [ <parameters> ] ")" ":" <suite>

<class_decl>     ::= "class" IDENTIFIER ":" <suite>

<if_stmt>        ::= "if" "(" <expression> ")" ":" <suite>
                   [ "else" ":" <suite> ]

<for_stmt>       ::= "for" IDENTIFIER "in" <expression> ":" <suite>

<while_stmt>     ::= "while" "(" <expression> ")" ":" <suite>

<print_stmt>     ::= "print" "(" <expression> ")"

<expression_stmt>::= <expression>

<expression>     ::= <assignment>

<assignment>     ::= IDENTIFIER "=" <expression>
                   | <logic_or>

<logic_or>       ::= <logic_and> { "or"  <logic_and> }

<logic_and>      ::= <equality> { "and" <equality> }

<equality>       ::= <comparison> { ( "==" | "!=" ) <comparison> }

<comparison>     ::= <term> { ( "<" | "<=" | ">" | ">=" ) <term> }

<term>           ::= <factor> { ( "+" | "-" ) <factor> }

<factor>         ::= <unary> { ( "*" | "/" | "//" | "%" ) <unary> }

<unary>          ::= ( "!" | "-" ) <unary>
                   | <primary>

<primary>        ::= <atom> { <trailer> }

<atom>           ::= IDENTIFIER
                   | INT_LITERAL
                   | FLOAT_LITERAL
                   | STRING_LITERAL
                   | CHAR_LITERAL
                   | BOOL_LITERAL
                   | "none"
                   | "(" <expression> ")"
                   | <array_literal>

<array_literal>  ::= "[" [ <expression> { "," <expression> } ] "]"

<trailer>        ::= "(" [ <argument_list> ] ")"
                   | "." IDENTIFIER
                   | "[" <expression> "]"

<parameters>     ::= IDENTIFIER { "," IDENTIFIER }

<argument_list>  ::= <expression> { "," <expression> }

```
- Statements end with a newline; compound statements introduce a suite indented one level deeper.

- Expressions follow usual precedence (assignment, or/and, equality, comparison, term, factor, unary, exponent, call, primary).

- Identifiers: start with a letter or underscore, followed by letters, digits, or underscores.

- Block structure: changing indentation at the start of a line emits Indent/Dedent tokens; mixing tabs/spaces is disallowed.

## Sample Code

```tundra
var x = 10
fun greet(name):
    print("Hello, " + name)

greet("World")
```
### Disassembled Bytecode

```bytecode
== == Disassembled Bytecode == ==
0000 Line    2    LoadConstant(0, Value { value: Int(10) })
0001 Line    2    DefineGlobal(0, "x")
0002 Line    6    LoadConstant(0, Value { value: Function(RefCell { value: FunctionObject { name: "greet", arity: 1, chunk: RefCell { value: Chunk { code: [LoadConstant(1, Value { value: String("\"Hello, \"") }), Move(2, 1), GetLocal(3, 0), Add(4, 2, 3), Print(4), Return(0)], constants: [], lines: [4, 4, 4, 4, 4, 6], max_register: 5 } }, jitted: None } }) })
0003 Line    6    DefineGlobal(0, "greet")
0004 Line    6    GetGlobal(0, "greet")
0005 Line    6    LoadConstant(1, Value { value: String("\"World\"") })
0006 Line    6    Call(2, 0, 1)
0007 Line    6    Pop(2)
0008 Line    7    Return(2)
```
### Output

""Hello, ""World""