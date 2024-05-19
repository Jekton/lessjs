

pub trait Node : ToString {
}

pub trait Expression : Node {
}

pub trait Statement : Node {
}

impl<T : ToString> Node for T {}

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
                ret += ";\n";
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

pub struct NoOpExpression;

impl ToString for NoOpExpression {
    fn to_string(&self) -> String {
        return "\"no-op\"".to_string()
    }
}

impl Expression for NoOpExpression {}

pub struct LetStatement {
    pub id: Identifier,
    pub value: Box<dyn Expression>,
}

impl ToString for LetStatement {
    fn to_string(&self) -> String {
        format!("let {}", self.id.to_string())
    }
}

impl Statement for LetStatement {}

pub struct ReturnStatement {
    pub value: Box<dyn Expression>,
}

impl ToString for ReturnStatement {
    fn to_string(&self) -> String {
        "return".to_string()
    }
}

impl Statement for ReturnStatement { }
