#![cfg(test)]

use std::io::stderr;
use std::io::Write;

use crate::ast::Statement;
use crate::lexer;
use crate::parser::*;

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
    test_statement(&program.statements[0], "let x");
    test_statement(&program.statements[1], "let y");
    test_statement(&program.statements[2], "let foobar");
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
        assert_eq!(statement.to_string(), "return");
    }
}
