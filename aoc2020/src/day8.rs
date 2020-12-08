use std::str::FromStr;
use std::iter::repeat;

use super::error::AocError;

pub fn part1(input: String) -> Result<i32, AocError> {
    if let RunResult::InfiniteLoop(acc) = input.parse::<Program>()?.run() {
        return Ok(acc);
    }
    Err(AocError::Unknown)
}

pub fn part2(input: String) -> Result<i32, AocError> {
    let mut prog = input.parse::<Program>()?;

    for i in 0..prog.0.len() {
        let orig = prog.0[i];
        let fixed = match orig {
            Instruction::Nop(v) => Instruction::Jmp(v),
            Instruction::Jmp(v) => Instruction::Nop(v),
            _ => continue,
        };
        prog.0[i] = fixed;
        if let RunResult::Terminated(acc) = prog.run() {
            return Ok(acc);
        }
        prog.0[i] = orig;
    }
    Err(AocError::Unknown)
}

#[derive(Clone, Copy)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Instruction {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.splitn(2, ' ');
        let opcode = split.next().unwrap();
        let val = split
            .next()
            .ok_or(AocError::ParseError("no argument".to_string()))?
            .parse::<i32>()?;

        match opcode {
            "nop" => Ok(Instruction::Nop(val)),
            "acc" => Ok(Instruction::Acc(val)),
            "jmp" => Ok(Instruction::Jmp(val)),
            _ => Err(AocError::ParseError("invalid opcode".to_string())),
        }
    }
}

struct Program(Vec<Instruction>);

impl FromStr for Program {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let prog = s.lines()
            .map(|line| line.parse::<Instruction>())
            .collect::<Result<Vec<Instruction>, AocError>>()?;
        Ok(Program(prog))
    }
}

enum RunResult {
    InfiniteLoop(i32),
    Terminated(i32),
    Failure,
}

impl Program {
    fn run(&self) -> RunResult {
        let mut visited: Vec<bool> = repeat(false).take(self.0.len()).collect();
        let mut eip = 0;
        let mut accumulator = 0;
        loop {
            if eip > self.0.len() {
                return RunResult::Failure;
            }

            if eip == self.0.len() {
                return RunResult::Terminated(accumulator);
            }

            if visited[eip] {
                return RunResult::InfiniteLoop(accumulator);
            }
            visited[eip] = true;

            let cur = self.0[eip];
            match cur {
                Instruction::Nop(_) => eip += 1,
                Instruction::Acc(v) => {
                    accumulator += v;
                    eip += 1;
                },
                Instruction::Jmp(v) => eip = eip.wrapping_add(v as usize)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn part1_example() {
        assert_eq!(Ok(5), part1(EXAMPLE.to_string()));
    }

    #[test]
    fn part2_example() {
        assert_eq!(Ok(8), part2(EXAMPLE.to_string()));
    }
}