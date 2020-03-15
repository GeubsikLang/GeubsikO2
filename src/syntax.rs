pub struct Identifier<'a> {
    pub name: &'a str,
}

pub enum Literal<'a> {
    Undefined,
    Number(f64),
    String(&'a str),
}

pub enum Expression<'a> {
    Negate(Box<Expression<'a>>),
    Add(Box<Expression<'a>>, Box<Expression<'a>>),
    Subtract(Box<Expression<'a>>, Box<Expression<'a>>),
    Multiply(Box<Expression<'a>>, Box<Expression<'a>>),
    Divide(Box<Expression<'a>>, Box<Expression<'a>>),
    Modulo(Box<Expression<'a>>, Box<Expression<'a>>),
    Equal(Box<Expression<'a>>, Box<Expression<'a>>),
    NotEqual(Box<Expression<'a>>, Box<Expression<'a>>),
    StrictEqual(Box<Expression<'a>>, Box<Expression<'a>>),
    StrictNotEqual(Box<Expression<'a>>, Box<Expression<'a>>),
    GreaterThan(Box<Expression<'a>>, Box<Expression<'a>>),
    GreaterOrEqualThan(Box<Expression<'a>>, Box<Expression<'a>>),
    LessThan(Box<Expression<'a>>, Box<Expression<'a>>),
    LessOrEqualThan(Box<Expression<'a>>, Box<Expression<'a>>),
    Object(Identifier<'a>),
    Literal(Literal<'a>),
}

pub enum Statement<'a> {
    Assign(Assignment<'a>),
    Increase(Increment<'a>),
    Decrease(Decrement<'a>),
    Print(Print<'a>),
    Read(Read<'a>),
    If(If<'a>),
    LoopWhile(LoopWhile<'a>),
    Break,
    Continue,
}

pub struct Assignment<'a> {
    pub variable: Identifier<'a>,
    pub value: Expression<'a>,
}

pub struct Increment<'a> {
    pub variable: Identifier<'a>,
}

pub struct Decrement<'a> {
    pub variable: Identifier<'a>,
}

pub enum PrintFormat {
    String,
    Line,
    Byte,
}

pub struct Print<'a> {
    pub value: Expression<'a>,
    pub format: PrintFormat,
}

pub enum ReadFormat {
    Number,
    Byte,
}

pub struct Read<'a> {
    pub variable: Identifier<'a>,
}

pub struct If<'a> {
    pub condition: Expression<'a>,
    pub block: Vec<Statement<'a>>,
    pub else_block: Option<Vec<Statement<'a>>>,
}

pub struct LoopWhile<'a> {
    pub condition: Expression<'a>,
    pub block: Vec<Statement<'a>>,
}