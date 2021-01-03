
use std::collections::HashSet;
use std::error;
use std::fmt;
use std::str::FromStr;

use crate::lib;

#[derive(Clone)]
enum Instruction {
    NOP(isize),
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
            "nop" => Ok(Instruction::NOP(param)),
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
        Instruction::NOP(_) => state.pc += 1,
        Instruction::ACC(x) => {
            state.acc += x;
            state.pc += 1;
        },
        Instruction::JMP(x) => state.pc += x,
    }
}

fn run_to_termination(prgm: &Vec<Instruction>) -> State {
    let mut state = State::new();
    let mut seen = HashSet::new();
    while !seen.contains(&state.pc) && state.pc != prgm.len() as isize {
        seen.insert(state.pc);
        execute(&mut state, &prgm);
    }
    state
}

pub fn run(part: u32) {
    let program: Vec<Instruction> = lib::parse::read_to_vec::<Instruction>("input/08.txt");
    if part == 1 {
        let state = run_to_termination(&program);
        println!("accumulator: {}", state.acc);
    } else if part == 2 {
        for i in 0..program.len() {
            let mut trial_prgm = program.clone();
            trial_prgm[i] = match trial_prgm[i] {
                Instruction::NOP(x) => Instruction::JMP(x),
                Instruction::ACC(x) => Instruction::ACC(x),
                Instruction::JMP(x) => Instruction::NOP(x),
            };
            let state = run_to_termination(&trial_prgm);
            if state.pc == trial_prgm.len() as isize {
                println!("accumulator: {}", state.acc);
                break;
            }
        }
    }
}
