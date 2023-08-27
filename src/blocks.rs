pub mod system_functions;
pub mod execution_agent;

pub enum Variable {
    Integer(i64),
    Float(f64),
    Char(char),
    Bool(bool),
    Vector(Vec<Variable>),
    NAV,
}

pub enum Argument {
    Var(usize),
    Constant(Variable),
}

pub enum UnaryOperation {
    Increment, Decrement, //works like push and pop with vectors
    Negation,
    Length,
}

pub enum BinaryOperation {
    Addition, Subtraction,
    Multiplication, Division, Modulus,
    And, Or, Xor, Nand, Nor,
    ShiftLeft, ShiftRight,
}

pub enum Statement {
    UOp(usize, UnaryOperation, Argument),
    BinOp(usize, BinaryOperation, Argument, Argument),
    Let(Argument),
    SysCall(SystemFunction, Vec<Argument>),
    Call(usize, Vec<Argument>),
    If(Block, Block),   // first block must return bool
    IfElse(Block, Block, Block),
    ForRange(Argument, Argument),   //for i in range(x, y):
    ForIn(Argument),      //for x in array
    ForInEnum(Variable, Variable, Argument),  //for (i, x) in array.enumerate()
    InnerBlock(Block),
}

pub struct Block {
    pub global: Vec<Variable>,
    pub local: Vec<Variable>,
    pub statement: Vec<Statement>,
    pub return_exp: Option<usize>,
}

pub enum SystemFunction {
    PRINT,
}