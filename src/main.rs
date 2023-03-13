use std::default::Default;
use std::fmt;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Instruction {
    operator: Op,
    operand: u32,
}
impl Default for Instruction {
    fn default() -> Self {
        Self {
            operator: Op::End,
            operand: 0,
        }
    }
}

#[derive(Clone)]
struct Program {
    ins: Vec<Instruction>,
}
impl Program {
    fn new() -> Self {
        Program{
            ins: vec![Instruction::default(); 4096], 
        }
    }
}
impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut len = 0;
        for (idx, i) in self.ins.iter().enumerate() {
            match i.operator {
                Op::End => break,
                _ => write!(f, "{idx}: {}, {}\n", i.operator, i.operand),
            }?;
            len = idx;
        }
        writeln!(f, "{}: Op::End, 0", len + 1)
    }
}
#[derive(Clone, Debug, PartialEq)]
enum Op {
    End,
    IncDp,
    DecDp,
    IncVal,
    DecVal,
    Out,
    In,
    JmpFwd,
    JmpBck
}
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::End => write!(f, "Op::End"),
            Op::IncDp => write!(f, "Op::IncDp"),
            Op::DecDp => write!(f, "Op::DecDp"),
            Op::IncVal => write!(f, "Op::IncVal"),
            Op::DecVal => write!(f, "Op::DecVal"),
            Op::Out => write!(f, "Op::Out"),
            Op::In => write!(f, "Op::In"),
            Op::JmpFwd => write!(f, "Op::JMP_FWD"),
            Op::JmpBck => write!(f, "Op::JmpBck"),
        }
    }
}

fn compile(file: &str) -> Program {
    let mut pc: usize = 0;
    let mut program = Program::new();
    let mut stck: Vec<u32> = vec![];
    for c in file.chars() {
        match c {
            '>' => program.ins[pc].operator = Op::IncDp,
            '<' => program.ins[pc].operator = Op::DecDp,
            '+' => program.ins[pc].operator = Op::IncVal,
            '-' => program.ins[pc].operator = Op::DecVal,
            '.' => program.ins[pc].operator = Op::Out,
            ',' => program.ins[pc].operator = Op::In,
            '[' => {
                program.ins[pc].operator = Op::JmpFwd;
                stck.push((pc).try_into().unwrap());
            }
            ']' => {
                let jmp_pc = stck.pop().unwrap();
                program.ins[pc].operator = Op::JmpBck;
                program.ins[pc].operand = jmp_pc;
                program.ins[jmp_pc as usize].operand = pc as u32;
            }
            _ => {
                pc -= 1;
                break;
            }

        }
        pc += 1;
    }
    program
}

fn get_input() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");

    let integer: i32 = input
        .trim()
        .parse()
        .expect("Could not parse int from line");
    integer
}

fn eval(program: Program) {
    let mut data: [i32; 65535] = [0; 65535];
    let mut pc: usize = 0;
    let mut ptr: usize = 0;
    while program.ins[pc].operator != Op::End && ptr < 65535 {
        match program.ins[pc].operator {
            Op::IncDp => ptr += 1,
            Op::DecDp => ptr -= 1,
            Op::IncVal => data[ptr] += 1, 
            Op::DecVal => data[ptr] -= 1,
            Op::Out => print!("{}", &data[ptr]),
            Op::In => {
                io::stdout().flush().unwrap();
                data[ptr] = get_input(); 
            },
            Op::JmpFwd => {
                if data[ptr] == 0 {
                    pc = program.ins[pc].operand as usize;
                }
            },
            Op::JmpBck => {
                if data[ptr] != 0 {
                    pc = program.ins[pc].operand as usize;
                }
            }
            Op::End => unreachable!(),
        }
        pc += 1
    }
}

fn main() {
    // brainfuck interpreter
    let file = include_str!("input.bf");
    let prog = compile(file);
    eval(prog.clone());
    #[cfg(feature = "print_ir")]
    {
        println!();
        println!("{:?}", prog);
    }
}
