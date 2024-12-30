use crate::helpers::take_input;

struct Reports {
    reports: Vec<Vec<i32>>,
}

#[derive(Debug, thiserror::Error)]
enum ReportsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
}

impl Reports {
    fn init() -> Result<Reports, ReportsError> {
        let input = take_input("aoc2.txt")?;

        let mut reports = Vec::new();

        for line in input.lines() {
            let report = line
                .split_whitespace()
                .map(str::parse::<i32>)
                .collect::<Result<Vec<_>, _>>()?;

            reports.push(report);
        }

        Ok(Reports { reports })
    }

    fn solve_pt1(&self) -> i32 {
        let mut ans = 0;
        for report in self.reports.iter() {
            if is_monotone(report.iter()) {
                ans += 1;
            }
        }
        ans
    }

    fn solve_pt2(&self) -> i32 {
        let mut ans = 0;

        for report in self.reports.iter() {
            if is_monotone(report.iter()) {
                ans += 1;
                continue;
            }

            for i in 0..report.len() {
                if is_monotone(report[..i].iter().chain(&report[i + 1..])) {
                    ans += 1;
                    break;
                }
            }
        }

        ans
    }
}

fn is_monotone<'a>(numbers: impl Iterator<Item = &'a i32>) -> bool {
    let mut iter = numbers.peekable();

    let mut prev = match iter.next() {
        Some(&val) => val,
        None => return true,
    };

    let mut ordering = None;

    while let Some(&curr) = iter.next() {
        if !(1..=3).contains(&(prev.abs_diff(curr))) {
            return false;
        }

        if let Some(ord) = ordering {
            if prev.cmp(&curr) != ord {
                return false;
            }
        } else {
            ordering = Some(prev.cmp(&curr));
        }

        prev = curr;
    }

    true
}
pub fn solve() -> anyhow::Result<()> {
    let reports = Reports::init()?;
    let _res_pt1 = reports.solve_pt1();
    let _res_pt2 = reports.solve_pt2();

    Ok(())
}
