use std::fs::File;
use std::path::Path;
use std::fmt;
use std::io::Write;

mod u15;
use u15::U15;

const BINARY_FILE: &'static [u8] = include_bytes!("../challenge.bin");
#[allow(dead_code)]
const TEST_PROGRAM: &'static [u16] = &[9, 32768, 32769, 4, 19, 32768];

const MAX_WM_WORD: u16 = 32775;

#[derive(Debug, Clone, Copy)]
enum WmWord {
    Constant(U15),
    Register(usize),
}

impl fmt::Display for WmWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WmWord::Constant(val) => write!(f, "{}", val),
            WmWord::Register(idx) => write!(f, "r{}", idx),
        }
    }
}

#[derive(Debug)]
enum Op {
    HALT,
    SET(WmWord, WmWord),
    PUSH(WmWord),
    POP(WmWord),
    EQ(WmWord, WmWord, WmWord),
    GT(WmWord, WmWord, WmWord),
    JMP(WmWord),
    JT(WmWord, WmWord),
    JF(WmWord, WmWord),
    ADD(WmWord, WmWord, WmWord),
    MULT(WmWord, WmWord, WmWord),
    MOD(WmWord, WmWord, WmWord),
    AND(WmWord, WmWord, WmWord),
    OR(WmWord, WmWord, WmWord),
    NOT(WmWord, WmWord),
    RMEM(WmWord, WmWord),
    WMEM(WmWord, WmWord),
    CALL(WmWord),
    RET,
    OUT(WmWord),
    IN(WmWord),
    NOOP,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Op::HALT => write!(f, "HALT"),
            Op::SET(a, b) => write!(f, "SET {} {}", a, b),
            Op::PUSH(a) => write!(f, "PUSH {}", a),
            Op::POP(a) => write!(f, "POP {}", a),
            Op::EQ(a, b, c) => write!(f, "EQ {} {} {}", a, b, c),
            Op::GT(a, b, c) => write!(f, "GT {} {} {}", a, b, c),
            Op::JMP(a) => write!(f, "JMP {}", a),
            Op::JT(a, b) => write!(f, "JT {} {}", a, b),
            Op::JF(a, b) => write!(f, "JF {} {}", a, b),
            Op::ADD(a, b, c) => write!(f, "ADD {} {} {}", a, b, c),
            Op::MULT(a, b, c) => write!(f, "MULT {} {} {}", a, b, c),
            Op::MOD(a, b, c) => write!(f, "MOD {} {} {}", a, b, c),
            Op::AND(a, b, c) => write!(f, "AND {} {} {}", a, b, c),
            Op::OR(a, b, c) => write!(f, "OR {} {} {}", a, b, c),
            Op::NOT(a, b) => write!(f, "NOT {} {}", a, b),
            Op::RMEM(a, b) => write!(f, "RMEM {} {}", a, b),
            Op::WMEM(a, b) => write!(f, "WMEM {} {}", a, b),
            Op::CALL(a) => write!(f, "CALL {}", a),
            Op::RET => write!(f, "RET"),
            Op::OUT(a) => {
                let repr = match a {
                    WmWord::Constant(con) => { 
                        if con == U15::new(10) {
                            ' '
                        } else {
                            con.to_char()
                        }
                    }
                    WmWord::Register(_) => '?'
                };
                write!(f, "OUT {} {}", a, repr)
            }
            Op::IN(a) => write!(f, "IN {}", a),
            Op::NOOP => write!(f, "NOOP"),
        }
    }
}


fn constant_or_register_value(val: WmWord, regs: &[U15; 8]) -> U15 {
    match val {
        WmWord::Constant(con) => con,
        WmWord::Register(idx) => regs[idx],
    }
}

fn decode_and_fetch(pc: U15, memory: &Memory) -> (U15, Option<Op>) {
    let word = memory.read_address(pc);
    let mut res = (pc + 1, None);
    if let WmWord::Constant(op_code) = word {
        let op_code = op_code.to_u16();
        match op_code {
            //Opcodes wich have no additional operand
            //HALT, RET, NOOP
            0 | 18 | 21 => {
                let new_pc = pc + 1;
                let op = match op_code {
                    0 => Op::HALT,
                    18 => Op::RET,
                    21 => Op::NOOP,
                    _ => panic!("Coded some shit in match arm"),
                };
                res = (new_pc, Some(op));
            }
            //Opcodes with one additional operand
            //PUSH POP JMP CALL OUT IN
            2 | 3 | 6 | 17 | 19 | 20 => {
                let a = memory.read_address(pc + 1);
                let new_pc = pc + 2;
                let op = match op_code {
                    2 => Op::PUSH(a),
                    3 => Op::POP(a),
                    6 => Op::JMP(a),
                    17 => Op::CALL(a),
                    19 => Op::OUT(a),
                    20 => Op::IN(a),
                    _ => panic!("Coded some shit in match arm"),
                };
                res = (new_pc, Some(op));
            }
            //Opcodes with two additional operands
            //SET JT JF NOT RMEM WMEM
            1 | 7 | 8 | 14 | 15 | 16 => {
                let a = memory.read_address(pc + 1);
                let b = memory.read_address(pc + 2);
                let new_pc = pc + 3;
                let op = match op_code {
                    1 => Op::SET(a, b),
                    7 => Op::JT(a, b),
                    8 => Op::JF(a, b),
                    14 => Op::NOT(a, b),
                    15 => Op::RMEM(a, b),
                    16 => Op::WMEM(a, b),
                    _ => panic!("Coded some shit in match arm"),
                };
                res = (new_pc, Some(op));
            }
            //Operations with 3 additional operands
            //EQ GT ADD MULT MOD AND OR
            4 | 5 | 9 | 10 | 11 | 12 | 13 => {
                let a = memory.read_address(pc + 1);
                let b = memory.read_address(pc + 2);
                let c = memory.read_address(pc + 3);
                let new_pc = pc + 4;
                let op = match op_code {
                    4 => Op::EQ(a, b, c),
                    5 => Op::GT(a, b, c),
                    9 => Op::ADD(a, b, c),
                    10 => Op::MULT(a, b, c),
                    11 => Op::MOD(a, b, c),
                    12 => Op::AND(a, b, c),
                    13 => Op::OR(a, b, c),
                    _ => panic!("Coded some shit in match arm"),
                };
                res = (new_pc, Some(op));
            }
            //Unknown op_code value; to support dissasembly we return None
            _ => {}
        }
    } 
    res
}

struct Memory {
    memory: Vec<WmWord>,
}

impl Memory {
    fn new() -> Memory {
        Memory {
            memory: vec![WmWord::Constant(U15::new(0)); 32768]
        }
    }

    #[allow(dead_code)]
    fn disassemble(&self, from: U15, length: u16, path: &Path) {
        let mut file = File::create(path).unwrap();
        let mut pc = from;
        let mut to_read = length;
        while to_read > 4 {
            let (new_pc, op) = decode_and_fetch(pc, self);
            to_read = to_read - (new_pc - pc).to_u16();
            file.write(format!("{:05}: ", pc.to_u16()).as_bytes()).unwrap();
            if let Some(op) = op {
                file.write(op.to_string().as_bytes()).unwrap();
            } else {
                file.write(format!("{}", self.memory[pc.to_idx()]).as_bytes())
                    .unwrap();
            }
            file.write(b"\n").unwrap();
            pc = new_pc;
        }
    }

    fn load(&mut self, program: &[u16]) {
        for (idx, word) in program.iter().enumerate() {
            let word = match *word {
                0...u15::MAX => WmWord::Constant(U15::new(*word)),
                u15::MAX...MAX_WM_WORD => WmWord::Register((word % (u15::MAX + 1)) as usize),
                _ => panic!("Invalid number was found in the binary file")
            };
            self.memory[idx] = word;
        }
    }

    fn read_address(&self, address: U15) -> WmWord {
        self.memory[address.to_idx()]
    }

    fn write_address(&mut self, address: U15, val: WmWord) {
        self.memory[address.to_idx()] = val;
    }
}


fn main() {
    use WmWord::*;

    let mut trace_execution = false;
    let mut trace_string = String::new();
    let mut pc = U15::new(0);
    let mut jump_address = None;
    let mut stack: Vec<U15> = Vec::new();
    let mut input_string = String::new();
    //Registers are named 0..7 and indexed as such
    let mut regs = [U15::new(0); 8];
    let mut memory = Memory::new();
    let to_load: Vec<u16> = BINARY_FILE
        .chunks(2)
        .map(|chunk| ((chunk[1] as u16) << 8) + (chunk[0] as u16))
        .collect();
    memory.load(&to_load);

    loop {
        let (new_pc, op) = decode_and_fetch(pc, &memory);
        //We are about to check for the correct value in the register
        if pc == U15::new(5451) {
            //Calculated value with  the program
            regs[7] = U15::new(25734);
            //Override the check if the register is correct with NOOP
            //because we know it is
            for add in 5483..5498 {
                memory.write_address(U15::new(add), WmWord::Constant(U15::new(21)));
            }
        }
        let op = op.expect("We couldn't decode the instruction!");
        if trace_execution {
            trace_string.push_str(&op.to_string());
            trace_string.push('\n');
        }
        match op {
            Op::HALT => {
                break;
            }
            Op::OUT(code) => {
                let code = constant_or_register_value(code, &regs);
                print!("{}", code.to_char());
            }
            Op::NOOP => {
                //Literally do nothing
            }
            Op::JMP(dest) => {
                let val = constant_or_register_value(dest, &regs);
                jump_address = Some(val);
            }
            Op::JT(cond, dest) => {
                let cond = constant_or_register_value(cond, &regs);
                if cond != U15::new(0) {
                    let dest = constant_or_register_value(dest, &regs);
                    jump_address = Some(dest);
                }
            }
            Op::JF(cond, dest) => {
                let cond = constant_or_register_value(cond, &regs);
                if cond == U15::new(0) {
                    let dest = constant_or_register_value(dest, &regs);
                    jump_address = Some(dest);
                }
            }
            Op::SET(dest, b) => {
                let val = constant_or_register_value(b, &regs);
                if let Register(idx) = dest {
                    regs[idx] = val;
                }
            }
            Op::ADD(dest, b, c) => {
                let b = constant_or_register_value(b, &regs);
                let c = constant_or_register_value(c, &regs);
                if let Register(idx) = dest {
                    regs[idx] = b + c;
                }
            }
            Op::EQ(dest, b, c) => {
                let b = constant_or_register_value(b, &regs);
                let c = constant_or_register_value(c, &regs);
                let set_value = if b == c { U15::new(1) } else { U15::new(0) };
                if let Register(idx) = dest {
                    regs[idx] = set_value;
                }

            }
            Op::PUSH(val) => {
                let val = constant_or_register_value(val, &regs);
                stack.push(val);
            }
            Op::POP(dest) => {
                let val = stack.pop().expect("Empty stack with pop panic!");
                if let Register(idx) = dest {
                    regs[idx] = val;
                }
            }
            Op::GT(dest, b, c) => {
                let b = constant_or_register_value(b, &regs);
                let c = constant_or_register_value(c, &regs);
                let set_value = if b > c { U15::new(1) } else { U15::new(0) };
                if let Register(idx) = dest {
                    regs[idx] = set_value;
                }
            }
            Op::AND(dest, b, c) => {
                let b = constant_or_register_value(b, &regs);
                let c = constant_or_register_value(c, &regs);
                if let Register(idx) = dest {
                    regs[idx] = b & c;
                }
            }
            Op::OR(dest, b, c) => {
                let b = constant_or_register_value(b, &regs);
                let c = constant_or_register_value(c, &regs);
                if let Register(idx) = dest {
                    regs[idx] = b | c;
                }
            }
            Op::NOT(dest, b) => {
                let b = constant_or_register_value(b, &regs);
                if let Register(idx) = dest {
                    regs[idx] = !b;
                }
            }
            Op::CALL(dest) => {
                let dest = constant_or_register_value(dest, &regs);
                stack.push(new_pc);
                jump_address = Some(dest);
            }
            Op::MULT(dest, b, c) => {
                let b = constant_or_register_value(b, &regs);
                let c = constant_or_register_value(c, &regs);
                if let Register(idx) = dest {
                    regs[idx] = b * c;
                }
            }
            Op::MOD(dest, b, c) => {
                let b = constant_or_register_value(b, &regs);
                let c = constant_or_register_value(c, &regs);
                if let Register(idx) = dest {
                    regs[idx] = b % c;
                }
            }
            // After thinking through it there is no possibilty to load or store 
            // a value > 15 bit with this instruction. These instructions are
            // constrained by the register size!
            Op::RMEM(dest, b) => {
                let from = constant_or_register_value(b, &regs);
                if let Constant(val) = memory.read_address(from) {
                    if let Register(idx) = dest {
                        regs[idx] = val;
                    }
                 }
            }
            Op::WMEM(a, b) => {
                let val = constant_or_register_value(b, &regs);
                let dest = constant_or_register_value(a, &regs);
                memory.write_address(dest, Constant(val));
            }
            Op::RET => {
                //Halt if the stack is empty like we were told to
                if stack.is_empty() {
                    break;
                }
                let val = stack.pop().unwrap();
                jump_address = Some(val);
            }
            //Handle the input here, Just read the string once and than each
            //cycle we hit here we chomp away from our string.
            //That's possible according to the arch-spec
            Op::IN(dest) => {
                if input_string.is_empty() {
                    //If we hit the next input request we stop tracing
                    //An put the trace in a file
                    if trace_execution {
                        trace_execution = false;
                        let mut file = File::create("trace").unwrap();
                        file.write(trace_string.as_bytes()).unwrap();
                        trace_string.clear();
                    }
                    // memory.disassemble(Value(0), to_load.len() as u16, Path::new("dissasembly"));
                    std::io::stdin().read_line(&mut input_string).unwrap();
                    
                    //If we use the teleporter start tracing the execution
                    // if input_string == "use teleporter\n" {
                    //     trace_execution = true;
                    // }
                    //reverse the string so we can easily pop on element after
                    //the other
                    input_string = input_string.chars().rev().collect();
                }
                if let Register(idx) = dest {
                    regs[idx] = U15::new(input_string.pop().unwrap() as u16);
                }
            }
        }
        pc = if let Some(jump) = jump_address {
            jump_address = None;
            jump
        } else {
            new_pc
        };
    }
}
