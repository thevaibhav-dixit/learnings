use regex::Regex;

use crate::helpers::take_input;

pub struct Instruction(String);

#[derive(Debug, thiserror::Error)]
pub enum InstructionError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
    #[error("No input found")]
    NoInput,
}
impl Instruction {
    fn init() -> Result<Self, InstructionError> {
        let instructions = take_input("aoc3.txt")?
            .lines()
            .next()
            .ok_or(InstructionError::NoInput)?
            .to_string();

        Ok(Instruction(instructions))
    }

    fn solve(&self) -> Result<i32, InstructionError> {
        let re = Regex::new(r"mul\((\w+),(\w+)\)").unwrap();

        let mut ans = 0;
        for captures in re.captures_iter(self.0.as_str()) {
            let x = captures
                .get(1)
                .expect("could not find value X ")
                .as_str()
                .parse::<i32>()?;
            let y = captures
                .get(2)
                .expect("could not find value Y")
                .as_str()
                .parse::<i32>()?;
            ans += x * y;
        }

        Ok(ans)
    }
}

pub fn aoc3_solve() -> anyhow::Result<()> {
    let instruction = Instruction::init()?;
    let _res = instruction.solve()?;

    Ok(())
}
