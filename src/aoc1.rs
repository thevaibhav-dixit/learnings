pub struct Lists {
    left_list: Vec<i32>,
    right_list: Vec<i32>,
}

#[derive(Debug, thiserror::Error)]
pub enum ListsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
}

impl Lists {
    fn init() -> Result<Lists, ListsError> {
        let input =
            std::fs::read_to_string("/home/vaibhav/Desktop/projects/learnings/inputs/aco1.txt")?;

        let (list1, list2): (Vec<_>, Vec<_>) = input
            .lines()
            .map(|line| {
                let nums = line
                    .split_whitespace()
                    .map(str::parse::<i32>)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok::<(i32, i32), ListsError>((nums[0], nums[1]))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .unzip();

        Ok(Lists {
            left_list: list1,
            right_list: list2,
        })
    }

    fn sort(&mut self) {
        self.left_list.sort();
        self.right_list.sort();
    }

    fn solve_pt1(&mut self) -> i32 {
        self.sort();
        let mut res = 0;
        for i in 0..self.left_list.len() {
            res += (self.left_list[i] - self.right_list[i]).abs();
        }
        res
    }

    fn solve_pt_2(&mut self) -> i32 {
        let mut visisted = std::collections::HashSet::new();
        let mut res = 0;

        for i in 0..self.left_list.len() {
            if visisted.contains(&self.left_list[i]) {
                continue;
            }
            let mut occurences = 0;
            for j in 0..self.right_list.len() {
                if self.left_list[i] == self.right_list[j] {
                    occurences += 1;
                }
            }
            visisted.insert(self.left_list[i]);
            res += self.left_list[i] * occurences;
        }
        res
    }
}

pub fn solve() -> anyhow::Result<()> {
    let mut lists = Lists::init()?;
    let _res_pt1 = lists.solve_pt1();
    let _res_pt2 = lists.solve_pt_2();
    Ok(())
}
