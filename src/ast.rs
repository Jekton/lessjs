
use crate::lexer::token::TokenKind;



pub trait Node : ToString {
}

pub trait Expression : Node {
}

pub trait Statement : Node {
}

impl<T: ToString> Node for T {}

pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl ToString for Program {

    fn to_string(&self) -> String {
        if self.statements.is_empty() {
            return "".to_string();
        }
        let mut ret = String::new();
        for statement in &self.statements {
            if !ret.is_empty() {
                ret += "\n";
            }
            ret += &statement.to_string();
        }
        return ret;
    }
}

pub struct Identifier {
    pub name: String,
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl Expression for Identifier {}

pub struct NumberLiteral {
    pub value: f64,
}

impl ToString for NumberLiteral {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Expression for NumberLiteral { }

pub struct NoOpExpression;

impl ToString for NoOpExpression {
    fn to_string(&self) -> String {
        return "\"no-op\"".to_string()
    }
}

impl Expression for NoOpExpression {}

pub struct PrefixExpression {
    pub op: TokenKind,
    pub expression: Box<dyn Expression>,
}

impl ToString for PrefixExpression {
    fn to_string(&self) -> String {
        match self.op {
            TokenKind::Band => format!("(!{})", self.expression.to_string()),
            TokenKind::Minus => format!("(-{})", self.expression.to_string()),
            _ => panic!("invalid prefix operator")
        }
    }
}

impl Expression for PrefixExpression { }

pub struct InfixExpression {
    pub op: TokenKind,
    pub lhs: Box<dyn Expression>,
    pub rhs: Box<dyn Expression>,
}

impl ToString for InfixExpression {
    fn to_string(&self) -> String {
        let op = match self.op {
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Asterisk => "*",
            TokenKind::Slash => "/",
            TokenKind::LT => "<",
            TokenKind::GT => ">",
            TokenKind::EQ => "==",
            TokenKind::NE => "!=",
            TokenKind::SEQ => "===",
            TokenKind::SNE => "!==",
            _ => panic!("invalid infix operator {:?}", self.op)
        };
        format!("({} {} {})", self.lhs.to_string(), op, self.rhs.to_string())
    }
}

impl Expression for InfixExpression { }

pub struct LetStatement {
    pub id: Identifier,
    pub value: Box<dyn Expression>,
}

impl ToString for LetStatement {
    fn to_string(&self) -> String {
        format!("let {} = {};", self.id.to_string(), self.value.to_string())
    }
}

impl Statement for LetStatement {}

pub struct ReturnStatement {
    pub value: Box<dyn Expression>,
}

impl ToString for ReturnStatement {
    fn to_string(&self) -> String {
        format!("return {};", self.value.to_string())
    }
}

impl Statement for ReturnStatement { }

pub struct ExpressionStatement {
    pub expression: Box<dyn Expression>,
}

impl ToString for ExpressionStatement {
    fn to_string(&self) -> String {
        format!("{};", self.expression.to_string())
    }
}

impl Statement for ExpressionStatement { }


