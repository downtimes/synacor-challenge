use std::ops::{Add, Deref, Mul, Rem};
use std::io::Write;
use std::fmt;
use std::fs::File;
use std::path::Path;
use std::str;

const MATH_MODULO: u16 = 32768;

const BINARY_FILE: &'static [u8] = include_bytes!("../challenge.bin");
#[allow(dead_code)]
const TEST_PROGRAM: &'static [u16] = &[9, 32768, 32769, 4, 19, 32768];

//TODO: Implement the from trait for this one so we don't have to 
//write Value all the time.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Value(u16);

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let to_disp = if self.is_register() {
            format!("r{}", self.0 % MATH_MODULO)
        } else {
            format!("{}", self.0)
        };
        write!(f, "{}", to_disp)
    }
}

impl Value {
    fn is_register(&self) -> bool {
        match self.0 {
            32768...32775 => true,
            _ => false,
        }
    }
}

impl Deref for Value {
    type Target = u16;

    fn deref(&self) -> &u16 {
        &self.0
    }
}

impl Add<Value> for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        Value((self.0 + other.0) % MATH_MODULO)
    }
}

impl Rem<Value> for Value {
    type Output = Value;

    fn rem(self, other: Value) -> Value {
        Value(self.0 % other.0)
    }
}

impl Add<u16> for Value {
    type Output = Value;

    fn add(self, other: u16) -> Value {
        Value((self.0 + other) % MATH_MODULO)
    }
}

impl Mul<Value> for Value {
    type Output = Value;

    fn mul(self, other: Value) -> Value {
        Value((((self.0 as u32) * (other.0 as u32)) % (MATH_MODULO as u32)) as u16)
    }
}


#[derive(Debug)]
enum Op {
    HALT,
    SET(Value, Value),
    PUSH(Value),
    POP(Value),
    EQ(Value, Value, Value),
    GT(Value, Value, Value),
    JMP(Value),
    JT(Value, Value),
    JF(Value, Value),
    ADD(Value, Value, Value),
    MULT(Value, Value, Value),
    MOD(Value, Value, Value),
    AND(Value, Value, Value),
    OR(Value, Value, Value),
    NOT(Value, Value),
    RMEM(Value, Value),
    WMEM(Value, Value),
    CALL(Value),
    RET,
    OUT(Value),
    IN(Value),
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
            Op::OUT(a) => write!(f, "OUT {} {}", a, (*a as u8) as char),
            Op::IN(a) => write!(f, "IN {}", a),
            Op::NOOP => write!(f, "NOOP"),
        }
    }
}

fn write_register(regs: &mut [Value; 8], address: Value, val: Value) {
    assert!(address.is_register());
    regs[(*address % MATH_MODULO) as usize] = val;
}

fn constant_or_register_value(val: Value, regs: &[Value; 8]) -> Value {
    match val.is_register() {
        false => val,
        true => regs[(*val % MATH_MODULO) as usize],
    }
}

fn decode_and_fetch(pc: Value, memory: &Memory) -> (Value, Option<Op>) {
    let op_code = memory.read_address(pc);
    match *op_code {
        //Opcodes wich have no additional operand
        //HALT, RET, NOOP
        0 | 18 | 21 => {
            let new_pc = pc + 1;
            let op = match *op_code {
                0 => Op::HALT,
                18 => Op::RET,
                21 => Op::NOOP,
                _ => panic!("Coded some shit in match arm"),
            };
            (new_pc, Some(op))
        }
        //Opcodes with one additional operand
        //PUSH POP JMP CALL OUT IN
        2 | 3 | 6 | 17 | 19 | 20 => {
            let a = memory.read_address(pc + 1);
            let new_pc = pc + 2;
            let op = match *op_code {
                2 => Op::PUSH(a),
                3 => Op::POP(a),
                6 => Op::JMP(a),
                17 => Op::CALL(a),
                19 => Op::OUT(a),
                20 => Op::IN(a),
                _ => panic!("Coded some shit in match arm"),
            };
            (new_pc, Some(op))
        }
        //Opcodes with two additional operands
        //SET JT JF NOT RMEM WMEM
        1 | 7 | 8 | 14 | 15 | 16 => {
            let a = memory.read_address(pc + 1);
            let b = memory.read_address(pc + 2);
            let new_pc = pc + 3;
            let op = match *op_code {
                1 => Op::SET(a, b),
                7 => Op::JT(a, b),
                8 => Op::JF(a, b),
                14 => Op::NOT(a, b),
                15 => Op::RMEM(a, b),
                16 => Op::WMEM(a, b),
                _ => panic!("Coded some shit in match arm"),
            };
            (new_pc, Some(op))
        }
        //Operations with 3 additional operands
        //EQ GT ADD MULT MOD AND OR
        4 | 5 | 9 | 10 | 11 | 12 | 13 => {
            let a = memory.read_address(pc + 1);
            let b = memory.read_address(pc + 2);
            let c = memory.read_address(pc + 3);
            let new_pc = pc + 4;
            let op = match *op_code {
                4 => Op::EQ(a, b, c),
                5 => Op::GT(a, b, c),
                9 => Op::ADD(a, b, c),
                10 => Op::MULT(a, b, c),
                11 => Op::MOD(a, b, c),
                12 => Op::AND(a, b, c),
                13 => Op::OR(a, b, c),
                _ => panic!("Coded some shit in match arm"),
            };
            (new_pc, Some(op))
        }
        //Unknown op_code value; to support dissasembly we return None
        _ => (pc + 1, None),
    }
}

struct Memory {
    memory: Vec<u16>,
}

impl Memory {
    fn new() -> Memory {
        Memory {
            memory: vec![0; 32768]
        }
    }

    #[allow(dead_code)]
    fn disassemble(&self, from: Value, length: u16, path: &Path) {
        let mut file = File::create(path).unwrap();
        let mut pc = from;
        let mut to_read = length;
        while to_read > 4 {
            let (new_pc, op) = decode_and_fetch(pc, self);
            to_read = to_read - (*new_pc - *pc);
            file.write(format!("{:05}: ", *pc).as_bytes()).unwrap();
            if let Some(op) = op {
                file.write(op.to_string().as_bytes()).unwrap();
            } else {
                file.write(format!("{}", self.memory[*pc as usize]).as_bytes())
                    .unwrap();
            }
            file.write(b"\n").unwrap();
            pc = new_pc;
        }
    }

    fn load(&mut self, program: &[u16]) {
        for (idx, word) in program.iter().enumerate() {
            self.memory[idx] = *word;
        }
    }

    fn read_address(&self, address: Value) -> Value {
        match *address {
            0...32767 => Value((self.memory)[*address as usize]),
            _ => panic!("Invalid memory read detected: {}", *address),
        }
    }

    fn write_address(&mut self, address: Value, val: Value) {
        match *address {
            0...32767 => (self.memory)[*address as usize] = *val,
            _ => panic!("Invalid memory write detected: {}", *address),
        }
    }
}

//TODO: we pass the selftest but somehow the stuff we wrote into memory is
//half garbage.. Why is that?
//TODO: This thing has self modifying code. Maybe we should write a
//Frontend for inspecting it while running (Testing limn or elm)
fn main() {
    let mut pc = Value(0);
    let mut jump_address = None;
    let mut stack: Vec<Value> = Vec::new();
    let mut input_string = String::new();
    //Registers are named 0..7 and indexed as such
    let mut regs = [Value(0); 8];
    let mut memory = Memory::new();
    let to_load: Vec<u16> = BINARY_FILE
        .chunks(2)
        .map(|chunk| ((chunk[1] as u16) << 8) + (chunk[0] as u16))
        .collect();
    memory.load(&to_load);

    loop {
        let (new_pc, op) = decode_and_fetch(pc, &memory);
        let op = op.expect("We couldn't decode the instruction!");
        match op {
            Op::HALT => {
                break;
            }
            Op::OUT(code) => {
                let code = [*code as u8];
                print!("{}", str::from_utf8(&code).unwrap());
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
                if cond != Value(0) {
                    let dest = constant_or_register_value(dest, &regs);
                    jump_address = Some(dest);
                }
            }
            Op::JF(cond, dest) => {
                let cond = constant_or_register_value(cond, &regs);
                if cond == Value(0) {
                    let dest = constant_or_register_value(dest, &regs);
                    jump_address = Some(dest);
                }
            }
            Op::SET(dest, b) => {
                let val = constant_or_register_value(b, &regs);
                write_register(&mut regs, dest, val);
            }
            Op::ADD(dest, b, c) => {
                let b = constant_or_register_value(b, &regs);
                let c = constant_or_register_value(c, &regs);
                write_register(&mut regs, dest, b + c);
            }
            Op::EQ(dest, b, c) => {
                let b = constant_or_register_value(b, &regs);
                let c = constant_or_register_value(c, &regs);
                let set_value = if b == c { Value(1) } else { Value(0) };
                write_register(&mut regs, dest, set_value);
            }
            Op::PUSH(val) => {
                let val = constant_or_register_value(val, &regs);
                stack.push(val);
            }
            Op::POP(dest) => {
                let val = stack.pop().expect("Empty stack with pop panic!");
                write_register(&mut regs, dest, val);
            }
            Op::GT(dest, b, c) => {
                let b = constant_or_register_value(b, &regs);
                let c = constant_or_register_value(c, &regs);
                let set_value = if b > c { Value(1) } else { Value(0) };
                write_register(&mut regs, dest, set_value);
            }
            Op::AND(dest, b, c) => {
                let b = constant_or_register_value(b, &regs);
                let c = constant_or_register_value(c, &regs);
                write_register(&mut regs, dest, Value(*b & *c));
            }
            Op::OR(dest, b, c) => {
                let b = constant_or_register_value(b, &regs);
                let c = constant_or_register_value(c, &regs);
                write_register(&mut regs, dest, Value(*b | *c));
            }
            Op::NOT(dest, b) => {
                //Flip b and then toggle the highest bit back to have 15bit not
                //CAUTION: DIESE FUCKING VERGESSENE ABFRAGE HAT MICH STUNDEN GEKOSTET
                let b = constant_or_register_value(b, &regs);
                let set_value = Value((1 << 15) ^ !*b);
                write_register(&mut regs, dest, set_value);
            }
            Op::CALL(dest) => {
                let dest = constant_or_register_value(dest, &regs);
                stack.push(new_pc);
                jump_address = Some(dest);
            }
            Op::MULT(dest, b, c) => {
                let b = constant_or_register_value(b, &regs);
                let c = constant_or_register_value(c, &regs);
                write_register(&mut regs, dest, b * c);
            }
            Op::MOD(dest, b, c) => {
                let b = constant_or_register_value(b, &regs);
                let c = constant_or_register_value(c, &regs);
                write_register(&mut regs, dest, b % c);
            }
            Op::RMEM(dest, b) => {
                let from = constant_or_register_value(b, &regs);
                let val = memory.read_address(from);
                write_register(&mut regs, dest, val);
            }
            Op::WMEM(a, b) => {
                let val = constant_or_register_value(b, &regs);
                let dest = constant_or_register_value(a, &regs);
                memory.write_address(dest, val);
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
                    // memory.disassemble(Value(0), to_load.len() as u16, Path::new("dissasembly"));
                    std::io::stdin().read_line(&mut input_string).unwrap();
                    //reverse the string so we can easily pop on element after
                    //the other
                    input_string = input_string.chars().rev().collect();
                }
                let c: Value = Value(input_string.pop().unwrap() as u16);
                write_register(&mut regs, dest, c);
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
