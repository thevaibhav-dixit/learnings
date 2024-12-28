use crate::helpers::take_input;

struct Reports {
    reports: Vec<Vec<i32>>,
}

#[derive(Debug, thiserror::Error)]
pub enum ReportsError {
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
            let mut valid = true;

            for window in report.windows(3) {
                let (prev, curr, next) = (window[0], window[1], window[2]);

                let is_trend_consistent =
                    (prev < curr && curr < next) || (prev > curr && curr > next);
                let is_difference_small = (curr - prev).abs() <= 3 && (next - curr).abs() <= 3;

                if !(is_trend_consistent && is_difference_small) {
                    valid = false;
                    break;
                }
            }

            if valid {
                ans += 1;
            }
        }

        ans
    }
}

pub fn solve() -> anyhow::Result<()> {
    let reports = Reports::init()?;
    let _ = reports.solve_pt1();

    Ok(())
}
