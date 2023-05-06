use std::{error::Error, fs};

use nom::{
    Parser,
    character::complete,
    sequence::{preceded, terminated},
    branch::alt, combinator::value,
    bytes::complete::tag,
    IResult
};

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
                if self.read_register(register) % 2 == 0 {
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

fn parse_instruction(input: &str) -> Result<Instruction, String> {
    fn register(input: &str) -> IResult<&str, Register> {
        value(Register::A, complete::char('a'))
            .or(value(Register::B, complete::char('b')))
            .parse(input)
    }
    
    let offset = complete::i64;
    let mut instruction = alt((
        preceded(tag("hlf "), register).map(|register| Instruction::Half(register)),
        preceded(tag("tpl "), register).map(|register| Instruction::Triple(register)),
        preceded(tag("inc "), register).map(|register| Instruction::Increment(register)),
        preceded(tag("jmp "), offset).map(|offset| Instruction::Jump(offset)),
        preceded(tag("jie "), terminated(register, tag(", ")).and(offset))
            .map(|(register, offset)| Instruction::JumpIfEven(register, offset)),
        preceded(tag("jio "), terminated(register, tag(", ")).and(offset))
            .map(|(register, offset)| Instruction::JumpIfOne(register, offset))
    ));
    
    Ok(instruction.parse(input)
        .map_err(|err| err.to_string())?.1)
}

fn parse_program(input: &str) -> Result<Vec<Instruction>, String> {
    input.lines()
        .map(parse_instruction)
        .collect()
}

fn run_program(program: &Vec<Instruction>, mut state: State) -> State {
    while let Some(instruction) = program.get(state.ip) {
        state.execute(instruction);
    }

    state
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("inputs/2015/day_23.txt")?;
    let program = parse_program(&content)?;

    let State { b, .. } = run_program(&program, State { a: 0, b: 0, ip: 0 });
    println!("The value for register b at the end of the program is {}", b);

    let State { b, .. } = run_program(&program, State { a: 1, b: 0, ip: 0 });
    println!("The value for register b at the end of the program when starting with a=1 is {}", b);

    Ok(())
}