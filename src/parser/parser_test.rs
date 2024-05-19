#![cfg(test)]

use std::io::stderr;
use std::io::Write;

use crate::ast::Statement;
use crate::lexer;
use crate::parser::*;

use self::ast::Program;

#[test]
fn test_let_statements() {
    let input = r"
let x = 5;
let y = 10;
let foobar = 838383;
";

    let lexer = lexer::Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    check_parser_errors(&mut parser);

    assert_eq!(program.statements.len(), 3);
    test_statement(&program.statements[0], "let x = \"no-op\";");
    test_statement(&program.statements[1], "let y = \"no-op\";");
    test_statement(&program.statements[2], "let foobar = \"no-op\";");
}

#[test]
#[should_panic]
fn test_illegal_let_statements() {
    let input = r"
let x 5;
let y;
";

    let lexer = lexer::Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let _program = parser.parse_program();
    check_parser_errors(&mut parser);
}

fn check_parser_errors(parser: &mut Parser) {
    let errors = parser.errors();
    if errors.is_empty() {
        return;
    }
    let mut stderr = stderr();
    writeln!(stderr, "Parser has {} error(s).", errors.len()).unwrap();
    for msg in errors {
        writeln!(stderr, "Parser error: {}", msg).unwrap();
    }
    panic!();
}

fn test_statement(s: &Box<dyn ast::Statement>, string: &str) {
    assert_eq!(s.to_string(), string);
}

#[test]
fn test_return_statement() {
    let input = r"
return 5;
return 10;
return 993 322;
";
    let lexer = lexer::Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    check_parser_errors(&mut parser);

    assert_eq!(program.statements.len(), 3);
    for statement in &program.statements {
        assert_eq!(statement.to_string(), "return \"no-op\";");
    }
}

#[test]
fn test_identifier_expression() {
    let input = r"
foobar;
";
    let lexer = lexer::Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    check_parser_errors(&mut parser);

    assert_eq!(program.statements.len(), 1);
    assert_eq!(program.statements[0].to_string(), "foobar;");
}

#[test]
fn test_number_literal() {
    let input = r"
5; 012; 034; 0; 042;
0x12; 0xff; 0Xfd; 0x42;
12.2; 2.4e8;
";
    let lexer = lexer::Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    check_parser_errors(&mut parser);

    let expects = [
        "5;",
        "10;",
        "28;",
        "0;",
        "34;",

        "18;",
        "255;",
        "253;",
        "66;",

        "12.2;",
        "240000000;",
    ];

    test_statements(&program, &expects);
}

fn test_statements(program: &Program, expects: &[&str]) {
    assert_eq!(program.statements.len(), expects.len());
    for (index, statement) in program.statements.iter().enumerate() {
        assert_eq!(expects[index], statement.to_string());
    }
}

#[test]
fn test_prefix_expression() {
    let input = r"
-5;
!5;
";
    let lexer = lexer::Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    check_parser_errors(&mut parser);

    let expects = [
        "-(5);",
        "!(5);",
    ];

    test_statements(&program, &expects);

}