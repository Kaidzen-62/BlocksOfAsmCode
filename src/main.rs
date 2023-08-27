use BlocksOfAsmCode::blocks as b;
use BlocksOfAsmCode::blocks::SystemFunction as sf;
use BlocksOfAsmCode::blocks::system_functions as sys_fn;
use BlocksOfAsmCode::blocks::execution_agent as exa;

fn main() {
    println!("Hello, world!");
    let mut program = b::FunctionBlock {
        arguments: vec![],
        variables: vec![],
        statements: vec![],
        return_exp: None,
    };

    program.statements = vec![
        b::Statements::Let,
        b::Statements::Let,
        b::Statements::Let,
        b::Statements::BinOp,
        b::Statements::SysCall(sf::PRINT, vec![]),
    ];
}
