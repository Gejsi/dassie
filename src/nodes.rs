#[derive(Debug)]
pub struct Selector(pub String);

#[derive(Debug)]
pub struct Property(pub String);

#[derive(Debug)]
pub struct Value(pub String);

#[derive(Debug)]
pub struct Declaration(pub Property, pub Value);

#[derive(Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declaration_block: Vec<Declaration>,
}

#[derive(Debug)]
pub struct AtRule {
    pub identifier: String,
    pub condition: String,
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Rule(Option<Rule>),
    AtRule(Option<AtRule>),
}

#[derive(Debug)]
pub struct Stylesheet {
    pub statements: Vec<Statement>,
}

// ------------------------------------------------------------
// TODO: refine the AST nodes for values (used both in declarations and selectors).
#[derive(Debug)]
pub enum Numeric {
    Number(bool, f64),
    Percentage(bool, f64),
    Dimension(bool, f64, String),
}

#[derive(Debug)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Value2>,
}

#[derive(Debug)]
pub enum Value2 {
    /// Keyword values (such as `auto`, `disc`, etc.), which appear literally, without quotes (e.g. `auto`).
    Keyword(String),
    Literal(String),
    Numeric,
    Functional(FunctionCall),
}
// ------------------------------------------------------------
