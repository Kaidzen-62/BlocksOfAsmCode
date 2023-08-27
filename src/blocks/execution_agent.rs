use crate::blocks::Block;

struct EXA {
    functions: Vec<Block>,
    program: Block,
}

impl EXA {
    fn new(program: Option<Block>) -> EXA {
        match program {
            None => EXA {
                functions: vec![],
                program: Block {
                    global: vec![],
                    local: vec![],
                    statement: vec![],
                    return_exp: None,
                },
            },
            Some(B) => EXA { functions: vec![], program: B},
        }
    }

    fn interpreter_it(&mut self) {
        for stat in self.program.statement.iter() {
            let a = &self.program.local[0];
        }
    }
}