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
            operator: Op::END,
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
                Op::END => break,
                _ => write!(f, "{idx}: {}, {}\n", i.operator, i.operand),
            }?;
            len = idx;
        }
        writeln!(f, "{}: Op::END, 0", len + 1)
    }
}
#[derive(Clone, Debug, PartialEq)]
enum Op {
    END,
    INC_DP,
    DEC_DP,
    INC_VAL,
    DEC_VAL,
    OUT,
    IN,
    JMP_FWD,
    JMP_BCK
}
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::END => write!(f, "Op::END"),
            Op::INC_DP => write!(f, "Op::INC_DP"),
            Op::DEC_DP => write!(f, "Op::DEC_DP"),
            Op::INC_VAL => write!(f, "Op::INC_VAL"),
            Op::DEC_VAL => write!(f, "Op::DEC_VAL"),
            Op::OUT => write!(f, "Op::OUT"),
            Op::IN => write!(f, "Op::IN"),
            Op::JMP_FWD => write!(f, "Op::JMP_FWD"),
            Op::JMP_BCK => write!(f, "Op::JMP_BCK"),
        }
    }
}

fn compile(file: &str) -> Program {
    let mut pc: usize = 0;
    let mut program = Program::new();
    let mut stck: Vec<u32> = vec![];
    for c in file.chars() {
        match c {
            '>' => program.ins[pc].operator = Op::INC_DP,
            '<' => program.ins[pc].operator = Op::DEC_DP,
            '+' => program.ins[pc].operator = Op::INC_VAL,
            '-' => program.ins[pc].operator = Op::DEC_VAL,
            '.' => program.ins[pc].operator = Op::OUT,
            ',' => program.ins[pc].operator = Op::IN,
            '[' => {
                program.ins[pc].operator = Op::JMP_FWD;
                stck.push((pc).try_into().unwrap());
            }
            ']' => {
                let jmp_pc = stck.pop().unwrap();
                program.ins[pc].operator = Op::JMP_BCK;
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
    while program.ins[pc].operator != Op::END && ptr < 65535 {
        match program.ins[pc].operator {
            Op::INC_DP => ptr += 1,
            Op::DEC_DP => ptr -= 1,
            Op::INC_VAL => data[ptr] += 1, 
            Op::DEC_VAL => data[ptr] -= 1,
            Op::OUT => print!("{}", &data[ptr]),
            Op::IN => {
                io::stdout().flush().unwrap();
                data[ptr] = get_input(); 
            },
            Op::JMP_FWD => {
                if data[ptr] == 0 {
                    pc = program.ins[pc].operand as usize;
                }
            },
            Op::JMP_BCK => {
                if data[ptr] != 0 {
                    pc = program.ins[pc].operand as usize;
                }
            }
            Op::END => unreachable!(),
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
