
use std::collections::HashSet;
use std::error;
use std::fmt;
use std::str::FromStr;

use crate::lib;

enum Instruction {
    NOP,
    ACC(isize),
    JMP(isize),
}

struct State {
    pc: isize,
    acc: isize,
}

impl State {
    pub fn new() -> Self {
        State{ pc: 0, acc: 0, }
    }
}

#[derive(Debug, Clone)]
struct BadInputLine;
impl fmt::Display for BadInputLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
impl error::Error for BadInputLine {}

#[derive(Debug, Clone)]
struct UnknownCode;
impl fmt::Display for UnknownCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
impl error::Error for UnknownCode {}

impl FromStr for Instruction {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split_whitespace().collect();
        if tokens.len() != 2 { return Err(BadInputLine.into()); }
        let param = tokens[1].parse::<isize>()?;
        match tokens[0] {
            "nop" => Ok(Instruction::NOP),
            "acc" => Ok(Instruction::ACC(param)),
            "jmp" => Ok(Instruction::JMP(param)),
            _ => Err(UnknownCode.into()),
        }
    }
}

fn execute(state: &mut State, prgm: &Vec<Instruction>) {
    assert!(state.pc >= 0 && state.pc < prgm.len() as isize,
        "state.pc out of bounds: {}, prgm len {}", state.pc, prgm.len());
    match prgm[state.pc as usize] {
        Instruction::NOP => state.pc += 1,
        Instruction::ACC(x) => {
            state.acc += x;
            state.pc += 1;
        },
        Instruction::JMP(x) => state.pc += x,
    }
}

pub fn run(part: u32) {
    let program: Vec<Instruction> = lib::parse::read_to_vec::<Instruction>("input/08.txt");
    let mut state = State::new();
    let mut seen = HashSet::new();
    while !seen.contains(&state.pc) {
        seen.insert(state.pc);
        execute(&mut state, &program);
    }
    println!("accumulator: {}", state.acc);
}
