use aoc_lib::parsing::{TextParserResult, parse_lines, Runnable, ParseError};
use aoc_runner_api::SolverResult;
use nom::{
    Parser,
    character::complete,
    sequence::{preceded, terminated},
    branch::alt, combinator::value,
    bytes::complete::tag
};
use num::Integer;

#[derive(Clone)]
enum Register {
    A, B
}

enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i64),
    JumpIfEven(Register, i64),
    JumpIfOne(Register, i64)
}

struct State {
    a: u32,
    b: u32,
    ip: usize
}

impl State {
    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Half(register) => { self.map_register(register, |x| x / 2) }
            Instruction::Triple(register) => { self.map_register(register, |x| x * 3) }
            Instruction::Increment(register) => { self.map_register(register, |x| x + 1) }
            Instruction::Jump(offset) => {
                let new_ip = self.ip as i64 + offset;
                if new_ip.is_negative() { panic!("Attempted to jump to negative instruction") }

                self.ip = new_ip as usize;
                return;
            },
            Instruction::JumpIfEven(register, offset) => {
                if self.read_register(register).is_even() {
                    return self.execute(&Instruction::Jump(*offset));
                }
            },
            Instruction::JumpIfOne(register, offset) => {
                if self.read_register(register) == 1 {
                    return self.execute(&Instruction::Jump(*offset));
                }
            }
        }

        self.ip += 1;
    }
}

impl State {
    fn read_register(&self, register: &Register) -> u32 {
        match register {
            Register::A => self.a,
            Register::B => self.b
        }
    }

    fn write_register(&mut self, register: &Register, value: u32) {
        *match register {
            Register::A => &mut self.a,
            Register::B => &mut self.b
        } = value;
    }
    
    fn map_register(&mut self, register: &Register, mapper: impl Fn(u32) -> u32) {
        self.write_register(register, mapper(self.read_register(register)));
    }
}

fn parse_instruction(input: &str) -> Result<Instruction, ParseError> {
    fn register(input: &str) -> TextParserResult<Register> {
        value(Register::A, complete::char('a'))
            .or(value(Register::B, complete::char('b')))
            .parse(input)
    }
    
    let offset = complete::i64;
    alt((
        preceded(tag("hlf "), register).map(Instruction::Half),
        preceded(tag("tpl "), register).map(Instruction::Triple),
        preceded(tag("inc "), register).map(Instruction::Increment),
        preceded(tag("jmp "), offset).map(Instruction::Jump),
        preceded(tag("jie "), terminated(register, tag(", ")).and(offset))
            .map(|(register, offset)| Instruction::JumpIfEven(register, offset)),
        preceded(tag("jio "), terminated(register, tag(", ")).and(offset))
            .map(|(register, offset)| Instruction::JumpIfOne(register, offset))
    )).run(input)
}

fn run_program(program: &[Instruction], mut state: State) -> State {
    while let Some(instruction) = program.get(state.ip) {
        state.execute(instruction);
    }

    state
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let program = parse_lines(parse_instruction, input)?;
    let State { b, .. } = run_program(&program, State { a: 0, b: 0, ip: 0 });

    Ok(Box::new(b))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let program = parse_lines(parse_instruction, input)?;
    let State { b, .. } = run_program(&program, State { a: 1, b: 0, ip: 0 });

    Ok(Box::new(b))
}