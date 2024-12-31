use regex::Regex;

use crate::helpers::take_input;

pub fn solve() -> Result<(), std::io::Error> {
    let input = take_input("aoc3.txt")?;

    let re = Regex::new(r"mul\((\w+),(\w+)\)").unwrap();

    let mut ans = 0;
    for line in input.lines() {
        for captures in re.captures_iter(line) {
            let a = captures
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .expect("should parse to i32");
            let b = captures
                .get(2)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .expect("should parse to i32");
            ans += a * b;
        }
    }

    println!("Answer: {}", ans);

    Ok(())
}
