use std::convert::Infallible;

fn run_intcode(mut prog: Vec<u32>) -> Option<u32>
{
    for i in (0..prog.len()).step_by(4) {
        let opcode = *prog.get(i)?;

        if opcode == 99 {
            break;
        }

        let left = *prog.get(i+1)? as usize;
        let right = *prog.get(i+2)? as usize;
        let out = *prog.get(i+3)? as usize;
        if out >= prog.len() {
            return None
        }
        match opcode {
            1 => prog[out] = *prog.get(left)? + *prog.get(right)?,
            2 => prog[out] = *prog.get(left)? * *prog.get(right)?,
            _ => panic!("invalid opcode"),
        }
    }
    prog.get(0).map(|&x| x)
}

pub fn part1(input: String) -> Result<u32, Infallible>
{
    let mut prog = parse_input(input);

    prog[1] = 12;
    prog[2] = 2;

    return Ok(run_intcode(prog).unwrap());
}

pub fn part2(input: String) -> Result<u32, Infallible>
{
    let prog = parse_input(input);

    for x in 0..prog.len() {
        for y in 0..prog.len() {
            let mut prog = prog.clone();
            prog[1] = x as u32;
            prog[2] = y as u32;

            if let Some(i) = run_intcode(prog) {
                if i == 19690720 {
                    return Ok((x * 100 + y) as u32);
                }
            }
        }
    }
    panic!("Solution not found")
}

fn parse_input(input: String) -> Vec<u32>
{
    input
        .split(',')
        .map(|l| l.trim().parse().unwrap())
        .collect()
}
